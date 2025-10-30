import { AddressLookupTableProgram, PublicKey } from '@solana/web3.js';

import {
  authority,
  BIONEO_TOKEN_MINT,
  connection,
  GDTC_TOKEN_MINT,
  sendTransaction,
  USDT_TOKEN_MINT,
} from './core';
import { GDTC_POOL_ID_DATA, POOL_ID_DATA } from './poolData';

export const DEFAULT_ADDRS_FOR_ALT = [
  USDT_TOKEN_MINT,
  GDTC_TOKEN_MINT,
  BIONEO_TOKEN_MINT,

  POOL_ID_DATA.id,
  POOL_ID_DATA.authority,
  POOL_ID_DATA.openOrders,
  POOL_ID_DATA.baseVault,
  POOL_ID_DATA.quoteVault,
  POOL_ID_DATA.marketProgramId,
  POOL_ID_DATA.marketId,
  POOL_ID_DATA.marketBids,
  POOL_ID_DATA.marketAsks,
  POOL_ID_DATA.marketEventQueue,
  POOL_ID_DATA.marketBaseVault,
  POOL_ID_DATA.marketQuoteVault,
  POOL_ID_DATA.marketAuthority,

  GDTC_POOL_ID_DATA.id,
  GDTC_POOL_ID_DATA.openOrders,
  GDTC_POOL_ID_DATA.baseVault,
  GDTC_POOL_ID_DATA.quoteVault,
  GDTC_POOL_ID_DATA.marketId,
  GDTC_POOL_ID_DATA.marketBids,
  GDTC_POOL_ID_DATA.marketAsks,
  GDTC_POOL_ID_DATA.marketEventQueue,
  GDTC_POOL_ID_DATA.marketBaseVault,
  GDTC_POOL_ID_DATA.marketQuoteVault,
  GDTC_POOL_ID_DATA.marketAuthority,
];

export function saveAltAddress(authority: PublicKey, lookupTableAddress: PublicKey) {
  localStorage.setItem(`alt_v2_${authority.toBase58()}`, lookupTableAddress.toBase58());
}

export function loadAltAddress(authority: PublicKey): PublicKey | null {
  const addr = localStorage.getItem(`alt_v2_${authority.toBase58()}`);

  return addr ? new PublicKey(addr) : null;
}

export function clearAltAddress() {
  localStorage.removeItem(`alt_v2_${authority.toBase58()}`);
}

export async function extendAlt(
  lookupTable: PublicKey,
  authority: PublicKey,
  addresses: PublicKey[],
) {
  const extendIx = AddressLookupTableProgram.extendLookupTable({
    payer: authority,
    authority,
    lookupTable,
    addresses,
  });

  await sendTransaction([extendIx]);
}

export async function fetchAltAccount(lookupTable: PublicKey) {
  const res = await connection.getAddressLookupTable(lookupTable);
  if (!res.value || res.value?.state?.addresses?.length === 0) {
    clearAltAddress();
    throw new Error('ALT not found or not activated yet');
  }

  return res.value;
}

export async function createAltIfNeeded(authority: PublicKey, addresses: PublicKey[] = []) {
  const existing = loadAltAddress(authority);
  if (existing) {
    return existing;
  }

  const recentSlot = await connection.getSlot('finalized');
  const [createIx, lookupTableAddress] = AddressLookupTableProgram.createLookupTable({
    authority,
    payer: authority,
    recentSlot,
  });

  const extendIx = AddressLookupTableProgram.extendLookupTable({
    payer: authority,
    authority,
    lookupTable: lookupTableAddress,
    addresses: [...DEFAULT_ADDRS_FOR_ALT],
  });

  await sendTransaction([createIx, extendIx]);

  if (addresses.length > 0) {
    await extendAlt(lookupTableAddress, authority, [...addresses]);
  }

  saveAltAddress(authority, lookupTableAddress);

  return lookupTableAddress;
}
