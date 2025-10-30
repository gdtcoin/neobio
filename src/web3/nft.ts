import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  getAccount,
  NATIVE_MINT,
  TOKEN_PROGRAM_ID,
} from '@solana/spl-token';
import { PublicKey, TransactionMessage, VersionedTransaction } from '@solana/web3.js';
import dayjs from 'dayjs';

import { addStakeApi } from '@/app/api';

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
  nftMiningProgram as program,
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

const NFT_PROGRAM_ID = new PublicKey(IDL.nft.address);

let nftMiningSystemAccount: any = null;

export const fetchNftMiningSystem = async (): Promise<any> => {
  const [nftMiningSystem] = PublicKey.findProgramAddressSync(
    [Buffer.from('nft_mining_system')],
    NFT_PROGRAM_ID,
  );

  const response = await program.account.nftMiningSystem.fetch(nftMiningSystem);

  nftMiningSystemAccount = convertBN(response);

  return nftMiningSystemAccount;
};

export const fetchOrderInfo = async (): Promise<any> => {
  const response = await program.account.orderInfo.all();

  return response
    .map((item) => convertBN(item))
    .map((item) => ({
      ...item.account,
      investmentAmount: movePointLeft(item.account.investmentAmount, config.USDT_DECIMALS),
      totalPower: movePointLeft(item.account.totalPower, config.USDT_DECIMALS),
      receivedReward: movePointLeft(item.account.receivedReward, 9),
      stakeStartTime: dayjs(Number(item.account.stakeStartTime) * 1000).format(
        'YYYY-MM-DD HH:mm:ss',
      ),
      orderInfoIndex: Number(item.account.orderInfoIndex),
    }))
    .filter((item) => item.userAddress.toLowerCase() === authority.toBase58().toLowerCase())
    .sort((a, b) => b.orderInfoIndex - a.orderInfoIndex);
};

