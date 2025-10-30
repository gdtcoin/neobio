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
  connection,
  convertBN,
  getMintDecimals,
  IDL,
  movePointLeft,
  movePointRight,
  vestingProgram as program,
  sendTransaction,
  toBN,
} from './core';
import { VestingPeriod } from './types';

/**
 * 获取用户的释放计划账户地址
 */
const VESTING_PROGRAM_ID = new PublicKey(IDL.vesting.address);

export const createVestingSchedule = async (
  beneficiary: string | PublicKey,
  mint: string | PublicKey,
  totalAmount: number,
  startTime: number,
  vestingPeriod: VestingPeriod = VestingPeriod.Monthly,
  periodCount: number,
): Promise<string> => {
  beneficiary = new PublicKey(beneficiary);
  mint = new PublicKey(mint);

  const decimals = await getMintDecimals(mint);

  const creatorTokenAccount = getAssociatedTokenAddressSync(mint, authority);

  const [vestingSchedule] = PublicKey.findProgramAddressSync(
    [Buffer.from('vesting'), authority.toBuffer(), beneficiary.toBuffer(), mint.toBuffer()],
    VESTING_PROGRAM_ID,
  );

  const [vaultTokenAccount] = PublicKey.findProgramAddressSync(
    [vestingSchedule.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), mint.toBuffer()],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  const instructions = [];

  let vestingPeriodArg;
  switch (vestingPeriod) {
    case VestingPeriod.Daily:
      vestingPeriodArg = { daily: {} };
      break;
    case VestingPeriod.Monthly:
      vestingPeriodArg = { monthly: {} };
      break;
    case VestingPeriod.Yearly:
      vestingPeriodArg = { yearly: {} };
      break;
    case VestingPeriod.Linear:
      vestingPeriodArg = { linear: {} };
      break;
    default:
      vestingPeriodArg = { monthly: {} };
  }

  await getAccount(connection, vaultTokenAccount).catch(() => {
    const instruction = createAssociatedTokenAccountInstruction(
      authority,
      vaultTokenAccount,
      vestingSchedule,
      mint,
    );
    instructions.push(instruction);
  });

  const instruction = await program.methods
    .createVestingSchedule(
      toBN(movePointRight(totalAmount, decimals)),
      toBN(startTime),
      vestingPeriodArg,
      periodCount,
    )
    .accounts({
      beneficiary,
      mint,
      creatorTokenAccount,
      // @ts-ignore
      vaultTokenAccount,
    })
    .instruction();

  instructions.push(instruction);

  return sendTransaction(instructions);
};

export const claimVestedTokens = async (creator, mint) => {
  mint = new PublicKey(mint);
  creator = new PublicKey(creator);

  const [vestingSchedule] = PublicKey.findProgramAddressSync(
    [Buffer.from('vesting'), creator.toBuffer(), authority.toBuffer(), mint.toBuffer()],
    VESTING_PROGRAM_ID,
  );

  const [vaultTokenAccount] = PublicKey.findProgramAddressSync(
    [vestingSchedule.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), mint.toBuffer()],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  const beneficiaryTokenAccount = getAssociatedTokenAddressSync(mint, authority);

  const instructions = [];

  await getAccount(connection, vaultTokenAccount).catch(() => {
    const instruction = createAssociatedTokenAccountInstruction(
      authority,
      vaultTokenAccount,
      vestingSchedule,
      mint,
    );
    instructions.push(instruction);
  });

  await getAccount(connection, beneficiaryTokenAccount).catch(() => {
    const instruction = createAssociatedTokenAccountInstruction(
      authority,
      beneficiaryTokenAccount,
      authority,
      mint,
    );
    instructions.push(instruction);
  });

  const instruction = await program.methods
    .claim()
    .accounts({
      // @ts-ignore
      vestingSchedule,
      vaultTokenAccount,
      beneficiaryTokenAccount,
    })
    .instruction();

  instructions.push(instruction);

  return sendTransaction(instructions);
};

export const getClaimableAmount = async (creator, beneficiary, mint): Promise<number> => {
  creator = new PublicKey(creator);
  beneficiary = new PublicKey(beneficiary);
  mint = new PublicKey(mint);

  const [vestingSchedule] = PublicKey.findProgramAddressSync(
    [Buffer.from('vesting'), authority.toBuffer(), beneficiary.toBuffer(), mint.toBuffer()],
    VESTING_PROGRAM_ID,
  );

  try {
    const claimableAmount = await program.methods
      .getClaimableAmount()
      .accounts({
        vestingSchedule,
      })
      .view();
    return Number(movePointLeft(convertBN(claimableAmount), 9));
  } catch (error) {
    console.error('获取可提取金额失败:', error);
    return 0;
  }
};

