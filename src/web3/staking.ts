import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  getAccount,
  getAssociatedTokenAddressSync,
  TOKEN_PROGRAM_ID,
} from '@solana/spl-token';
import { PublicKey } from '@solana/web3.js';

import {
  authority,
  BIONEO_TOKEN_MINT,
  config,
  connection,
  convertBN,
  IDL,
  movePointRight,
  stakingProgram as program,
  sendTransaction,
  toBN,
} from './core';
import { StakingInstance, User } from './types';

const STAKING_PROGRAM_ID = new PublicKey(IDL.staking.address);

let stakingInfo: StakingInstance;

const [STAKING_INSTANCE] = PublicKey.findProgramAddressSync(
  [Buffer.from('staking_instance')],
  STAKING_PROGRAM_ID,
);

export const fetchStakingPools = async (): Promise<StakingInstance> => {
  try {
    const bnResponse = await program.account.stakingInstance.fetch(STAKING_INSTANCE);

    stakingInfo = convertBN(bnResponse);

    return stakingInfo;
  } catch (error) {
    console.log('🚀 ~ fetchStakingPools ~ error:', error);
    return {
      pools: [],
    } as StakingInstance;
  }
};

export const fetchUserStakedInfo = async (): Promise<User> => {
  try {
    const [userInstance] = PublicKey.findProgramAddressSync(
      [Buffer.from('user'), authority.toBuffer()],
      STAKING_PROGRAM_ID,
    );

    const bnResponse = await program.account.user.fetch(userInstance);

    return convertBN(bnResponse);
  } catch (error) {
    console.log('🚀 ~ fetchUserStakedInfo ~ error:', error);
    return {
      stakedInfo: [],
    } as User;
  }
};

export const initializeStakingUser = async () => {
  const instruction = await program.methods
    .initializeUser(new PublicKey(config.SUPERIOR_ADDRESS))
    .accounts({
      authority,
    })
    .instruction();

  return sendTransaction([instruction]);
};

export const enterStaking = async (lpAmount, stakeType, index) => {
  const [gdtcLpInAccount] = PublicKey.findProgramAddressSync(
    [
      STAKING_INSTANCE.toBuffer(),
      TOKEN_PROGRAM_ID.toBuffer(),
      new PublicKey(stakingInfo.stakingTokenMint).toBuffer(),
    ],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  const userLpTokenAccount = getAssociatedTokenAddressSync(
    new PublicKey(stakingInfo.stakingTokenMint),
    authority,
  );

  const instructions = [];

  await getAccount(connection, gdtcLpInAccount).catch(() => {
    const instruction = createAssociatedTokenAccountInstruction(
      authority,
      gdtcLpInAccount,
      STAKING_INSTANCE,
      new PublicKey(stakingInfo.stakingTokenMint),
    );

    instructions.push(instruction);
  });

  const instruction = await program.methods
    .enterStaking(toBN(movePointRight(lpAmount, 9)), toBN(stakeType), toBN(index))
    .accounts({
      authority,
      userLpTokenAccount,
      gdtcLpInAccount,
    })
    .instruction();

  instructions.push(instruction);

  return sendTransaction(instructions);
};

export const claimStakingRewards = async (index, userSuperiorAccount) => {
  const [gdtcRewardOutAccount] = PublicKey.findProgramAddressSync(
    [
      STAKING_INSTANCE.toBuffer(),
      TOKEN_PROGRAM_ID.toBuffer(),
      new PublicKey(stakingInfo.rewardTokenMint).toBuffer(),
    ],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  console.log('🚀 ~ claimStakingRewards ~ STAKING_INSTANCE:', STAKING_INSTANCE.toBase58());

  const userGdtcTokenAccount = getAssociatedTokenAddressSync(
    new PublicKey(stakingInfo.rewardTokenMint),
    authority,
  );

  const systemProgram = new PublicKey('11111111111111111111111111111111');

  const [blackHoleBioAccount] = PublicKey.findProgramAddressSync(
    [
      systemProgram.toBuffer(),
      TOKEN_PROGRAM_ID.toBuffer(),
      new PublicKey(stakingInfo.rewardTokenMint).toBuffer(),
    ],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  const instructions = [];

  await getAccount(connection, blackHoleBioAccount).catch(() => {
    const instruction = createAssociatedTokenAccountInstruction(
      authority,
      blackHoleBioAccount,
      systemProgram,
      BIONEO_TOKEN_MINT,
    );
    instructions.push(instruction);
  });

  const [userSuperGdtcTokenAccount] = PublicKey.findProgramAddressSync(
    [
      new PublicKey(userSuperiorAccount).toBuffer(),
      TOKEN_PROGRAM_ID.toBuffer(),
      new PublicKey(stakingInfo.rewardTokenMint).toBuffer(),
    ],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  await getAccount(connection, userSuperGdtcTokenAccount).catch(() => {
    const instruction = createAssociatedTokenAccountInstruction(
      authority,
      userSuperGdtcTokenAccount,
      new PublicKey(userSuperiorAccount),
      BIONEO_TOKEN_MINT,
    );
    instructions.push(instruction);
  });

  const [userGlobalPoolTokenAccount] = PublicKey.findProgramAddressSync(
    [
      new PublicKey(stakingInfo.gdtcPoolAddress).toBuffer(),
      TOKEN_PROGRAM_ID.toBuffer(),
      BIONEO_TOKEN_MINT.toBuffer(),
    ],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );
  console.log(
    '🚀 ~ claimStakingRewards ~ userGlobalPoolTokenAccount:',
    userGlobalPoolTokenAccount.toBase58(),
  );

  await getAccount(connection, userGlobalPoolTokenAccount).catch(() => {
    const instruction = createAssociatedTokenAccountInstruction(
      authority,
      userGlobalPoolTokenAccount,
      new PublicKey(stakingInfo.gdtcPoolAddress),
      BIONEO_TOKEN_MINT,
    );
    instructions.push(instruction);
  });

  const instruction = await program.methods
    .claimRewards(toBN(index))
    .accounts({
      authority,
      userGdtcTokenAccount,
      gdtcRewardOutAccount,
      userSuperGdtcTokenAccount,
      blackHoleBioAccount,
      userGlobalPoolTokenAccount,
      bioMintAccount: BIONEO_TOKEN_MINT,
    })
    .instruction();

  instructions.push(instruction);

  return sendTransaction(instructions);
};

export const cancelStaking = async (index) => {
  const [gdtcLpInAccount] = await PublicKey.findProgramAddressSync(
    [
      STAKING_INSTANCE.toBuffer(),
      TOKEN_PROGRAM_ID.toBuffer(),
      new PublicKey(stakingInfo.stakingTokenMint).toBuffer(),
    ],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  const userLpTokenAccount = getAssociatedTokenAddressSync(
    new PublicKey(stakingInfo.stakingTokenMint),
    authority,
  );

  const instruction = await program.methods
    .cancelStaking(toBN(index))
    .accounts({
      authority,
      userLpTokenAccount,
      gdtcLpInAccount,
    })
    .instruction();

  return sendTransaction([instruction]);
};
