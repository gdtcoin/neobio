import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  getAccount,
  getAssociatedTokenAddressSync,
  NATIVE_MINT,
  TOKEN_PROGRAM_ID,
} from '@solana/spl-token';
import { PublicKey, TransactionMessage, VersionedTransaction } from '@solana/web3.js';
import dayjs from 'dayjs';

import { applyCrowdfunding } from '@/app/api';
import { formatTime } from '@/app/utils/format';

import {
  authority,
  BIONEO_TOKEN_MINT,
  config,
  connection,
  convertBN,
  deriveSwapAccounts,
  GDTC_TOKEN_MINT,
  getMintDecimals,
  IDL,
  movePointLeft,
  movePointRight,
  POOL_ID,
  crowdfundingProgram as program,
  provider,
  RAYDIUM_V4_PROGRAM_ID,
  sendTransaction,
  toBN,
  toBuffer,
  USDT_TOKEN_MINT,
  WSOL_GDTC_POOL_ID,
} from './core';
import { createAltIfNeeded, fetchAltAccount } from './lookupTable';
import {
  bioneoAmmConfig,
  bioneoInputVault,
  bioneoObservationAddress,
  bioneoOutputVault,
  bioneoPoolAddress,
} from './pda';
import { AllCrowdfundingInfo, CrowdfundingInfo, SalePhase, UserPurchase } from './types';

const CROWDFUNDING_PROGRAM_ID = new PublicKey(IDL.crowdfunding.address);

export let crowdfundingInfo: CrowdfundingInfo = null;
export let decimals = 9;

const [CROWDFUNDING_INSTANCE] = PublicKey.findProgramAddressSync(
  [Buffer.from('crowdfunding_instance')],
  CROWDFUNDING_PROGRAM_ID,
);

export const fetchCrowdfundingInfo = async (): Promise<CrowdfundingInfo> => {
  try {
    const bnResponse = await program.account.crowdfundingInfo.fetch(CROWDFUNDING_INSTANCE);

    crowdfundingInfo = convertBN(bnResponse);

    decimals = await getMintDecimals(USDT_TOKEN_MINT);
    return crowdfundingInfo;
  } catch {
    return null;
  }
};

export const fetchSalePhases = async (): Promise<SalePhase[]> => {
  const salePhases = await program.account.salePhase.all();

  return salePhases
    .map((salePhase) => convertBN(salePhase))
    .map((item) => ({
      ...item.account,
      pricePerShare: movePointLeft(item.account.pricePerShare, decimals),
    }))
    .sort((a, b) => b.phaseId - a.phaseId);
};

export const fetchUserPurchases = async (): Promise<UserPurchase[]> => {
  const userPurchases = await program.account.userPurchase.all();

  return userPurchases
    .map((userPurchase) => convertBN(userPurchase))
    .map((item) => ({
      ...item.account,
    }))
    .sort((a, b) => b.purchaseTime - a.purchaseTime)
    .map((item) => {
      const diffDays = dayjs().diff(dayjs(formatTime(item.purchaseTime)), 'day') - 1;
      const totalDays = Number(item.vestingDays);

      let claimableAmount =
        (diffDays / totalDays) * Number(item.tokenAmount) - Number(item.claimedAmount);
      claimableAmount = Math.max(claimableAmount, 0);

      return {
        ...item,
        purchaseTime: formatTime(item.purchaseTime),
        claimedAmount: Number(movePointLeft(item.claimedAmount, decimals)).toFixed(9),
        tokenAmount: Number(movePointLeft(item.tokenAmount, decimals)).toFixed(9),
        claimableAmount: Number(movePointLeft(claimableAmount, decimals)).toFixed(9),
      };
    });
};

export const fetchAllCrowdfundingInfo = async (): Promise<AllCrowdfundingInfo> => {
  const crowdfundingInfo = await fetchCrowdfundingInfo();
  const [salePhases, userPurchases] = await Promise.all([fetchSalePhases(), fetchUserPurchases()]);

  return {
    crowdfundingInfo,
    salePhases,
    userPurchases,
  };
};

