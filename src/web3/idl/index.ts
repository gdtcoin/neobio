export * from './crowdfunding';
export * from './nft';
export * from './staking';
export * from './vesting';

export const IDL =
  process.env.ENV === 'production'
    ? {
        crowdfunding: require('./crowdfunding-prod.json'),
        nft: require('./nft-prod.json'),
        staking: require('./staking.json'),
        vesting: require('./vesting.json'),
      }
    : {
        crowdfunding: require('./crowdfunding.json'),
        nft: require('./nft.json'),
        staking: require('./staking.json'),
        vesting: require('./vesting.json'),
      };
