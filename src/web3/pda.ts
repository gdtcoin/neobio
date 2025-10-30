import * as anchor from '@coral-xyz/anchor';
import { getCpmmPdaAmmConfigId } from '@raydium-io/raydium-sdk-v2/lib/raydium/cpmm';
import { NATIVE_MINT } from '@solana/spl-token';
import { PublicKey } from '@solana/web3.js';

import { BIONEO_TOKEN_MINT, GDTC_TOKEN_MINT, USDT_TOKEN_MINT } from './core';

export function getPoolAddress(
  ammConfig: PublicKey,
  tokenMint0: PublicKey,
  tokenMint1: PublicKey,
  programId: PublicKey,
): [PublicKey, number] {
  const POOL_SEED = Buffer.from(anchor.utils.bytes.utf8.encode('pool'));

  const [address, bump] = PublicKey.findProgramAddressSync(
    [POOL_SEED, ammConfig.toBuffer(), tokenMint0.toBuffer(), tokenMint1.toBuffer()],
    programId,
  );
  return [address, bump];
}

export function getPoolVaultAddress(
  pool: PublicKey,
  vaultTokenMint: PublicKey,
  programId: PublicKey,
): [PublicKey, number] {
  const POOL_VAULT_SEED = Buffer.from(anchor.utils.bytes.utf8.encode('pool_vault'));

  const [address, bump] = PublicKey.findProgramAddressSync(
    [POOL_VAULT_SEED, pool.toBuffer(), vaultTokenMint.toBuffer()],
    programId,
  );
  return [address, bump];
}

export function getOrcleAccountAddress(pool: PublicKey, programId: PublicKey): [PublicKey, number] {
  const ORACLE_SEED = Buffer.from(anchor.utils.bytes.utf8.encode('observation'));
  const [address, bump] = PublicKey.findProgramAddressSync(
    [ORACLE_SEED, pool.toBuffer()],
    programId,
  );
  return [address, bump];
}

const cpSwapProgram = new PublicKey(process.env.CP_SWAP_PROGRAM_ID);
const defaultAmmConfig = new PublicKey('D4FPEruKEHrG5TenZ2mpDGEfu1iUvTiqBxvpU8HLBvC2');
const ammConfig = defaultAmmConfig;
const gdtcAmmConfig = defaultAmmConfig;
let bioneoAmmConfig = getCpmmPdaAmmConfigId(cpSwapProgram, 0).publicKey;

for (let i = 0; i < 50; i++) {
  const _bioneoAmmConfig = getCpmmPdaAmmConfigId(cpSwapProgram, i).publicKey;
  const [bioneoPoolAddress] = getPoolAddress(
    _bioneoAmmConfig,
    BIONEO_TOKEN_MINT,
    GDTC_TOKEN_MINT,
    cpSwapProgram,
  );

  if (
    bioneoPoolAddress.toBase58().toLowerCase() === process.env.BIONEO_POOL_ADDRESS.toLowerCase()
  ) {
    bioneoAmmConfig = _bioneoAmmConfig;
    break;
  }
}

// const [poolAddress] = getPoolAddress(ammConfig, NATIVE_MINT, USDT_TOKEN_MINT, cpSwapProgram);
// const [gdtcPoolAddress] = getPoolAddress(
//   gdtcAmmConfig,
//   NATIVE_MINT,
//   GDTC_TOKEN_MINT,
//   cpSwapProgram,
// );
// const [bioneoPoolAddress] = getPoolAddress(
//   bioneoAmmConfig,
//   BIONEO_TOKEN_MINT,
//   GDTC_TOKEN_MINT,
//   cpSwapProgram,
// );
const poolAddress = new PublicKey(process.env.POOL_ADDRESS);
const gdtcPoolAddress = new PublicKey(process.env.GDTC_POOL_ADDRESS);
const bioneoPoolAddress = new PublicKey(process.env.BIONEO_POOL_ADDRESS);

const [observationAddress] = getOrcleAccountAddress(poolAddress, cpSwapProgram);
const [gdtcObservationAddress] = getOrcleAccountAddress(gdtcPoolAddress, cpSwapProgram);
const [bioneoObservationAddress] = getOrcleAccountAddress(bioneoPoolAddress, cpSwapProgram);

const [inputVault] = getPoolVaultAddress(poolAddress, USDT_TOKEN_MINT, cpSwapProgram);
const [gdtcInputVault] = getPoolVaultAddress(gdtcPoolAddress, NATIVE_MINT, cpSwapProgram);
const [bioneoInputVault] = getPoolVaultAddress(bioneoPoolAddress, GDTC_TOKEN_MINT, cpSwapProgram);

const [outputVault] = getPoolVaultAddress(poolAddress, NATIVE_MINT, cpSwapProgram);
const [gdtcOutputVault] = getPoolVaultAddress(gdtcPoolAddress, GDTC_TOKEN_MINT, cpSwapProgram);
const [bioneoOutputVault] = getPoolVaultAddress(
  bioneoPoolAddress,
  BIONEO_TOKEN_MINT,
  cpSwapProgram,
);

export {
  ammConfig,
  gdtcAmmConfig,
  bioneoAmmConfig,
  poolAddress,
  gdtcPoolAddress,
  bioneoPoolAddress,
  observationAddress,
  gdtcObservationAddress,
  bioneoObservationAddress,
  inputVault,
  gdtcInputVault,
  bioneoInputVault,
  outputVault,
  gdtcOutputVault,
  bioneoOutputVault,
};