export const fetchVestingInfo = async () => {
  try {
    const bnResponse = await program.account.vestingSchedule.all();

    const vestingInfoList = [];

    for (const item of bnResponse) {
      const vestingInfo = convertBN(item.account);
      const decimals = await getMintDecimals(new PublicKey(vestingInfo.mint));
      vestingInfo.totalAmount = movePointLeft(vestingInfo.totalAmount, decimals);
      vestingInfo.claimedAmount = movePointLeft(vestingInfo.claimedAmount, decimals);
      vestingInfoList.push(vestingInfo);
    }

    return vestingInfoList;
  } catch {
    return null;
  }
};

/**
 * 查询从vaultTokenAccount到beneficiaryTokenAccount的转账记录
 * @param vaultTokenAccount vault代币账户地址
 * @param beneficiaryTokenAccount 受益人代币账户地址
 * @param limit 查询限制数量，默认100
 * @returns 转账记录数组
 */
export const getTransferHistory = async (
  vaultTokenAccount: string | PublicKey,
  beneficiaryTokenAccount: string | PublicKey,
  limit: number = 100,
): Promise<any[]> => {
  try {
    const vaultPubkey = new PublicKey(vaultTokenAccount);
    const beneficiaryPubkey = new PublicKey(beneficiaryTokenAccount);

    // 获取vault账户的交易历史
    const vaultSignatures = await connection.getSignaturesForAddress(vaultPubkey, { limit });

    console.log('🚀 ~ getTransferHistory ~ vaultSignatures:', vaultSignatures);
    // 获取交易详情
    const transferRecords = [];

    for (const sigInfo of vaultSignatures) {
      try {
        console.log(`🔍 处理交易: ${sigInfo.signature}`);
        const transaction = await connection.getTransaction(sigInfo.signature, {
          maxSupportedTransactionVersion: 0,
        });

        if (transaction && transaction.meta) {
          console.log(`  ✅ 成功获取交易数据`);
          // 检查是否包含从vault到beneficiary的代币转账
          const preTokenBalances = transaction.meta.preTokenBalances || [];
          const postTokenBalances = transaction.meta.postTokenBalances || [];
          console.log(
            `  📊 Token余额数据 - pre: ${preTokenBalances.length}条, post: ${postTokenBalances.length}条`,
          );

          // 查找vault和beneficiary的余额变化
          const accountKeys = transaction.transaction.message.getAccountKeys();
          const vaultPreBalance = preTokenBalances.find((balance) => {
            const accountIndex = balance.accountIndex;
            if (accountIndex !== undefined) {
              const key = accountKeys.get(accountIndex);
              return key && key.equals(vaultPubkey);
            }
            return false;
          });

          const vaultPostBalance = postTokenBalances.find((balance) => {
            const accountIndex = balance.accountIndex;
            if (accountIndex !== undefined) {
              const key = accountKeys.get(accountIndex);
              return key && key.equals(vaultPubkey);
            }
            return false;
          });

          const beneficiaryPreBalance = preTokenBalances.find((balance) => {
            const accountIndex = balance.accountIndex;
            if (accountIndex !== undefined) {
              const key = accountKeys.get(accountIndex);
              return key && key.equals(beneficiaryPubkey);
            }
            return false;
          });

          const beneficiaryPostBalance = postTokenBalances.find((balance) => {
            const accountIndex = balance.accountIndex;
            if (accountIndex !== undefined) {
              const key = accountKeys.get(accountIndex);
              return key && key.equals(beneficiaryPubkey);
            }
            return false;
          });

          // 调试日志：输出余额查找结果
          console.log(`📊 交易 ${sigInfo.signature} 余额检查:`);
          console.log('  vaultPreBalance:', !!vaultPreBalance);
          console.log('  vaultPostBalance:', !!vaultPostBalance);
          console.log('  beneficiaryPreBalance:', !!beneficiaryPreBalance);
          console.log('  beneficiaryPostBalance:', !!beneficiaryPostBalance);

          // 检查vault余额变化（必须有前后余额）
          if (vaultPreBalance && vaultPostBalance) {
            const vaultBalanceChange =
              Number(vaultPostBalance.uiTokenAmount.uiAmount || 0) -
              Number(vaultPreBalance.uiTokenAmount.uiAmount || 0);

            // 检查beneficiary余额变化（允许从无到有的情况）
            const beneficiaryPreAmount = beneficiaryPreBalance
              ? Number(beneficiaryPreBalance.uiTokenAmount.uiAmount || 0)
              : 0; // 如果没有交易前余额，视为0

            const beneficiaryPostAmount = beneficiaryPostBalance
              ? Number(beneficiaryPostBalance.uiTokenAmount.uiAmount || 0)
              : 0; // 如果没有交易后余额，视为0

            const beneficiaryBalanceChange = beneficiaryPostAmount - beneficiaryPreAmount;

            console.log(
              `  💰 余额变化 - vault: ${vaultBalanceChange}, beneficiary: ${beneficiaryBalanceChange}`,
            );
            console.log(
              `  📝 具体数值 - vault: ${Number(vaultPreBalance.uiTokenAmount.uiAmount || 0)} → ${Number(vaultPostBalance.uiTokenAmount.uiAmount || 0)}`,
            );
            console.log(
              `  📝 具体数值 - beneficiary: ${beneficiaryPreAmount} → ${beneficiaryPostAmount}`,
            );

            // 如果vault余额减少且beneficiary余额增加，说明发生了转账
            if (vaultBalanceChange < 0 && beneficiaryBalanceChange > 0) {
              console.log(`  ✅ 符合条件，添加到结果中`);
              transferRecords.push({
                signature: sigInfo.signature,
                blockTime: sigInfo.blockTime,
                slot: sigInfo.slot,
                amount: Math.abs(beneficiaryBalanceChange),
                mint: vaultPostBalance.mint,
                from: vaultPubkey.toBase58(),
                to: beneficiaryPubkey.toBase58(),
                status: transaction.meta.err ? 'failed' : 'success',
              });
            } else {
              console.log(
                `  ❌ 不符合条件：vault变化${vaultBalanceChange >= 0 ? '>=0' : '<0'}, beneficiary变化${beneficiaryBalanceChange <= 0 ? '<=0' : '>0'}`,
              );
            }
          } else {
            console.log(
              `  ❌ 缺少vault余额数据，跳过此交易（vault前余额：${!!vaultPreBalance}，vault后余额：${!!vaultPostBalance}）`,
            );
          }
        } else {
          console.log(`  ❌ 未能获取交易数据或meta信息，跳过`);
        }
      } catch (error) {
        console.warn(`❌ 获取交易详情失败: ${sigInfo.signature}`, error);
        continue;
      }
    }

    // 按时间排序，最新的在前
    return transferRecords.sort((a, b) => (b.blockTime || 0) - (a.blockTime || 0));
  } catch (error) {
    console.error('获取转账记录失败:', error);
    return [];
  }
};

