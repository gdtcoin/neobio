/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/NftMiningProgram.json`.
 */
export type NftMiningProgram = {
  address: 'Cyc7r9MqrmNECxDhs25cmjWdY6kXxWtUZmezBFCfaJkb';
  metadata: {
    name: 'nftMiningProgram';
    version: '0.1.0';
    spec: '0.1.0';
    description: 'Created with Anchor';
  };
  instructions: [
    {
      name: 'addStake';
      discriminator: [58, 135, 189, 105, 160, 120, 165, 224];
      accounts: [
        {
          name: 'user';
          writable: true;
          signer: true;
        },
        {
          name: 'admin';
          docs: ['管理员签名者'];
          writable: true;
          signer: true;
        },
        {
          name: 'nftMiningSystem';
          docs: ['系统 PDA（已初始化）'];
          writable: true;
          pda: {
            seeds: [
              {
                kind: 'const';
                value: [
                  110,
                  102,
                  116,
                  95,
                  109,
                  105,
                  110,
                  105,
                  110,
                  103,
                  95,
                  115,
                  121,
                  115,
                  116,
                  101,
                  109,
                ];
              },
            ];
          };
        },
        {
          name: 'orderInfo';
          docs: ['用户状态账户'];
          writable: true;
          pda: {
            seeds: [
              {
                kind: 'const';
                value: [111, 114, 100, 101, 114, 95, 105, 110, 102, 111];
              },
              {
                kind: 'account';
                path: 'order_info.order_info_index';
                account: 'orderInfo';
              },
            ];
          };
        },
        {
          name: 'userAddress';
          docs: ['用户地址（用于验证）'];
          signer: true;
        },
        {
          name: 'userGdtcAccount';
          writable: true;
        },
        {
          name: 'blackHoleGdtcAccount';
          writable: true;
        },
        {
          name: 'tokenProgram';
          address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
        },
        {
          name: 'systemProgram';
          docs: ['系统程序'];
          address: '11111111111111111111111111111111';
        },
      ];
      args: [
        {
          name: 'reduceAmount';
          type: 'u64';
        },
        {
          name: 'gdtcAmount';
          type: 'u64';
        },
      ];
    },
    {
      name: 'ammUsdtWsol';
      docs: [
        '使用 Raydium AMM V4 将 USDT 兑换为 WSOL (swap_base_out)',
        '',
        '# Arguments',
        '* `usdt_amount` - USDT 金额 (100/300/500/1000)',
        '* `expected_wsol_amount` - 期望获得的 WSOL 数量（0 表示根据市场价格）',
      ];
      discriminator: [100, 166, 125, 236, 4, 107, 150, 70];
      accounts: [
        {
          name: 'user';
          docs: ['付款人与接收者'];
          writable: true;
          signer: true;
        },
        {
          name: 'nftMiningSystem';
          docs: ['系统 PDA（已初始化，持有 Master 那 1 枚）'];
          writable: true;
          pda: {
            seeds: [
              {
                kind: 'const';
                value: [
                  110,
                  102,
                  116,
                  95,
                  109,
                  105,
                  110,
                  105,
                  110,
                  103,
                  95,
                  115,
                  121,
                  115,
                  116,
                  101,
                  109,
                ];
              },
            ];
          };
        },
        {
          name: 'orderInfo';
          docs: ['用户订单信息'];
          writable: true;
        },
        {
          name: 'userUsdtAccount';
          docs: ['用户 USDT 账户'];
          writable: true;
        },
        {
          name: 'userWsolAccount';
          docs: ['用户 WSOL 账户'];
          writable: true;
        },
        {
          name: 'userSuperiorUsdtAccount';
          docs: ['上级 USDT 账户'];
          writable: true;
        },
        {
          name: 'marketPoolAddressUsdtAccount';
          docs: ['市场分红 USDT 账户'];
          writable: true;
        },
        {
          name: 'usdtMintAccount';
          docs: ['USDT Mint'];
          writable: true;
        },
        {
          name: 'swapAccounts';
          accounts: [
            {
              name: 'ammProgram';
            },
            {
              name: 'amm';
              writable: true;
            },
            {
              name: 'ammAuthority';
            },
            {
              name: 'ammOpenOrders';
              writable: true;
            },
            {
              name: 'ammCoinVault';
              writable: true;
            },
            {
              name: 'ammPcVault';
              writable: true;
            },
            {
              name: 'marketProgram';
            },
            {
              name: 'market';
              writable: true;
            },
            {
              name: 'marketBids';
              writable: true;
            },
            {
              name: 'marketAsks';
              writable: true;
            },
            {
              name: 'marketEventQueue';
              writable: true;
            },
            {
              name: 'marketCoinVault';
              writable: true;
            },
            {
              name: 'marketPcVault';
              writable: true;
            },
            {
              name: 'marketVaultSigner';
              writable: true;
            },
            {
              name: 'userTokenSource';
              writable: true;
            },
            {
              name: 'userTokenDestination';
              writable: true;
            },
            {
              name: 'userSourceOwner';
              writable: true;
              signer: true;
            },
            {
              name: 'tokenProgram';
              address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
            },
          ];
        },
        {
          name: 'tokenProgram';
          address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
        },
        {
          name: 'systemProgram';
          address: '11111111111111111111111111111111';
        },
      ];
      args: [
        {
          name: 'usdtAmount';
          type: 'u64';
        },
        {
          name: 'expectedWsolAmount';
          type: 'u64';
        },
      ];
    },
    {
      name: 'ammWsolGdtc';
      docs: ['使用 Raydium AMM V4 将 WSOL 兑换为 GDTC (swap_base_in)'];
      discriminator: [94, 238, 185, 110, 47, 184, 91, 28];
      accounts: [
        {
          name: 'user';
          writable: true;
          signer: true;
        },
        {
          name: 'nftMiningSystem';
          docs: ['系统 PDA（已初始化）'];
          writable: true;
          pda: {
            seeds: [
              {
                kind: 'const';
                value: [
                  110,
                  102,
                  116,
                  95,
                  109,
                  105,
                  110,
                  105,
                  110,
                  103,
                  95,
                  115,
                  121,
                  115,
                  116,
                  101,
                  109,
                ];
              },
            ];
          };
        },
        {
          name: 'orderInfo';
          docs: ['用户状态账户'];
          writable: true;
          pda: {
            seeds: [
              {
                kind: 'const';
                value: [111, 114, 100, 101, 114, 95, 105, 110, 102, 111];
              },
              {
                kind: 'account';
                path: 'order_info.order_info_index';
                account: 'orderInfo';
              },
            ];
          };
        },
        {
          name: 'userWsolAccount';
          writable: true;
        },
        {
          name: 'userGdtcAccount';
          writable: true;
        },
        {
          name: 'blackHoleGdtcAccount';
          writable: true;
        },
        {
          name: 'swapAccounts';
          accounts: [
            {
              name: 'ammProgram';
            },
            {
              name: 'amm';
              writable: true;
            },
            {
              name: 'ammAuthority';
            },
            {
              name: 'ammOpenOrders';
              writable: true;
            },
            {
              name: 'ammCoinVault';
              writable: true;
            },
            {
              name: 'ammPcVault';
              writable: true;
            },
            {
              name: 'marketProgram';
            },
            {
              name: 'market';
              writable: true;
            },
            {
              name: 'marketBids';
              writable: true;
            },
            {
              name: 'marketAsks';
              writable: true;
            },
            {
              name: 'marketEventQueue';
              writable: true;
            },
            {
              name: 'marketCoinVault';
              writable: true;
            },
            {
              name: 'marketPcVault';
              writable: true;
            },
            {
              name: 'marketVaultSigner';
              writable: true;
            },
            {
              name: 'userTokenSource';
              writable: true;
            },
            {
              name: 'userTokenDestination';
              writable: true;
            },
            {
              name: 'userSourceOwner';
              writable: true;
              signer: true;
            },
            {
              name: 'tokenProgram';
              address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
            },
          ];
        },
        {
          name: 'tokenProgram';
          address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
        },
        {
          name: 'systemProgram';
          address: '11111111111111111111111111111111';
        },
      ];
      args: [
        {
          name: 'minimumGdtcAmount';
          type: 'u64';
        },
      ];
    },
    {
      name: 'cancelStaking';
      discriminator: [2, 183, 90, 76, 17, 140, 174, 5];
      accounts: [
        {
          name: 'admin';
          docs: ['管理员签名者'];
          writable: true;
          signer: true;
        },
        {
          name: 'nftMiningSystem';
          docs: ['系统 PDA（已初始化）'];
          writable: true;
          pda: {
            seeds: [
              {
                kind: 'const';
                value: [
                  110,
                  102,
                  116,
                  95,
                  109,
                  105,
                  110,
                  105,
                  110,
                  103,
                  95,
                  115,
                  121,
                  115,
                  116,
                  101,
                  109,
                ];
              },
            ];
          };
        },
        {
          name: 'orderInfo';
          docs: ['用户状态账户'];
          writable: true;
          pda: {
            seeds: [
              {
                kind: 'const';
                value: [111, 114, 100, 101, 114, 95, 105, 110, 102, 111];
              },
              {
                kind: 'account';
                path: 'order_info.order_info_index';
                account: 'orderInfo';
              },
            ];
          };
        },
        {
          name: 'userAddress';
          docs: ['用户地址（用于验证）'];
        },
        {
          name: 'systemProgram';
          docs: ['系统程序'];
          address: '11111111111111111111111111111111';
        },
      ];
      args: [
        {
          name: 'reduceAmount';
          type: 'u64';
        },
      ];
    },
    {
      name: 'claimNft';
      discriminator: [6, 193, 146, 120, 48, 218, 69, 33];
      accounts: [
        {
          name: 'admin';
          docs: ['管理员签名者'];
          writable: true;
          signer: true;
        },
        {
          name: 'nftMiningSystem';
          docs: ['系统 PDA（已初始化）'];
          writable: true;
          pda: {
            seeds: [
              {
                kind: 'const';
                value: [
                  110,
                  102,
                  116,
                  95,
                  109,
                  105,
                  110,
                  105,
                  110,
                  103,
                  95,
                  115,
                  121,
                  115,
                  116,
                  101,
                  109,
                ];
              },
            ];
          };
        },
        {
          name: 'orderInfo';
          docs: ['用户状态账户'];
          writable: true;
          pda: {
            seeds: [
              {
                kind: 'const';
                value: [111, 114, 100, 101, 114, 95, 105, 110, 102, 111];
              },
              {
                kind: 'account';
                path: 'order_info.order_info_index';
                account: 'orderInfo';
              },
            ];
          };
        },
        {
          name: 'userAddress';
          docs: ['用户地址（用于验证）'];
        },
        {
          name: 'systemProgram';
          docs: ['系统程序'];
          address: '11111111111111111111111111111111';
        },
      ];
      args: [
        {
          name: 'nftMintAddress';
          type: 'pubkey';
        },
      ];
    },
    {
      name: 'claimRewards';
      discriminator: [4, 144, 132, 71, 116, 23, 151, 80];
      accounts: [
        {
          name: 'user';
          docs: ['用户签名者'];
          writable: true;
          signer: true;
        },
        {
          name: 'nftMiningSystem';
          docs: ['系统 PDA（已初始化）'];
          writable: true;
          pda: {
            seeds: [
              {
                kind: 'const';
                value: [
                  110,
                  102,
                  116,
                  95,
                  109,
                  105,
                  110,
                  105,
                  110,
                  103,
                  95,
                  115,
                  121,
                  115,
                  116,
                  101,
                  109,
                ];
              },
            ];
          };
        },
        {
          name: 'orderInfo';
          docs: ['用户状态账户'];
          writable: true;
          pda: {
            seeds: [
              {
                kind: 'const';
                value: [111, 114, 100, 101, 114, 95, 105, 110, 102, 111];
              },
              {
                kind: 'account';
                path: 'order_info.order_info_index';
                account: 'orderInfo';
              },
            ];
          };
        },
        {
          name: 'userBioAccount';
          docs: ['用户的 BIO Token 账户（用于接收奖励）'];
          writable: true;
        },
        {
          name: 'poolBioAccount';
          docs: ['全网分红池的 BIO Token 账户'];
          writable: true;
        },
        {
          name: 'systemBioAccount';
          docs: ['系统 BIO 奖励账户（Vault）'];
          writable: true;
        },
        {
          name: 'userSuperiorTokenAccount';
          writable: true;
        },
        {
          name: 'blackHoleBioAccount';
          docs: ['黑洞地址 BIO Token 账户'];
          writable: true;
        },
        {
          name: 'bioMint';
          docs: ['BIO Mint 账户'];
        },
        {
          name: 'systemProgram';
          docs: ['系统程序'];
          address: '11111111111111111111111111111111';
        },
        {
          name: 'tokenProgram';
          docs: ['Token 程序'];
          address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
        },
      ];
      args: [];
    },
    {
      name: 'enterStaking';
      discriminator: [6, 26, 79, 73, 227, 51, 221, 91];
      accounts: [
        {
          name: 'user';
          docs: ['用户签名者'];
          writable: true;
          signer: true;
        },
        {
          name: 'nftMiningSystem';
          docs: ['系统 PDA（已初始化）'];
          writable: true;
          pda: {
            seeds: [
              {
                kind: 'const';
                value: [
                  110,
                  102,
                  116,
                  95,
                  109,
                  105,
                  110,
                  105,
                  110,
                  103,
                  95,
                  115,
                  121,
                  115,
                  116,
                  101,
                  109,
                ];
              },
            ];
          };
        },
        {
          name: 'orderInfo';
          docs: ['用户状态账户'];
          writable: true;
          pda: {
            seeds: [
              {
                kind: 'const';
                value: [111, 114, 100, 101, 114, 95, 105, 110, 102, 111];
              },
              {
                kind: 'account';
                path: 'order_info.order_info_index';
                account: 'orderInfo';
              },
            ];
          };
        },
        {
          name: 'systemProgram';
          docs: ['系统程序'];
          address: '11111111111111111111111111111111';
        },
      ];
      args: [];
    },
    {
      name: 'gdtcToBio';
      discriminator: [65, 94, 146, 190, 122, 187, 215, 213];
      accounts: [
        {
          name: 'user';
          writable: true;
          signer: true;
        },
        {
          name: 'nftMiningSystem';
          docs: ['系统 PDA（已初始化）'];
          writable: true;
          pda: {
            seeds: [
              {
                kind: 'const';
                value: [
                  110,
                  102,
                  116,
                  95,
                  109,
                  105,
                  110,
                  105,
                  110,
                  103,
                  95,
                  115,
                  121,
                  115,
                  116,
                  101,
                  109,
                ];
              },
            ];
          };
        },
        {
          name: 'orderInfo';
          docs: ['用户状态账户'];
          writable: true;
          pda: {
            seeds: [
              {
                kind: 'const';
                value: [111, 114, 100, 101, 114, 95, 105, 110, 102, 111];
              },
              {
                kind: 'account';
                path: 'order_info.order_info_index';
                account: 'orderInfo';
              },
            ];
          };
        },
        {
          name: 'userGdtcAccount';
          writable: true;
        },
        {
          name: 'userBioAccount';
          writable: true;
        },
        {
          name: 'blackHoleBioAccount';
          writable: true;
        },
        {
          name: 'poolAddressBioMint';
          writable: true;
        },
        {
          name: 'gdtcMint';
          writable: true;
        },
        {
          name: 'bioMint';
          writable: true;
        },
        {
          name: 'cpSwapProgram';
          address: 'DRaycpLY18LhpbydsBWbVJtxpNv9oXPgjRSfpF2bWpYb';
        },
        {
          name: 'authority';
          pda: {
            seeds: [
              {
                kind: 'const';
                value: [
                  118,
                  97,
                  117,
                  108,
                  116,
                  95,
                  97,
                  110,
                  100,
                  95,
                  108,
                  112,
                  95,
                  109,
                  105,
                  110,
                  116,
                  95,
                  97,
                  117,
                  116,
                  104,
                  95,
                  115,
                  101,
                  101,
                  100,
                ];
              },
            ];
            program: {
              kind: 'account';
              path: 'cpSwapProgram';
            };
          };
        },
        {
          name: 'ammConfig';
          docs: ['GDTC-BIO 兑换池的 AMM 配置'];
          writable: true;
        },
        {
          name: 'poolState';
          docs: ['GDTC-BIO 兑换池状态'];
          writable: true;
        },
        {
          name: 'inputVault';
          docs: ['GDTC 输入金库'];
          writable: true;
        },
        {
          name: 'outputVault';
          docs: ['BIO 输出金库'];
          writable: true;
        },
        {
          name: 'inputTokenProgram';
          docs: ['GDTC 输入代币程序'];
        },
        {
          name: 'outputTokenProgram';
          docs: ['BIO 输出代币程序'];
        },
        {
          name: 'inputTokenMint';
          docs: ['GDTC 输入代币 Mint'];
        },
        {
          name: 'outputTokenMint';
          docs: ['BIO 输出代币 Mint'];
        },
        {
          name: 'observationState';
          docs: ['GDTC-BIO 兑换池的观察状态'];
          writable: true;
        },
        {
          name: 'tokenProgram';
          address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
        },
        {
          name: 'systemProgram';
          address: '11111111111111111111111111111111';
        },
      ];
      args: [];
    },
    {
      name: 'initializeSystem';
      docs: ['初始化NFT算力挖矿系统'];
      discriminator: [50, 173, 248, 140, 202, 35, 141, 150];
      accounts: [
        {
          name: 'nftMiningSystem';
          writable: true;
          pda: {
            seeds: [
              {
                kind: 'const';
                value: [
                  110,
                  102,
                  116,
                  95,
                  109,
                  105,
                  110,
                  105,
                  110,
                  103,
                  95,
                  115,
                  121,
                  115,
                  116,
                  101,
                  109,
                ];
              },
            ];
          };
        },
        {
          name: 'authority';
          writable: true;
          signer: true;
        },
        {
          name: 'usdtMintAccount';
        },
        {
          name: 'systemProgram';
          address: '11111111111111111111111111111111';
        },
      ];
      args: [
        {
          name: 'totalSupply';
          type: 'u64';
        },
        {
          name: 'dailyOutput';
          type: 'u64';
        },
        {
          name: 'startTimestamp';
          type: 'u64';
        },
        {
          name: 'poolAddress';
          type: 'pubkey';
        },
        {
          name: 'marketPoolAddress';
          type: 'pubkey';
        },
        {
          name: 'gdtcMint';
          type: 'pubkey';
        },
        {
          name: 'bioMint';
          type: 'pubkey';
        },
        {
          name: 'wsolMint';
          type: 'pubkey';
        },
        {
          name: 'admin';
          type: 'pubkey';
        },
        {
          name: 'blackHoleAddress';
          type: 'pubkey';
        },
      ];
    },
    {
      name: 'usdtWsol';
      discriminator: [251, 64, 252, 153, 209, 169, 177, 176];
      accounts: [
        {
          name: 'user';
          docs: ['付款人与接收者（本例中一致；若要送给他人，可把 recipient 改成独立账户）'];
          writable: true;
          signer: true;
        },
        {
          name: 'nftMiningSystem';
          docs: ['系统 PDA（已初始化，持有 Master 那 1 枚）'];
          writable: true;
          pda: {
            seeds: [
              {
                kind: 'const';
                value: [
                  110,
                  102,
                  116,
                  95,
                  109,
                  105,
                  110,
                  105,
                  110,
                  103,
                  95,
                  115,
                  121,
                  115,
                  116,
                  101,
                  109,
                ];
              },
            ];
          };
        },
        {
          name: 'orderInfo';
          docs: ['用户状态（按用户+固定种子派生，避免多用户冲突）'];
          writable: true;
        },
        {
          name: 'userUsdtAccount';
          writable: true;
        },
        {
          name: 'userWsolAccount';
          writable: true;
        },
        {
          name: 'userSuperiorUsdtAccount';
          writable: true;
        },
        {
          name: 'marketPoolAddressUsdtAccount';
          writable: true;
        },
        {
          name: 'usdtMintAccount';
          writable: true;
        },
        {
          name: 'cpSwapProgram';
          address: 'DRaycpLY18LhpbydsBWbVJtxpNv9oXPgjRSfpF2bWpYb';
        },
        {
          name: 'authority';
          pda: {
            seeds: [
              {
                kind: 'const';
                value: [
                  118,
                  97,
                  117,
                  108,
                  116,
                  95,
                  97,
                  110,
                  100,
                  95,
                  108,
                  112,
                  95,
                  109,
                  105,
                  110,
                  116,
                  95,
                  97,
                  117,
                  116,
                  104,
                  95,
                  115,
                  101,
                  101,
                  100,
                ];
              },
            ];
            program: {
              kind: 'account';
              path: 'cpSwapProgram';
            };
          };
        },
        {
          name: 'ammConfig';
          docs: ['usdt-wsol 兑换池的 AMM 配置'];
          writable: true;
        },
        {
          name: 'poolState';
          docs: ['usdt-wsol 兑换池状态'];
          writable: true;
        },
        {
          name: 'inputVault';
          docs: ['usdt 输入金库'];
          writable: true;
        },
        {
          name: 'outputVault';
          docs: ['wsol 输出金库'];
          writable: true;
        },
        {
          name: 'inputTokenProgram';
          docs: ['usdt 输入代币程序'];
        },
        {
          name: 'outputTokenProgram';
          docs: ['wsol 输出代币程序'];
        },
        {
          name: 'inputTokenMint';
          docs: ['usdt 输入代币 Mint'];
        },
        {
          name: 'outputTokenMint';
          docs: ['wsol 输出代币 Mint'];
        },
        {
          name: 'observationState';
          docs: ['usdt-wsol 兑换池的观察状态'];
          writable: true;
        },
        {
          name: 'tokenProgram';
          address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
        },
        {
          name: 'systemProgram';
          address: '11111111111111111111111111111111';
        },
      ];
      args: [
        {
          name: 'usdtAmount';
          type: 'u64';
        },
      ];
    },
    {
      name: 'wsolGdtc';
      docs: ['使用WSOL购买GDTC并销毁一半（CP Swap）'];
      discriminator: [109, 2, 124, 173, 42, 200, 106, 137];
      accounts: [
        {
          name: 'user';
          writable: true;
          signer: true;
        },
        {
          name: 'nftMiningSystem';
          docs: ['系统 PDA（已初始化）'];
          writable: true;
          pda: {
            seeds: [
              {
                kind: 'const';
                value: [
                  110,
                  102,
                  116,
                  95,
                  109,
                  105,
                  110,
                  105,
                  110,
                  103,
                  95,
                  115,
                  121,
                  115,
                  116,
                  101,
                  109,
                ];
              },
            ];
          };
        },
        {
          name: 'orderInfo';
          docs: ['用户状态账户'];
          writable: true;
          pda: {
            seeds: [
              {
                kind: 'const';
                value: [111, 114, 100, 101, 114, 95, 105, 110, 102, 111];
              },
              {
                kind: 'account';
                path: 'order_info.order_info_index';
                account: 'orderInfo';
              },
            ];
          };
        },
        {
          name: 'userWsolAccount';
          writable: true;
        },
        {
          name: 'userGdtcAccount';
          writable: true;
        },
        {
          name: 'blackHoleGdtcAccount';
          writable: true;
        },
        {
          name: 'cpSwapProgram';
          address: 'DRaycpLY18LhpbydsBWbVJtxpNv9oXPgjRSfpF2bWpYb';
        },
        {
          name: 'authority';
          pda: {
            seeds: [
              {
                kind: 'const';
                value: [
                  118,
                  97,
                  117,
                  108,
                  116,
                  95,
                  97,
                  110,
                  100,
                  95,
                  108,
                  112,
                  95,
                  109,
                  105,
                  110,
                  116,
                  95,
                  97,
                  117,
                  116,
                  104,
                  95,
                  115,
                  101,
                  101,
                  100,
                ];
              },
            ];
            program: {
              kind: 'account';
              path: 'cpSwapProgram';
            };
          };
        },
        {
          name: 'ammConfig';
          docs: ['WSOL-GDTC 兑换池的 AMM 配置'];
          writable: true;
        },
        {
          name: 'poolState';
          docs: ['WSOL-GDTC 兑换池状态'];
          writable: true;
        },
        {
          name: 'inputVault';
          docs: ['WSOL 输入金库'];
          writable: true;
        },
        {
          name: 'outputVault';
          docs: ['GDTC 输出金库'];
          writable: true;
        },
        {
          name: 'inputTokenProgram';
          docs: ['WSOL 输入代币程序'];
        },
        {
          name: 'outputTokenProgram';
          docs: ['GDTC 输出代币程序'];
        },
        {
          name: 'inputTokenMint';
          docs: ['WSOL 输入代币 Mint'];
        },
        {
          name: 'outputTokenMint';
          docs: ['GDTC 输出代币 Mint'];
        },
        {
          name: 'observationState';
          docs: ['WSOL-GDTC 兑换池的观察状态'];
          writable: true;
        },
        {
          name: 'tokenProgram';
          address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
        },
        {
          name: 'systemProgram';
          address: '11111111111111111111111111111111';
        },
      ];
      args: [];
    },
  ];
  accounts: [
    {
      name: 'ammConfig';
      discriminator: [218, 244, 33, 104, 203, 203, 43, 111];
    },
    {
      name: 'nftMiningSystem';
      discriminator: [145, 115, 180, 141, 211, 2, 118, 220];
    },
    {
      name: 'observationState';
      discriminator: [122, 174, 197, 53, 129, 9, 165, 132];
    },
    {
      name: 'orderInfo';
      discriminator: [89, 17, 178, 164, 42, 111, 227, 211];
    },
    {
      name: 'poolState';
      discriminator: [247, 237, 227, 245, 215, 195, 222, 70];
    },
  ];
  errors: [
    {
      code: 6000;
      name: 'unauthorized';
      msg: 'unauthorized';
    },
    {
      code: 6001;
      name: 'userAccountIsNotMatch';
      msg: 'userAccountIsNotMatch';
    },
    {
      code: 6002;
      name: 'userNotInitialized';
      msg: 'userNotInitialized';
    },
    {
      code: 6003;
      name: 'userAlreadyStaked';
      msg: 'userAlreadyStaked';
    },
    {
      code: 6004;
      name: 'invalidWsolMint';
      msg: 'invalidWsolMint';
    },
    {
      code: 6005;
      name: 'invalidWsolOwner';
      msg: 'invalidWsolOwner';
    },
    {
      code: 6006;
      name: 'invalidNftMint';
      msg: 'invalidNftMint';
    },
    {
      code: 6007;
      name: 'invalidNftOwner';
      msg: 'invalidNftOwner';
    },
    {
      code: 6008;
      name: 'invalidTierLevel';
      msg: 'invalidTierLevel';
    },
    {
      code: 6009;
      name: 'insufficientWsolBalance';
      msg: 'insufficientWsolBalance';
    },
    {
      code: 6010;
      name: 'invalidWsolAmount';
      msg: 'invalidWsolAmount';
    },
    {
      code: 6011;
      name: 'invalidAccountArrayLength';
      msg: 'invalidAccountArrayLength';
    },
    {
      code: 6012;
      name: 'notEnoughRemainingAccounts';
      msg: 'notEnoughRemainingAccounts';
    },
    {
      code: 6013;
      name: 'invalidGdtcMint';
      msg: 'invalidGdtcMint';
    },
    {
      code: 6014;
      name: 'invalidGdtcOwner';
      msg: 'invalidGdtcOwner';
    },
    {
      code: 6015;
      name: 'invalidBioMint';
      msg: 'invalidBioMint';
    },
    {
      code: 6016;
      name: 'invalidPoolAddress';
      msg: 'invalidPoolAddress';
    },
    {
      code: 6017;
      name: 'invalidMarketPoolAddress';
      msg: 'invalidMarketPoolAddress';
    },
    {
      code: 6018;
      name: 'invalidAmount';
      msg: 'invalidAmount';
    },
    {
      code: 6019;
      name: 'insufficientBalance';
      msg: 'insufficientBalance';
    },
    {
      code: 6020;
      name: 'invalidTokenMint';
      msg: 'invalidTokenMint';
    },
    {
      code: 6021;
      name: 'systemAlreadyInitialized';
      msg: 'systemAlreadyInitialized';
    },
    {
      code: 6022;
      name: 'invalidUsdtMint';
      msg: 'invalidUsdtMint';
    },
    {
      code: 6023;
      name: 'invalidUsdtOwner';
      msg: 'invalidUsdtOwner';
    },
    {
      code: 6024;
      name: 'invalidUsdtAmount';
      msg: 'invalidUsdtAmount';
    },
    {
      code: 6025;
      name: 'insufficientUsdtBalance';
      msg: 'insufficientUsdtBalance';
    },
    {
      code: 6026;
      name: 'arithmeticOverflow';
      msg: 'arithmeticOverflow';
    },
    {
      code: 6027;
      name: 'noRewardsToClaim';
      msg: 'noRewardsToClaim';
    },
    {
      code: 6028;
      name: 'maxRewardsToClaim';
      msg: 'maxRewardsToClaim';
    },
    {
      code: 6029;
      name: 'tokenMintMismatch';
      msg: 'tokenMintMismatch';
    },
    {
      code: 6030;
      name: 'notBlackHole';
      msg: 'notBlackHole';
    },
    {
      code: 6031;
      name: 'insufficientOutputAmount';
      msg: 'insufficientOutputAmount';
    },
  ];
  types: [
    {
      name: 'ammConfig';
      docs: ['Holds the current owner of the factory'];
      type: {
        kind: 'struct';
        fields: [
          {
            name: 'bump';
            docs: ['Bump to identify PDA'];
            type: 'u8';
          },
          {
            name: 'disableCreatePool';
            docs: ['Status to control if new pool can be create'];
            type: 'bool';
          },
          {
            name: 'index';
            docs: ['Config index'];
            type: 'u16';
          },
          {
            name: 'tradeFeeRate';
            docs: ['The trade fee, denominated in hundredths of a bip (10^-6)'];
            type: 'u64';
          },
          {
            name: 'protocolFeeRate';
            docs: ['The protocol fee'];
            type: 'u64';
          },
          {
            name: 'fundFeeRate';
            docs: ['The fund fee, denominated in hundredths of a bip (10^-6)'];
            type: 'u64';
          },
          {
            name: 'createPoolFee';
            docs: ['Fee for create a new pool'];
            type: 'u64';
          },
          {
            name: 'protocolOwner';
            docs: ['Address of the protocol fee owner'];
            type: 'pubkey';
          },
          {
            name: 'fundOwner';
            docs: ['Address of the fund fee owner'];
            type: 'pubkey';
          },
          {
            name: 'creatorFeeRate';
            docs: ['The pool creator fee, denominated in hundredths of a bip (10^-6)'];
            type: 'u64';
          },
          {
            name: 'padding';
            docs: ['padding'];
            type: {
              array: ['u64', 15];
            };
          },
        ];
      };
    },
    {
      name: 'nftMiningSystem';
      type: {
        kind: 'struct';
        fields: [
          {
            name: 'authority';
            type: 'pubkey';
          },
          {
            name: 'isInitialized';
            type: 'bool';
          },
          {
            name: 'usdtMint';
            type: 'pubkey';
          },
          {
            name: 'wsolMint';
            type: 'pubkey';
          },
          {
            name: 'gdtcMint';
            type: 'pubkey';
          },
          {
            name: 'bioMint';
            type: 'pubkey';
          },
          {
            name: 'poolAddress';
            type: 'pubkey';
          },
          {
            name: 'marketPoolAddress';
            type: 'pubkey';
          },
          {
            name: 'blackHoleAddress';
            type: 'pubkey';
          },
          {
            name: 'admin';
            type: 'pubkey';
          },
          {
            name: 'totalSupply';
            type: 'u64';
          },
          {
            name: 'dailyOutput';
            type: 'u64';
          },
          {
            name: 'startTimestamp';
            type: 'u64';
          },
          {
            name: 'pool';
            type: {
              defined: {
                name: 'stakingPool';
              };
            };
          },
          {
            name: 'orderInfoIndex';
            type: 'u64';
          },
        ];
      };
    },
    {
      name: 'observation';
      docs: ['The element of observations in ObservationState'];
      serialization: 'bytemuckunsafe';
      repr: {
        kind: 'c';
        packed: true;
      };
      type: {
        kind: 'struct';
        fields: [
          {
            name: 'blockTimestamp';
            docs: ['The block timestamp of the observation'];
            type: 'u64';
          },
          {
            name: 'cumulativeToken0PriceX32';
            docs: [
              'the cumulative of token0 price during the duration time, Q32.32, the remaining 64 bit for overflow',
            ];
            type: 'u128';
          },
          {
            name: 'cumulativeToken1PriceX32';
            docs: [
              'the cumulative of token1 price during the duration time, Q32.32, the remaining 64 bit for overflow',
            ];
            type: 'u128';
          },
        ];
      };
    },
    {
      name: 'observationState';
      serialization: 'bytemuckunsafe';
      repr: {
        kind: 'c';
        packed: true;
      };
      type: {
        kind: 'struct';
        fields: [
          {
            name: 'initialized';
            docs: ['Whether the ObservationState is initialized'];
            type: 'bool';
          },
          {
            name: 'observationIndex';
            docs: ['the most-recently updated index of the observations array'];
            type: 'u16';
          },
          {
            name: 'poolId';
            type: 'pubkey';
          },
          {
            name: 'observations';
            docs: ['observation array'];
            type: {
              array: [
                {
                  defined: {
                    name: 'observation';
                  };
                },
                100,
              ];
            };
          },
          {
            name: 'padding';
            docs: ['padding for feature update'];
            type: {
              array: ['u64', 4];
            };
          },
        ];
      };
    },
    {
      name: 'orderInfo';
      type: {
        kind: 'struct';
        fields: [
          {
            name: 'userAddress';
            type: 'pubkey';
          },
          {
            name: 'orderInfoIndex';
            type: 'u64';
          },
          {
            name: 'userSuperiorAccount';
            type: 'pubkey';
          },
          {
            name: 'totalPower';
            type: 'u64';
          },
          {
            name: 'accumulatedReward';
            type: 'u64';
          },
          {
            name: 'lastClaimTimestamp';
            type: 'u64';
          },
          {
            name: 'investmentAmount';
            type: 'u64';
          },
          {
            name: 'isTransferUsdt';
            type: 'bool';
          },
          {
            name: 'transferWsolAmount';
            type: 'u64';
          },
          {
            name: 'isInit';
            type: 'bool';
          },
          {
            name: 'stakeStartTime';
            type: 'u64';
          },
          {
            name: 'rewardDebt';
            type: 'u64';
          },
          {
            name: 'isStaked';
            type: 'bool';
          },
          {
            name: 'receivedReward';
            type: 'u64';
          },
          {
            name: 'gdtcAmount';
            type: 'u64';
          },
          {
            name: 'burnGdtc';
            type: 'bool';
          },
          {
            name: 'remainingGdtc';
            type: 'u64';
          },
          {
            name: 'bioAmount';
            type: 'u64';
          },
          {
            name: 'burnBio';
            type: 'bool';
          },
          {
            name: 'isNftMinted';
            type: 'bool';
          },
          {
            name: 'nftMintedTime';
            type: 'u64';
          },
          {
            name: 'nftMintAddress';
            type: 'pubkey';
          },
        ];
      };
    },
    {
      name: 'poolState';
      serialization: 'bytemuckunsafe';
      repr: {
        kind: 'c';
        packed: true;
      };
      type: {
        kind: 'struct';
        fields: [
          {
            name: 'ammConfig';
            docs: ['Which config the pool belongs'];
            type: 'pubkey';
          },
          {
            name: 'poolCreator';
            docs: ['pool creator'];
            type: 'pubkey';
          },
          {
            name: 'token0Vault';
            docs: ['Token A'];
            type: 'pubkey';
          },
          {
            name: 'token1Vault';
            docs: ['Token B'];
            type: 'pubkey';
          },
          {
            name: 'lpMint';
            docs: [
              'Pool tokens are issued when A or B tokens are deposited.',
              'Pool tokens can be withdrawn back to the original A or B token.',
            ];
            type: 'pubkey';
          },
          {
            name: 'token0Mint';
            docs: ['Mint information for token A'];
            type: 'pubkey';
          },
          {
            name: 'token1Mint';
            docs: ['Mint information for token B'];
            type: 'pubkey';
          },
          {
            name: 'token0Program';
            docs: ['token_0 program'];
            type: 'pubkey';
          },
          {
            name: 'token1Program';
            docs: ['token_1 program'];
            type: 'pubkey';
          },
          {
            name: 'observationKey';
            docs: ['observation account to store oracle data'];
            type: 'pubkey';
          },
          {
            name: 'authBump';
            type: 'u8';
          },
          {
            name: 'status';
            docs: [
              'Bitwise representation of the state of the pool',
              'bit0, 1: disable deposit(value is 1), 0: normal',
              'bit1, 1: disable withdraw(value is 2), 0: normal',
              'bit2, 1: disable swap(value is 4), 0: normal',
            ];
            type: 'u8';
          },
          {
            name: 'lpMintDecimals';
            type: 'u8';
          },
          {
            name: 'mint0Decimals';
            docs: ['mint0 and mint1 decimals'];
            type: 'u8';
          },
          {
            name: 'mint1Decimals';
            type: 'u8';
          },
          {
            name: 'lpSupply';
            docs: ['True circulating supply without burns and lock ups'];
            type: 'u64';
          },
          {
            name: 'protocolFeesToken0';
            docs: ['The amounts of token_0 and token_1 that are owed to the liquidity provider.'];
            type: 'u64';
          },
          {
            name: 'protocolFeesToken1';
            type: 'u64';
          },
          {
            name: 'fundFeesToken0';
            type: 'u64';
          },
          {
            name: 'fundFeesToken1';
            type: 'u64';
          },
          {
            name: 'openTime';
            docs: ['The timestamp allowed for swap in the pool.'];
            type: 'u64';
          },
          {
            name: 'recentEpoch';
            docs: ['recent epoch'];
            type: 'u64';
          },
          {
            name: 'creatorFeeOn';
            docs: [
              'Creator fee collect mode',
              '0: both token_0 and token_1 can be used as trade fees. It depends on what the input token is when swapping',
              '1: only token_0 as trade fee',
              '2: only token_1 as trade fee',
            ];
            type: 'u8';
          },
          {
            name: 'enableCreatorFee';
            type: 'bool';
          },
          {
            name: 'padding1';
            type: {
              array: ['u8', 6];
            };
          },
          {
            name: 'creatorFeesToken0';
            type: 'u64';
          },
          {
            name: 'creatorFeesToken1';
            type: 'u64';
          },
          {
            name: 'padding';
            docs: ['padding for future updates'];
            type: {
              array: ['u64', 28];
            };
          },
        ];
      };
    },
    {
      name: 'stakingPool';
      type: {
        kind: 'struct';
        fields: [
          {
            name: 'rewardTokenPerSec';
            type: 'u64';
          },
          {
            name: 'accumulatedRewardPerShare';
            type: 'u64';
          },
          {
            name: 'lastRewardTimestamp';
            type: 'u64';
          },
          {
            name: 'totalShares';
            type: 'u64';
          },
        ];
      };
    },
  ];
};
