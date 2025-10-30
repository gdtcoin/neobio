import * as anchor from '@coral-xyz/anchor';
import { LIQUIDITY_STATE_LAYOUT_V4, MARKET_STATE_LAYOUT_V3 } from '@raydium-io/raydium-sdk';
import { getAssociatedTokenAddressSync, getMint } from '@solana/spl-token';
import { PhantomWalletAdapter } from '@solana/wallet-adapter-phantom';
import {
  ComputeBudgetProgram,
  Connection,
  PublicKey,
  TransactionMessage,
  VersionedTransaction,
} from '@solana/web3.js';
import BN from 'bn.js';
import Decimal from 'decimal.js';

import type { Crowdfunding, NftMiningProgram, StakeProgram, VestingProject } from './idl';
import { IDL } from './idl';
import { GDTC_POOL_ID_DATA, POOL_ID_DATA } from './poolData';

Decimal.set({ toExpNeg: -100, toExpPos: 100 });

export { IDL };
export const config = {
  RPC_URL: process.env.RPC_URL,
  SUPERIOR_ADDRESS: 'EaeUAHS4prbWNezkcCgs1hzHGTyHGKKXL9XzhXfNnPYQ',
  LP_TOKEN_MINT: process.env.LP_TOKEN_MINT,
  BIONEO_TOKEN_MINT: process.env.BIONEO_TOKEN_MINT,
  USDT_TOKEN_MINT: process.env.USDT_TOKEN_MINT,
  GDTC_TOKEN_MINT: process.env.GDTC_TOKEN_MINT,
  BLACK_HOLE_ADDRESS: process.env.BLACK_HOLE_ADDRESS,
  USDT_DECIMALS: process.env.USDT_DECIMALS,
};

export const USDT_TOKEN_MINT = new PublicKey(config.USDT_TOKEN_MINT);
export const BIONEO_TOKEN_MINT = new PublicKey(config.BIONEO_TOKEN_MINT);
export const GDTC_TOKEN_MINT = new PublicKey(config.GDTC_TOKEN_MINT);

export const setSuperiorAddressConfig = (walletAddress) => {
  config.SUPERIOR_ADDRESS = walletAddress;
};

export const connection = new Connection(config.RPC_URL, 'confirmed');
export let authority: PublicKey;
export let provider: anchor.AnchorProvider;
export let stakingProgram: anchor.Program<StakeProgram>;
export let crowdfundingProgram: anchor.Program<Crowdfunding>;
export let vestingProgram: anchor.Program<VestingProject>;
export let nftMiningProgram: anchor.Program<NftMiningProgram>;
export let wallet: PhantomWalletAdapter;

export const movePointRight = (value, point) => {
  return Number(new Decimal(value).times(new Decimal(10).pow(point)).toFixed(9));
};

export const movePointLeft = (value, point) => {
  return Number(new Decimal(value).div(new Decimal(10).pow(point)).toFixed(9));
};

export const convertBN = (obj) => {
  if (obj && obj.toBase58) {
    return obj.toBase58();
  }

  if (BN.isBN(obj)) {
    return obj.toString();
  }

  if (Array.isArray(obj)) {
    return obj.map(convertBN);
  }

  if (typeof obj === 'object' && obj !== null) {
    return Object.fromEntries(Object.entries(obj).map(([key, value]) => [key, convertBN(value)]));
  }

  return obj;
};

export const toBN = (value) => {
  return new anchor.BN(value);
};

export const toBuffer = (value) => {
  const buffer = Buffer.alloc(8);
  buffer.writeBigUInt64LE(BigInt(value));

  return buffer;
};

export const toBufferLE = (value) => {
  return toBN(value).toArrayLike(Buffer, 'le', 4);
};

export const connectWallet = async () => {
  try {
    wallet = new PhantomWalletAdapter();
    await wallet.connect();

    authority = wallet.publicKey;
    provider = new anchor.AnchorProvider(connection, wallet, {
      preflightCommitment: 'confirmed',
    });
    stakingProgram = new anchor.Program(IDL.staking, provider);
    crowdfundingProgram = new anchor.Program(IDL.crowdfunding, provider);
    vestingProgram = new anchor.Program(IDL.vesting, provider);
    nftMiningProgram = new anchor.Program(IDL.nft, provider);
    return authority.toBase58();
  } catch (error) {
    console.log('🚀 ~ connectWallet ~ error:', error);
    throw error;
  }
};

export const sendTransaction = async (instructions, addressLookupTableAccounts?) => {
  const computeUnitLimitIx = ComputeBudgetProgram.setComputeUnitLimit({
    units: 500000,
  });

  const { blockhash } = await provider.connection.getLatestBlockhash();

  const message = new TransactionMessage({
    payerKey: authority,
    recentBlockhash: blockhash,
    instructions: [computeUnitLimitIx, ...instructions],
  }).compileToV0Message(addressLookupTableAccounts);

  const transaction = new VersionedTransaction(message);

  try {
    const signature = await provider.sendAndConfirm(transaction);
    console.log('🚀 ~ sendTransaction ~ signature:', signature);

    return signature;
  } catch (error) {
    if (error.message && error.message.includes('already been processed')) {
      console.error('🚀 ~ sendTransaction ~ error:', error);

      return 'transaction_already_processed';
    }

    throw error;
  }
};