export const purchaseNft = async (usdtAmount: number) => {
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

  const [userSuperiorUsdtAccount] = PublicKey.findProgramAddressSync(
    [
      new PublicKey(config.SUPERIOR_ADDRESS).toBuffer(),
      TOKEN_PROGRAM_ID.toBuffer(),
      USDT_TOKEN_MINT.toBuffer(),
    ],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  await getAccount(connection, userSuperiorUsdtAccount).catch(() => {
    transactionResult.push(
      createAssociatedTokenAccountInstruction(
        authority,
        userSuperiorUsdtAccount,
        new PublicKey(config.SUPERIOR_ADDRESS),
        USDT_TOKEN_MINT,
      ),
    );
  });

  const [marketPoolAddressUsdtAccount] = PublicKey.findProgramAddressSync(
    [
      new PublicKey(nftMiningSystemAccount.marketPoolAddress).toBuffer(),
      TOKEN_PROGRAM_ID.toBuffer(),
      USDT_TOKEN_MINT.toBuffer(),
    ],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  await getAccount(connection, marketPoolAddressUsdtAccount).catch(() => {
    transactionResult.push(
      createAssociatedTokenAccountInstruction(
        authority,
        marketPoolAddressUsdtAccount,
        new PublicKey(nftMiningSystemAccount.marketPoolAddress),
        USDT_TOKEN_MINT,
      ),
    );
  });

  const [orderInfo] = PublicKey.findProgramAddressSync(
    [Buffer.from('order_info'), toBuffer(Number(nftMiningSystemAccount.orderInfoIndex) + 1)],
    NFT_PROGRAM_ID,
  );

  const [userGdtcTokenAccount] = PublicKey.findProgramAddressSync(
    [authority.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), GDTC_TOKEN_MINT.toBuffer()],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

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

  const [blackHoleGdtcTokenAccount] = PublicKey.findProgramAddressSync(
    [
      new PublicKey(nftMiningSystemAccount.blackHoleAddress).toBuffer(),
      TOKEN_PROGRAM_ID.toBuffer(),
      GDTC_TOKEN_MINT.toBuffer(),
    ],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  await getAccount(connection, blackHoleGdtcTokenAccount).catch(() => {
    transactionResult.push(
      createAssociatedTokenAccountInstruction(
        authority,
        blackHoleGdtcTokenAccount,
        new PublicKey(nftMiningSystemAccount.blackHoleAddress),
        GDTC_TOKEN_MINT,
      ),
    );
  });

  const [userBioneoTokenAccount] = PublicKey.findProgramAddressSync(
    [authority.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), BIONEO_TOKEN_MINT.toBuffer()],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

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

  const [bioneoBlackholeTokenAccount] = PublicKey.findProgramAddressSync(
    [
      new PublicKey(nftMiningSystemAccount.blackHoleAddress).toBuffer(),
      TOKEN_PROGRAM_ID.toBuffer(),
      BIONEO_TOKEN_MINT.toBuffer(),
    ],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  await getAccount(connection, bioneoBlackholeTokenAccount).catch(() => {
    transactionResult.push(
      createAssociatedTokenAccountInstruction(
        authority,
        bioneoBlackholeTokenAccount,
        new PublicKey(nftMiningSystemAccount.blackHoleAddress),
        BIONEO_TOKEN_MINT,
      ),
    );
  });

  const [poolAddressBioMint] = PublicKey.findProgramAddressSync(
    [
      new PublicKey(nftMiningSystemAccount.poolAddress).toBuffer(),
      TOKEN_PROGRAM_ID.toBuffer(),
      BIONEO_TOKEN_MINT.toBuffer(),
    ],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  await getAccount(connection, poolAddressBioMint).catch(() => {
    transactionResult.push(
      createAssociatedTokenAccountInstruction(
        authority,
        poolAddressBioMint,
        new PublicKey(nftMiningSystemAccount.poolAddress),
        BIONEO_TOKEN_MINT,
      ),
    );
  });

  const swapAccounts = await deriveSwapAccounts(POOL_ID);
  const [ammAuthority] = PublicKey.findProgramAddressSync(
    [Buffer.from('amm authority')],
    RAYDIUM_V4_PROGRAM_ID,
  );

  const usdtWsolInstruction = await program.methods
    .ammUsdtWsol(toBN(usdtAmount), toBN(1))
    .accounts({
      user: authority,
      orderInfo,
      userUsdtAccount: userUsdtTokenAccount,
      userWsolAccount: userWsolTokenAccount,
      userSuperiorUsdtAccount,
      marketPoolAddressUsdtAccount,
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
    })
    .instruction();

  transactionResult.push(usdtWsolInstruction);

  const wsolGdtcSwapAccounts = await deriveSwapAccounts(WSOL_GDTC_POOL_ID);

  const wsolGdtcInstruction = await program.methods
    .ammWsolGdtc(toBN(1))
    .accounts({
      user: authority,
      userWsolAccount: userWsolTokenAccount,
      userGdtcAccount: userGdtcTokenAccount,
      blackHoleGdtcAccount: blackHoleGdtcTokenAccount,
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
      orderInfo,
    })
    .instruction();

  transactionResult.push(wsolGdtcInstruction);

  const gdtcToBioInstruction = await program.methods
    .gdtcToBio()
    .accounts({
      user: authority,
      // @ts-ignore
      orderInfo,
      userGdtcAccount: userGdtcTokenAccount,
      userBioAccount: userBioneoTokenAccount,
      blackHoleBioAccount: bioneoBlackholeTokenAccount,
      poolAddressBioMint: poolAddressBioMint,
      gdtcMint: GDTC_TOKEN_MINT,
      bioMint: BIONEO_TOKEN_MINT,
      ammConfig: bioneoAmmConfig,
      poolState: bioneoPoolAddress,
      inputVault: bioneoInputVault,
      outputVault: bioneoOutputVault,
      inputTokenProgram: TOKEN_PROGRAM_ID,
      outputTokenProgram: TOKEN_PROGRAM_ID,
      inputTokenMint: GDTC_TOKEN_MINT,
      outputTokenMint: BIONEO_TOKEN_MINT,
      observationState: bioneoObservationAddress,
    })
    .instruction();

  transactionResult.push(gdtcToBioInstruction);

  const enterStakingInstruction = await program.methods
    .enterStaking()
    .accounts({
      user: authority,
      // @ts-ignore
      orderInfo,
    })
    .instruction();

  transactionResult.push(enterStakingInstruction);

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
  console.log('🚀 ~ lookupTableAccount:', convertBN(lookupTableAccount));

  return sendTransaction(transactionResult, [lookupTableAccount]);
};

export const claimRewards = async (index: number) => {
  const [orderInfo] = PublicKey.findProgramAddressSync(
    [Buffer.from('order_info'), toBuffer(index)],
    NFT_PROGRAM_ID,
  );

  const [userBioneoTokenAccount] = PublicKey.findProgramAddressSync(
    [authority.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), BIONEO_TOKEN_MINT.toBuffer()],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  const [userSuperiorTokenAccount] = PublicKey.findProgramAddressSync(
    [
      new PublicKey(config.SUPERIOR_ADDRESS).toBuffer(),
      TOKEN_PROGRAM_ID.toBuffer(),
      BIONEO_TOKEN_MINT.toBuffer(),
    ],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  const instructions = [];

  await getAccount(connection, userSuperiorTokenAccount).catch(() => {
    instructions.push(
      createAssociatedTokenAccountInstruction(
        authority,
        userSuperiorTokenAccount,
        new PublicKey(config.SUPERIOR_ADDRESS),
        BIONEO_TOKEN_MINT,
      ),
    );
  });

  const [poolAddressBioMint] = PublicKey.findProgramAddressSync(
    [
      new PublicKey(nftMiningSystemAccount.poolAddress).toBuffer(),
      TOKEN_PROGRAM_ID.toBuffer(),
      BIONEO_TOKEN_MINT.toBuffer(),
    ],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  const [nftMiningSystem] = PublicKey.findProgramAddressSync(
    [Buffer.from('nft_mining_system')],
    NFT_PROGRAM_ID,
  );
  console.log('🚀 ~ claimRewards ~ nftMiningSystem:', nftMiningSystem.toBase58());

  const [systemBioAccount] = PublicKey.findProgramAddressSync(
    [nftMiningSystem.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), BIONEO_TOKEN_MINT.toBuffer()],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  const systemProgram = new PublicKey('11111111111111111111111111111111');

  const [blackHoleBioAccount] = PublicKey.findProgramAddressSync(
    [systemProgram.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), BIONEO_TOKEN_MINT.toBuffer()],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  const claimRewardInstruction = await program.methods
    .claimRewards()
    .accounts({
      user: authority,
      // @ts-ignore
      orderInfo,
      userBioAccount: userBioneoTokenAccount,
      userSuperiorTokenAccount,
      poolBioAccount: poolAddressBioMint,
      systemBioAccount,
      bioMint: BIONEO_TOKEN_MINT,
      blackHoleBioAccount,
    })
    .instruction();

  instructions.push(claimRewardInstruction);

  return sendTransaction(instructions);
};

export const addStake = async (
  orderInfoIndex: number,
  reduceAmount: number,
  gdtcAmount: number,
  gdtcPrice: number,
) => {
  console.log('🚀 ~ addStake ~ gdtcAmount:', gdtcAmount);
  const [orderInfo] = PublicKey.findProgramAddressSync(
    [Buffer.from('order_info'), toBuffer(orderInfoIndex)],
    NFT_PROGRAM_ID,
  );

  const [userGdtcTokenAccount] = PublicKey.findProgramAddressSync(
    [authority.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), GDTC_TOKEN_MINT.toBuffer()],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  const [blackHoleGdtcTokenAccount] = PublicKey.findProgramAddressSync(
    [
      new PublicKey(nftMiningSystemAccount.blackHoleAddress).toBuffer(),
      TOKEN_PROGRAM_ID.toBuffer(),
      GDTC_TOKEN_MINT.toBuffer(),
    ],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  const instructions = [];

  const usdtDecimals = await getMintDecimals(USDT_TOKEN_MINT);
  const gdtcDecimals = await getMintDecimals(GDTC_TOKEN_MINT);

  console.log('🚀 ~ addStake ~ gdtcAmount:', gdtcAmount, gdtcDecimals);
  const addStakeInstruction = await program.methods
    .addStake(
      toBN(movePointRight(reduceAmount, usdtDecimals)),
      toBN(movePointRight(gdtcAmount, gdtcDecimals)),
    )
    .accounts({
      user: authority,
      // @ts-ignore
      orderInfo,
      admin: new PublicKey(nftMiningSystemAccount.admin),
      userAddress: authority,
      userGdtcAccount: userGdtcTokenAccount,
      gdtcMint: GDTC_TOKEN_MINT,
      blackHoleGdtcAccount: blackHoleGdtcTokenAccount,
    })
    .instruction();

  instructions.push(addStakeInstruction);

  const { blockhash, lastValidBlockHeight } = await provider.connection.getLatestBlockhash();

  const message = new TransactionMessage({
    payerKey: authority,
    recentBlockhash: blockhash,
    instructions: instructions,
  }).compileToV0Message();

  const transaction = new VersionedTransaction(message);
  const b64Unsigned = Buffer.from(transaction.serialize()).toString('base64');

  const response = await addStakeApi({
    tx: b64Unsigned,
    expect: {
      program_id: IDL.nft.address,
      user: authority.toBase58(),
      project_signer: nftMiningSystemAccount.admin,
      price: gdtcPrice,
      order_info_index: Number(orderInfoIndex),
    },
  });

  const txPartiallySigned = VersionedTransaction.deserialize(
    Buffer.from(response.data.signature, 'base64'),
  );

  const txFullySigned = await provider.wallet.signTransaction(txPartiallySigned);

  const sig = await connection.sendTransaction(txFullySigned, {
    skipPreflight: false,
  });
  await connection.confirmTransaction({ signature: sig, blockhash, lastValidBlockHeight });
  return sig;
};
