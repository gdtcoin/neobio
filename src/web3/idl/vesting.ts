/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/vesting_project.json`.
 */
export type VestingProject = {
  address: '9YLCGJaks5rLCpthMP8LoqW72yzkuxBGVxRzGK3ACrTc';
  metadata: {
    name: 'vestingProject';
    version: '0.1.0';
    spec: '0.1.0';
    description: 'Created with Anchor';
  };
  instructions: [
    {
      name: 'cancelVesting';
      docs: ['取消释放计划（仅创建者可用）'];
      discriminator: [171, 166, 241, 72, 155, 48, 30, 253];
      accounts: [
        {
          name: 'creator';
          writable: true;
          signer: true;
        },
        {
          name: 'vestingSchedule';
          docs: ['释放计划账户'];
          writable: true;
          pda: {
            seeds: [
              {
                kind: 'const';
                value: [118, 101, 115, 116, 105, 110, 103];
              },
              {
                kind: 'account';
                path: 'creator';
              },
              {
                kind: 'account';
                path: 'vesting_schedule.beneficiary';
                account: 'vestingSchedule';
              },
              {
                kind: 'account';
                path: 'vesting_schedule.mint';
                account: 'vestingSchedule';
              },
            ];
          };
        },
        {
          name: 'vaultTokenAccount';
          docs: ['托管代币账户'];
          writable: true;
          pda: {
            seeds: [
              {
                kind: 'const';
                value: [118, 97, 117, 108, 116];
              },
              {
                kind: 'account';
                path: 'vestingSchedule';
              },
            ];
          };
        },
        {
          name: 'creatorTokenAccount';
          docs: ['创建者的代币账户（用于退还剩余代币）'];
          writable: true;
        },
        {
          name: 'tokenProgram';
          address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
        },
      ];
      args: [];
    },
    {
      name: 'claim';
      docs: ['提取已释放的代币'];
      discriminator: [62, 198, 214, 193, 213, 159, 108, 210];
      accounts: [
        {
          name: 'beneficiary';
          writable: true;
          signer: true;
        },
        {
          name: 'vestingSchedule';
          docs: ['释放计划账户'];
          writable: true;
          pda: {
            seeds: [
              {
                kind: 'const';
                value: [118, 101, 115, 116, 105, 110, 103];
              },
              {
                kind: 'account';
                path: 'vesting_schedule.creator';
                account: 'vestingSchedule';
              },
              {
                kind: 'account';
                path: 'vesting_schedule.beneficiary';
                account: 'vestingSchedule';
              },
              {
                kind: 'account';
                path: 'vesting_schedule.mint';
                account: 'vestingSchedule';
              },
            ];
          };
        },
        {
          name: 'vaultTokenAccount';
          docs: ['托管代币账户'];
          writable: true;
        },
        {
          name: 'beneficiaryTokenAccount';
          docs: ['受益人的代币账户'];
          writable: true;
        },
        {
          name: 'tokenProgram';
          address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
        },
      ];
      args: [];
    },
    {
      name: 'createVestingSchedule';
      docs: ['创建释放计划'];
      discriminator: [195, 30, 184, 253, 77, 154, 187, 66];
      accounts: [
        {
          name: 'creator';
          writable: true;
          signer: true;
        },
        {
          name: 'beneficiary';
        },
        {
          name: 'mint';
          docs: ['代币mint'];
        },
        {
          name: 'creatorTokenAccount';
          docs: ['创建者的代币账户'];
          writable: true;
        },
        {
          name: 'vaultTokenAccount';
          docs: ['托管代币账户（PDA）'];
          writable: true;
          pda: {
            seeds: [
              {
                kind: 'const';
                value: [118, 97, 117, 108, 116];
              },
              {
                kind: 'account';
                path: 'vestingSchedule';
              },
            ];
          };
        },
        {
          name: 'vestingSchedule';
          docs: ['释放计划账户（PDA）'];
          writable: true;
          pda: {
            seeds: [
              {
                kind: 'const';
                value: [118, 101, 115, 116, 105, 110, 103];
              },
              {
                kind: 'account';
                path: 'creator';
              },
              {
                kind: 'account';
                path: 'beneficiary';
              },
              {
                kind: 'account';
                path: 'mint';
              },
            ];
          };
        },
        {
          name: 'tokenProgram';
          address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
        },
        {
          name: 'systemProgram';
          address: '11111111111111111111111111111111';
        },
        {
          name: 'rent';
          address: 'SysvarRent111111111111111111111111111111111';
        },
      ];
      args: [
        {
          name: 'totalAmount';
          type: 'u64';
        },
        {
          name: 'startTime';
          type: 'i64';
        },
        {
          name: 'vestingPeriod';
          type: {
            defined: {
              name: 'vestingPeriod';
            };
          };
        },
        {
          name: 'periodCount';
          type: 'u32';
        },
      ];
    },
    {
      name: 'getClaimableAmount';
      docs: ['查询可提取金额'];
      discriminator: [216, 219, 61, 62, 140, 223, 122, 15];
      accounts: [
        {
          name: 'vestingSchedule';
          docs: ['释放计划账户'];
        },
      ];
      args: [];
      returns: 'u64';
    },
    {
      name: 'getVestingInfo';
      docs: ['查询释放计划详细信息'];
      discriminator: [194, 159, 240, 192, 75, 100, 162, 30];
      accounts: [
        {
          name: 'vestingSchedule';
          docs: ['释放计划账户'];
        },
      ];
      args: [];
      returns: {
        defined: {
          name: 'vestingInfo';
        };
      };
    },
  ];
  accounts: [
    {
      name: 'vestingSchedule';
      discriminator: [130, 200, 173, 148, 39, 75, 243, 147];
    },
  ];
  errors: [
    {
      code: 6000;
      name: 'invalidAmount';
      msg: 'Invalid amount: must be greater than 0';
    },
    {
      code: 6001;
      name: 'invalidDuration';
      msg: 'Invalid duration: must be greater than 0';
    },
    {
      code: 6002;
      name: 'invalidStartTime';
      msg: 'Invalid start time: must be in the future';
    },
    {
      code: 6003;
      name: 'nothingToClaim';
      msg: 'Nothing to claim';
    },
    {
      code: 6004;
      name: 'mathOverflow';
      msg: 'Math overflow';
    },
    {
      code: 6005;
      name: 'unauthorized';
      msg: 'Unauthorized: only beneficiary can claim';
    },
    {
      code: 6006;
      name: 'unauthorized2';
      msg: 'Unauthorized: only beneficiary can claim2';
    },
    {
      code: 6007;
      name: 'vestingScheduleNotFound';
      msg: 'Vesting schedule not found';
    },
    {
      code: 6008;
      name: 'invalidTokenMint';
      msg: 'Invalid token mint';
    },
    {
      code: 6009;
      name: 'insufficientBalance';
      msg: 'Insufficient balance';
    },
    {
      code: 6010;
      name: 'invalidPeriodCount';
      msg: 'Invalid period count: must be greater than 0';
    },
    {
      code: 6011;
      name: 'invalidVestingPeriod';
      msg: 'Invalid Vesting Period';
    },
  ];
  types: [
    {
      name: 'vestingInfo';
      docs: ['释放计划信息结构体'];
      type: {
        kind: 'struct';
        fields: [
          {
            name: 'creator';
            type: 'pubkey';
          },
          {
            name: 'beneficiary';
            type: 'pubkey';
          },
          {
            name: 'mint';
            type: 'pubkey';
          },
          {
            name: 'totalAmount';
            type: 'u64';
          },
          {
            name: 'claimedAmount';
            type: 'u64';
          },
          {
            name: 'claimableAmount';
            type: 'u64';
          },
          {
            name: 'lockedAmount';
            type: 'u64';
          },
          {
            name: 'startTime';
            type: 'i64';
          },
          {
            name: 'vestingPeriod';
            type: {
              defined: {
                name: 'vestingPeriod';
              };
            };
          },
          {
            name: 'periodCount';
            type: 'u32';
          },
          {
            name: 'amountPerPeriod';
            type: 'u64';
          },
          {
            name: 'completedPeriods';
            type: 'u32';
          },
          {
            name: 'createdAt';
            type: 'i64';
          },
          {
            name: 'progress';
            type: 'u8';
          },
          {
            name: 'isFullyVested';
            type: 'bool';
          },
          {
            name: 'nextReleaseTime';
            type: {
              option: 'i64';
            };
          },
        ];
      };
    },
    {
      name: 'vestingPeriod';
      docs: ['释放周期类型'];
      type: {
        kind: 'enum';
        variants: [
          {
            name: 'daily';
          },
          {
            name: 'monthly';
          },
          {
            name: 'yearly';
          },
          {
            name: 'linear';
          },
        ];
      };
    },
    {
      name: 'vestingSchedule';
      docs: ['线性释放计划账户'];
      type: {
        kind: 'struct';
        fields: [
          {
            name: 'creator';
            docs: ['创建者'];
            type: 'pubkey';
          },
          {
            name: 'beneficiary';
            docs: ['受益人'];
            type: 'pubkey';
          },
          {
            name: 'mint';
            docs: ['代币mint'];
            type: 'pubkey';
          },
          {
            name: 'totalAmount';
            docs: ['总金额'];
            type: 'u64';
          },
          {
            name: 'claimedAmount';
            docs: ['已提取金额'];
            type: 'u64';
          },
          {
            name: 'startTime';
            docs: ['开始时间（Unix时间戳）'];
            type: 'i64';
          },
          {
            name: 'vestingPeriod';
            docs: ['释放周期类型'];
            type: {
              defined: {
                name: 'vestingPeriod';
              };
            };
          },
          {
            name: 'periodCount';
            docs: ['释放周期数量（例如：12个月、365天、2年等）'];
            type: 'u32';
          },
          {
            name: 'amountPerPeriod';
            docs: ['每个周期释放的金额'];
            type: 'u64';
          },
          {
            name: 'createdAt';
            docs: ['创建时间'];
            type: 'i64';
          },
        ];
      };
    },
  ];
};