export const getMintDecimals = async (mintPk: PublicKey) => {
  const mintInfo = await getMint(connection, mintPk);

  return mintInfo.decimals;
};

export const getTokenAccountBalance = async (tokenMint, walletAddress = authority) => {
  try {
    const tokenAccount = getAssociatedTokenAddressSync(new PublicKey(tokenMint), walletAddress);

    const { value } = await connection.getTokenAccountBalance(tokenAccount);

    return Number(movePointLeft(value.amount, value.decimals));
  } catch (error) {
    console.log('🚀 ~ getTokenAccountBalance ~ error:', error);
    return 0;
  }
};

export const getLpBalance = () => getTokenAccountBalance(config.LP_TOKEN_MINT);
export const getBioneoBalance = () => getTokenAccountBalance(config.BIONEO_TOKEN_MINT);
export const getBlackHoleBioneoBalance = () =>
  getTokenAccountBalance(config.BIONEO_TOKEN_MINT, new PublicKey(config.BLACK_HOLE_ADDRESS));
export const getBlackHoleGdtcBalance = () =>
  getTokenAccountBalance(config.GDTC_TOKEN_MINT, new PublicKey(config.BLACK_HOLE_ADDRESS));

export function deriveSerumVaultSigner(
  marketId: PublicKey,
  marketProgramId: PublicKey,
  vaultSignerNonce: BN,
): PublicKey {
  const nonceLE8 = Buffer.alloc(8);
  nonceLE8.writeBigUInt64LE(BigInt(vaultSignerNonce.toString()));
  return PublicKey.createProgramAddressSync([marketId.toBuffer(), nonceLE8], marketProgramId);
}

export const POOL_ID = new PublicKey(process.env.POOL_ADDRESS);
export const RAYDIUM_V4_PROGRAM_ID = new PublicKey(process.env.RAYDIUM_V4_PROGRAM_ID);
export const WSOL_GDTC_POOL_ID = new PublicKey(process.env.GDTC_POOL_ADDRESS);

export async function deriveSwapAccounts(poolId: PublicKey) {
  if (poolId.toBase58().toLowerCase() === POOL_ID_DATA.id.toBase58().toLowerCase()) {
    return POOL_ID_DATA;
  }
  if (poolId.toBase58().toLowerCase() === GDTC_POOL_ID_DATA.id.toBase58().toLowerCase()) {
    return GDTC_POOL_ID_DATA;
  }

  // 1. 读池账户
  const poolInfo = await connection.getAccountInfo(poolId);
  if (!poolInfo) throw new Error('pool not found');
  if (!poolInfo.owner.equals(RAYDIUM_V4_PROGRAM_ID)) {
    throw new Error('pool not owned by expected Raydium V4 program');
  }

  // 2. decode v4池
  const poolState = LIQUIDITY_STATE_LAYOUT_V4.decode(poolInfo.data);

  // helper: safe PublicKey wrapper
  const pk = (x: any) => new PublicKey(x);

  // 3. 拿 marketId / marketProgramId
  const marketPk = pk(poolState.marketId);
  const marketProgramPk = pk(poolState.marketProgramId);

  // 4. 读市场账户 (Serum/OpenBook Market)
  const marketInfo = await connection.getAccountInfo(marketPk);
  if (!marketInfo) throw new Error('market not found');

  const marketState = MARKET_STATE_LAYOUT_V3.decode(marketInfo.data);

  const bidsPk = pk(marketState.bids);
  const asksPk = pk(marketState.asks);
  const eventQueuePk = pk(marketState.eventQueue);
  const baseVaultPk = pk(marketState.baseVault);
  const quoteVaultPk = pk(marketState.quoteVault);

  const serumVaultSigner = deriveSerumVaultSigner(
    marketPk,
    pk(poolState.marketProgramId),
    marketState.vaultSignerNonce instanceof BN
      ? marketState.vaultSignerNonce
      : new BN(marketState.vaultSignerNonce),
  );

  // 6. 现在我们可以组出完整的 accounts 对象（基本跟官方示例一模一样）
  const accounts = {
    // pool side
    id: poolId,
    authority: pk(poolState.owner), // ammAuthority
    openOrders: pk(poolState.openOrders),
    targetOrders: pk(poolState.targetOrders),
    baseVault: pk(poolState.baseVault),
    quoteVault: pk(poolState.quoteVault),

    // market side
    marketProgramId: marketProgramPk,
    marketId: marketPk,
    marketBids: bidsPk,
    marketAsks: asksPk,
    marketEventQueue: eventQueuePk,
    marketBaseVault: baseVaultPk,
    marketQuoteVault: quoteVaultPk,
    marketAuthority: serumVaultSigner, // serumVaultSigner

    // plus metadata mints etc if you need them
    baseMint: pk(poolState.baseMint),
    quoteMint: pk(poolState.quoteMint),
    lpMint: pk(poolState.lpMint),
  };

  return accounts;
}