/**
 * 查询特定vesting计划的转账记录
 * @param creator 创建者地址
 * @param beneficiary 受益人地址
 * @param mint 代币mint地址
 * @param limit 查询限制数量
 * @returns 转账记录数组
 */
export const getVestingTransferHistory = async (
  creator: string | PublicKey,
  beneficiary: string | PublicKey,
  mint: string | PublicKey,
  limit: number = 10,
): Promise<any[]> => {
  try {
    creator = new PublicKey(creator);
    beneficiary = new PublicKey(beneficiary);
    mint = new PublicKey(mint);

    const [vestingSchedule] = PublicKey.findProgramAddressSync(
      [Buffer.from('vesting'), creator.toBuffer(), beneficiary.toBuffer(), mint.toBuffer()],
      VESTING_PROGRAM_ID,
    );

    // 计算vault token account地址
    const [vaultTokenAccount] = PublicKey.findProgramAddressSync(
      [vestingSchedule.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), mint.toBuffer()],
      ASSOCIATED_TOKEN_PROGRAM_ID,
    );

    // 计算受益人token account地址
    const beneficiaryTokenAccount = getAssociatedTokenAddressSync(mint, beneficiary);

    // 获取转账记录
    return await getTransferHistory(vaultTokenAccount, beneficiaryTokenAccount, limit);
  } catch (error) {
    console.error('获取vesting转账记录失败:', error);
    return [];
  }
};