export const purchaseShare = async (sharesToBuy, phaseId, soldShares, crowdfundingLink) => {
  const transactionResult = [];

  const [userUsdtTokenAccount] = PublicKey.findProgramAddressSync(
    [authority.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), USDT_TOKEN_MINT.toBuffer()],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  try {
    await getAccount(connection, userUsdtTokenAccount);
  } catch {
    throw new Error('balance_not_enough');
  }

  const [userWsolTokenAccount] = PublicKey.findProgramAddressSync(
    [authority.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), NATIVE_MINT.toBuffer()],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  await getAccount(connection, userWsolTokenAccount).catch(() => {
    transactionResult.push(
      createAssociatedTokenAccountInstruction(
        authority,
        userWsolTokenAccount,
        authority,
        NATIVE_MINT,
      ),
    );
  });

  const [crowdfundingInfoWsolAccount] = PublicKey.findProgramAddressSync(
    [CROWDFUNDING_INSTANCE.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), NATIVE_MINT.toBuffer()],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  const [pdaGdtcAccount] = PublicKey.findProgramAddressSync(
    [authority.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), GDTC_TOKEN_MINT.toBuffer()],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  const [pdaBioAccount] = PublicKey.findProgramAddressSync(
    [authority.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), BIONEO_TOKEN_MINT.toBuffer()],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  const [salePhase] = PublicKey.findProgramAddressSync(
    [Buffer.from('sale_phase'), toBuffer(phaseId)],
    CROWDFUNDING_PROGRAM_ID,
  );

  const [userPurchase] = PublicKey.findProgramAddressSync(
    [Buffer.from('user_purchase'), authority.toBuffer(), toBuffer(phaseId), toBuffer(soldShares)],
    CROWDFUNDING_PROGRAM_ID,
  );

  const [userGdtcTokenAccount] = PublicKey.findProgramAddressSync(
    [authority.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), GDTC_TOKEN_MINT.toBuffer()],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );
  const [userBioneoTokenAccount] = PublicKey.findProgramAddressSync(
    [authority.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), BIONEO_TOKEN_MINT.toBuffer()],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  const [bioneoBlackholeTokenAccount] = PublicKey.findProgramAddressSync(
    [
      new PublicKey(crowdfundingInfo.gdtcBlackholeAddress).toBuffer(),
      TOKEN_PROGRAM_ID.toBuffer(),
      BIONEO_TOKEN_MINT.toBuffer(),
    ],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  const [gdtcBlackholeTokenAccount] = PublicKey.findProgramAddressSync(
    [
      new PublicKey(crowdfundingInfo.gdtcBlackholeAddress).toBuffer(),
      TOKEN_PROGRAM_ID.toBuffer(),
      GDTC_TOKEN_MINT.toBuffer(),
    ],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  await getAccount(connection, pdaBioAccount).catch(() => {
    transactionResult.push(
      createAssociatedTokenAccountInstruction(
        authority,
        pdaBioAccount,
        authority,
        BIONEO_TOKEN_MINT,
      ),
    );
  });

  await getAccount(connection, crowdfundingInfoWsolAccount).catch(() => {
    transactionResult.push(
      createAssociatedTokenAccountInstruction(
        authority,
        crowdfundingInfoWsolAccount,
        CROWDFUNDING_INSTANCE,
        NATIVE_MINT,
      ),
    );
  });

  await getAccount(connection, pdaGdtcAccount).catch(() => {
    transactionResult.push(
      createAssociatedTokenAccountInstruction(
        authority,
        pdaGdtcAccount,
        authority,
        GDTC_TOKEN_MINT,
      ),
    );
  });

  await getAccount(connection, bioneoBlackholeTokenAccount).catch(() => {
    transactionResult.push(
      createAssociatedTokenAccountInstruction(
        authority,
        bioneoBlackholeTokenAccount,
        new PublicKey(crowdfundingInfo.gdtcBlackholeAddress),
        BIONEO_TOKEN_MINT,
      ),
    );
  });

  await getAccount(connection, userBioneoTokenAccount).catch(() => {
    transactionResult.push(
      createAssociatedTokenAccountInstruction(
        authority,
        userBioneoTokenAccount,
        authority,
        BIONEO_TOKEN_MINT,
      ),
    );
  });

  await getAccount(connection, userGdtcTokenAccount).catch(() => {
    transactionResult.push(
      createAssociatedTokenAccountInstruction(
        authority,
        userGdtcTokenAccount,
        authority,
        GDTC_TOKEN_MINT,
      ),
    );
  });

  await getAccount(connection, gdtcBlackholeTokenAccount).catch(() => {
    transactionResult.push(
      createAssociatedTokenAccountInstruction(
        authority,
        gdtcBlackholeTokenAccount,
        new PublicKey(crowdfundingInfo.gdtcBlackholeAddress),
        GDTC_TOKEN_MINT,
      ),
    );
  });

  const swapAccounts = await deriveSwapAccounts(POOL_ID);
  const [ammAuthority] = PublicKey.findProgramAddressSync(
    [Buffer.from('amm authority')],
    RAYDIUM_V4_PROGRAM_ID,
  );

  const usdtToWsolInstruction = await program.methods
    .ammUsdtToWsol(
      toBN(sharesToBuy),
      toBN(phaseId),
      new PublicKey(config.SUPERIOR_ADDRESS),
      toBN(1),
    )
    .accounts({
      user: authority,
      userUsdtTokenAccount,
      userWsolTokenAccount,
      usdtMintAccount: USDT_TOKEN_MINT,
      swapAccounts: {
        ammProgram: RAYDIUM_V4_PROGRAM_ID,
        amm: POOL_ID,
        ammAuthority,
        ammOpenOrders: swapAccounts.openOrders,
        ammCoinVault: swapAccounts.baseVault,
        ammPcVault: swapAccounts.quoteVault,
        marketProgram: swapAccounts.marketProgramId,
        market: swapAccounts.marketId,
        marketBids: swapAccounts.marketBids,
        marketAsks: swapAccounts.marketAsks,
        marketEventQueue: swapAccounts.marketEventQueue,
        marketCoinVault: swapAccounts.marketBaseVault,
        marketPcVault: swapAccounts.marketQuoteVault,
        marketVaultSigner: swapAccounts.marketAuthority,
        userTokenSource: userUsdtTokenAccount,
        userTokenDestination: userWsolTokenAccount,
        userSourceOwner: authority,
      },
      // @ts-ignore
      userPurchase,
      salePhase,
      projectSigner: new PublicKey(crowdfundingInfo.projectSigner),
    })
    .instruction();
  transactionResult.push(usdtToWsolInstruction);

  const wsolGdtcSwapAccounts = await deriveSwapAccounts(WSOL_GDTC_POOL_ID);

  const wsolGdtcInstruction = await program.methods
    .ammWsolGdtc(toBN(sharesToBuy), toBN(phaseId), toBN(1))
    .accounts({
      user: authority,
      userWsolAccount: userWsolTokenAccount,
      userGdtcTokenAccount,
      gdtcBlackholeTokenAccount,
      gdtcMintAccount: GDTC_TOKEN_MINT,
      swapAccounts: {
        ammProgram: RAYDIUM_V4_PROGRAM_ID,
        amm: WSOL_GDTC_POOL_ID,
        ammAuthority,
        ammOpenOrders: wsolGdtcSwapAccounts.openOrders,
        ammCoinVault: wsolGdtcSwapAccounts.baseVault,
        ammPcVault: wsolGdtcSwapAccounts.quoteVault,
        marketProgram: wsolGdtcSwapAccounts.marketProgramId,
        market: wsolGdtcSwapAccounts.marketId,
        marketBids: wsolGdtcSwapAccounts.marketBids,
        marketAsks: wsolGdtcSwapAccounts.marketAsks,
        marketEventQueue: wsolGdtcSwapAccounts.marketEventQueue,
        marketCoinVault: wsolGdtcSwapAccounts.marketBaseVault,
        marketPcVault: wsolGdtcSwapAccounts.marketQuoteVault,
        marketVaultSigner: wsolGdtcSwapAccounts.marketAuthority,
        userTokenSource: userWsolTokenAccount,
        userTokenDestination: userGdtcTokenAccount,
        userSourceOwner: authority,
      },
      // @ts-ignore
      userPurchase,
      projectSigner: new PublicKey(crowdfundingInfo.projectSigner),
    })
    .instruction();
  transactionResult.push(wsolGdtcInstruction);

  const gdtcBioInstruction = await program.methods
    .gdtcToBio(toBN(sharesToBuy), toBN(phaseId))
    .accounts({
      user: authority,
      userGdtcTokenAccount,
      userBioTokenAccount: userBioneoTokenAccount,
      bioBlackholeTokenAccount: bioneoBlackholeTokenAccount,
      gdtcBioAmmConfig: bioneoAmmConfig,
      gdtcBioPoolState: bioneoPoolAddress,
      gdtcBioInputVault: bioneoInputVault,
      gdtcBioOutputVault: bioneoOutputVault,
      gdtcBioInputTokenProgram: TOKEN_PROGRAM_ID,
      gdtcBioOutputTokenProgram: TOKEN_PROGRAM_ID,
      gdtcBioInputTokenMint: GDTC_TOKEN_MINT,
      gdtcBioOutputTokenMint: BIONEO_TOKEN_MINT,
      gdtcBioObservationState: bioneoObservationAddress,
      // @ts-ignore
      userPurchase,
    })
    .instruction();
  transactionResult.push(gdtcBioInstruction);

  const lookupTableAddress = await createAltIfNeeded(authority, [
    USDT_TOKEN_MINT,
    GDTC_TOKEN_MINT,
    BIONEO_TOKEN_MINT,
    ammAuthority,
    userUsdtTokenAccount,
    userWsolTokenAccount,
    userGdtcTokenAccount,
    userBioneoTokenAccount,
    bioneoBlackholeTokenAccount,
    bioneoAmmConfig,
    bioneoPoolAddress,
    bioneoInputVault,
    bioneoOutputVault,
    bioneoObservationAddress,
  ]);
  const lookupTableAccount = await fetchAltAccount(lookupTableAddress);

  const { blockhash, lastValidBlockHeight } = await provider.connection.getLatestBlockhash();
  const msg = new TransactionMessage({
    payerKey: authority,
    recentBlockhash: blockhash,
    instructions: transactionResult,
  }).compileToV0Message([lookupTableAccount]);

  const tx = new VersionedTransaction(msg);

  const b64Unsigned = Buffer.from(tx.serialize()).toString('base64');
  const response = await applyCrowdfunding({
    tx: b64Unsigned,
    expect: {
      program_id: IDL.crowdfunding.address,
      user: authority.toBase58(),
      project_signer: crowdfundingInfo.projectSigner,
      phase_id: phaseId,
      purchase_id: Number(soldShares),
      shares_to_buy: Number(sharesToBuy),
    },
    purchase_link: crowdfundingLink,
  });
  const txPartiallySigned = VersionedTransaction.deserialize(
    Buffer.from(response.data.signature, 'base64'),
  );
  const txFullySigned = await provider.wallet.signTransaction(txPartiallySigned);

  const sig = await connection.sendTransaction(txFullySigned, { skipPreflight: false });
  await connection.confirmTransaction({ signature: sig, blockhash, lastValidBlockHeight });

  return sig;
};

export const createPhase = async (pricePerShare, startTime = dayjs().unix()) => {
  const nextPhaseId = crowdfundingInfo.phaseCount + 1;

  const instruction = await program.methods
    .createPhase(toBN(movePointRight(pricePerShare, decimals)), toBN(startTime), toBN(nextPhaseId))
    .accounts({
      admin: new PublicKey(crowdfundingInfo.admin),
    })
    .instruction();

  return sendTransaction([instruction]);
};

export const claimTokens = async (phaseId, purchaseId, userSuperiorAccount) => {
  const userTokenAccount = getAssociatedTokenAddressSync(BIONEO_TOKEN_MINT, authority);

  const [crowdfundingInstance] = PublicKey.findProgramAddressSync(
    [Buffer.from('crowdfunding_instance')],
    CROWDFUNDING_PROGRAM_ID,
  );

  const [vaultTokenAccount] = PublicKey.findProgramAddressSync(
    [crowdfundingInstance.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), BIONEO_TOKEN_MINT.toBuffer()],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  const instructions = [];

  await getAccount(connection, userTokenAccount).catch(() => {
    const instruction = createAssociatedTokenAccountInstruction(
      authority,
      userTokenAccount,
      authority,
      BIONEO_TOKEN_MINT,
    );

    instructions.push(instruction);
  });

  const [gdtcBlackholeTokenAccount] = PublicKey.findProgramAddressSync(
    [
      new PublicKey(crowdfundingInfo.gdtcBlackholeAddress).toBuffer(),
      TOKEN_PROGRAM_ID.toBuffer(),
      BIONEO_TOKEN_MINT.toBuffer(),
    ],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  await getAccount(connection, gdtcBlackholeTokenAccount).catch(() => {
    instructions.push(
      createAssociatedTokenAccountInstruction(
        authority,
        gdtcBlackholeTokenAccount,
        new PublicKey(crowdfundingInfo.gdtcBlackholeAddress),
        BIONEO_TOKEN_MINT,
      ),
    );
  });

  const [userGlobalPoolTokenAccount] = PublicKey.findProgramAddressSync(
    [
      new PublicKey(crowdfundingInfo.gdtcPoolAddress).toBuffer(),
      TOKEN_PROGRAM_ID.toBuffer(),
      BIONEO_TOKEN_MINT.toBuffer(),
    ],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  await getAccount(connection, userGlobalPoolTokenAccount).catch(() => {
    instructions.push(
      createAssociatedTokenAccountInstruction(
        authority,
        userGlobalPoolTokenAccount,
        new PublicKey(crowdfundingInfo.gdtcPoolAddress),
        BIONEO_TOKEN_MINT,
      ),
    );
  });

  const [userSuperiorTokenAccount] = PublicKey.findProgramAddressSync(
    [
      new PublicKey(userSuperiorAccount).toBuffer(),
      TOKEN_PROGRAM_ID.toBuffer(),
      BIONEO_TOKEN_MINT.toBuffer(),
    ],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  await getAccount(connection, userSuperiorTokenAccount).catch(() => {
    instructions.push(
      createAssociatedTokenAccountInstruction(
        authority,
        userSuperiorTokenAccount,
        new PublicKey(userSuperiorAccount),
        BIONEO_TOKEN_MINT,
      ),
    );
  });

  const [userPurchase] = PublicKey.findProgramAddressSync(
    [Buffer.from('user_purchase'), authority.toBuffer(), toBuffer(phaseId), toBuffer(purchaseId)],
    CROWDFUNDING_PROGRAM_ID,
  );

  const instruction = await program.methods
    .claimTokens(toBN(phaseId), toBN(purchaseId))
    .accounts({
      // @ts-ignore
      userPurchase,
      user: authority,
      vaultTokenAccount,
      userTokenAccount,
      userSuperiorTokenAccount,
      userGlobalPoolTokenAccount,
      gdtcBlackholeTokenAccount,
    })
    .instruction();

  instructions.push(instruction);

  return sendTransaction(instructions);
};
