<template>
  <div class="user-page text-black">
    <top-bar :back="false" :title="$t('user')">
      <template #right>
        <Locale />
      </template>
    </top-bar>

    <van-tabs v-model:active="state.activeTab" swipeable>
      <van-tab :title="$t('user')">
        <div class="user-fields bg-card">
          <div class="field-item">
            <img src="@/app/assets/images/ico_code.png" class="icon" />
            <div class="item-label">{{ $t('myBioneo') }}</div>
            <div class="item-value">
              {{ state.bioneoBalance }}
            </div>
          </div>

          <div class="field-item" @click="copy(store.userInfo?.user_code)">
            <img src="@/app/assets/images/ico_code.png" class="icon" />
            <div class="item-label">{{ $t('myInviteCode') }}</div>
            <div class="item-value">
              {{ store.userInfo?.user_code }}
            </div>
            <img src="@/app/assets/images/copy.png" class="copy-icon" />
          </div>
          <div class="field-item" @click="copy(store.userInfo?.team_user_code)">
            <img src="@/app/assets/images/ico_code.png" class="icon" />
            <div class="item-label">{{ $t('teamCode') }}</div>
            <div class="item-value">
              {{ store.userInfo?.team_user_code }}
            </div>
            <img src="@/app/assets/images/copy.png" class="copy-icon" />
          </div>
          <div class="field-item" @click="copy(store.walletAddress)">
            <img src="@/app/assets/images/ico_address.png" class="icon" />
            <div class="item-label">{{ $t('myAddress') }}</div>
            <div class="item-value">
              {{ formatWallet(store.walletAddress) }}
            </div>
            <img src="@/app/assets/images/copy.png" class="copy-icon" />
          </div>
          <div
            v-if="store.userInfo?.parent_address"
            class="field-item"
            @click="copy(store.userInfo?.parent_address)"
          >
            <img src="@/app/assets/images/ico_superior.png" class="icon" />
            <div class="item-label">{{ $t('mySuperior') }}</div>
            <div class="item-value">
              {{ formatWallet(store.userInfo?.parent_address) }}
            </div>
          </div>
          <div v-if="hasVesting" class="field-item" @click="router.push('/user/vesting')">
            <img src="@/app/assets/images/ico_vesting.png" class="icon" />
            <div class="item-label">{{ $t('vesting') }}</div>
            <div class="item-value">
              <van-icon name="arrow" />
            </div>
          </div>
          <div class="field-item" @click="router.push('/crowdfunding/mine')">
            <img src="@/app/assets/images/ico_address.png" class="icon" />
            <div class="item-label">{{ $t('myNode') }}</div>
            <div class="item-value">
              <van-icon name="arrow" />
            </div>
          </div>
          <div
            v-if="state.crowdfundingEligibility"
            class="field-item"
            @click="handleApplyCrowdfunding"
          >
            <img src="@/app/assets/images/ico_address.png" class="icon" />
            <div class="item-label">{{ $t('applyCrowdfunding') }}</div>
            <div class="item-value">
              {{ getCrowdfundingStatus(state.crowdfundingStatus) }}
              <van-icon name="arrow" />
            </div>
          </div>
        </div>
      </van-tab>

      <van-tab :title="$t('myDirectReferrals')">
        <div
          v-if="!store.userInfo?.team_parent_code"
          class="bg-card m-[12px] rounded-[8px] p-[16px]"
        >
          <van-field
            v-model="state.teamCode"
            center
            round
            clearable
            label=""
            :placeholder="$t('enterTeamCode')"
          />

          <van-button block round class="ghost-button !mt-[16px]" @click="handleSetTeamCode">
            {{ $t('confirm') }}
          </van-button>
        </div>

        <div v-else-if="store.userInfo" class="px-[16px] py-[12px]">
          <div class="bg-card mb-[12px] rounded-[8px]">
            <div class="flex items-center justify-between px-[12px] py-[16px] text-[13px]">
              <div class="font-bold">{{ $t('teamCode') }}</div>
              <div class="" @click="copy(store.userInfo.team_user_code)">
                {{ store.userInfo.team_user_code }}
                <img src="@/app/assets/images/copy.png" class="w-[12px]" />
              </div>
            </div>
            <div class="flex items-center justify-between px-[12px] py-[16px] text-[13px]">
              <div class="font-bold">{{ $t('teamSuperiorAddress') }}</div>
              <div class="">
                {{ formatWallet(store.userInfo.team_parent_address) }}
              </div>
            </div>
          </div>
          <div
            v-for="item in state.queryUser2.direct_under_user"
            :key="item.id"
            class="bg-card mb-[12px] rounded-[8px]"
          >
            <div class="flex items-center justify-between p-[12px] text-[13px]">
              <div class="font-bold">{{ $t('address') }}</div>
              <div class="" @click="copy(item.sol_address)">
                {{ formatWallet(item.sol_address) }}
                <img src="@/app/assets/images/copy.png" class="w-[12px]" />
              </div>
            </div>
            <div class="flex items-center justify-between p-[12px] text-[13px]">
              <div class="font-bold">{{ $t('level') }}</div>
              <div class="flex items-center gap-[4px]">
                <span>
                  {{ getLevelInfo(item.level).text }}
                </span>
                <img :src="getLevelInfo(item.level).icon" class="w-[14px]" />
              </div>
            </div>
            <div class="flex items-center justify-between p-[12px] text-[13px]">
              <div class="font-bold">{{ $t('power') }}</div>
              <div class="">{{ item.total_power }}</div>
            </div>
            <div class="flex items-center justify-between p-[12px] text-[13px]">
              <div class="font-bold">{{ $t('nftCount') }}</div>
              <div class="">{{ item.nft_count }}</div>
            </div>
          </div>
        </div>
      </van-tab>

      <van-tab :title="$t('globalPromotion')">
        <div
          v-if="!store.userInfo?.team_parent_code"
          class="bg-card m-[12px] rounded-[8px] p-[16px]"
        >
          <van-field
            v-model="state.teamCode"
            center
            round
            clearable
            label=""
            :placeholder="$t('enterTeamCode')"
          />

          <van-button block round class="ghost-button !mt-[16px]" @click="handleSetTeamCode">
            {{ $t('confirm') }}
          </van-button>
        </div>

        <div
          v-else-if="state.queryUser3"
          class="bg-card border-primary mx-[16px] my-[12px] overflow-hidden rounded-[12px] border-[1px]"
        >
          <div class="flex items-center p-[12px]">
            <div class="text-[13px] font-bold">{{ $t('myLevel') }}</div>
            <div class="flex flex-1 items-center justify-end gap-[8px]">
              <div class="text-[13px]">
                {{ getLevelInfo().text }}
              </div>
              <img :src="getLevelInfo().icon" class="w-[14px]" />
            </div>
          </div>
          <div class="flex items-center p-[12px]" @click="router.push('/user/dividend')">
            <div class="text-[13px] font-bold">{{ $t('totalDividendPool') }}</div>
            <div class="flex flex-1 items-center justify-end gap-[8px]">
              <div class="text-[13px]">{{ totalDividend }}</div>
              <van-icon name="arrow" />
            </div>
          </div>
          <div class="flex items-center p-[12px]">
            <div class="text-[13px] font-bold">{{ $t('globalLevelCount') }}</div>
          </div>
          <div class="text-primary flex items-center bg-black p-[12px]">
            <div class="text-[13px]">V0</div>
            <div class="flex flex-1 items-center justify-end gap-[8px]">
              <div class="text-[13px]">
                {{ state.queryUser3.total_level_0_num }}
              </div>
            </div>
          </div>
          <div class="text-primary flex items-center bg-black p-[12px]">
            <div class="text-[13px]">V1</div>
            <div class="flex flex-1 items-center justify-end gap-[8px]">
              <div class="text-[13px]">
                {{ state.queryUser3.total_level_1_num }}
              </div>
            </div>
          </div>
          <div class="text-primary flex items-center bg-black p-[12px]">
            <div class="text-[13px]">V2</div>
            <div class="flex flex-1 items-center justify-end gap-[8px]">
              <div class="text-[13px]">
                {{ state.queryUser3.total_level_2_num }}
              </div>
            </div>
          </div>
          <div class="text-primary flex items-center bg-black p-[12px]">
            <div class="text-[13px]">V3</div>
            <div class="flex flex-1 items-center justify-end gap-[8px]">
              <div class="text-[13px]">
                {{ state.queryUser3.total_level_3_num }}
              </div>
            </div>
          </div>
          <div class="text-primary flex items-center bg-black p-[12px]">
            <div class="text-[13px]">V4</div>
            <div class="flex flex-1 items-center justify-end gap-[8px]">
              <div class="text-[13px]">
                {{ state.queryUser3.total_level_4_num }}
              </div>
            </div>
          </div>
          <div class="text-primary flex items-center bg-black p-[12px]">
            <div class="text-[13px]">V5</div>
            <div class="flex flex-1 items-center justify-end gap-[8px]">
              <div class="text-[13px]">
                {{ state.queryUser3.total_level_5_num }}
              </div>
            </div>
          </div>
        </div>
      </van-tab>

      <van-tab v-if="store.userInfo?.white_state && state.queryUser3" :title="$t('whiteList')">
        <div class="bg-card mx-[16px] my-[12px] overflow-hidden rounded-[12px]">
          <div class="flex items-center p-[12px]">
            <div class="text-[13px] font-bold">{{ $t('globalPower') }}</div>
            <div class="flex flex-1 items-center justify-end gap-[8px]">
              <div class="text-[13px]">
                {{ state.queryUser3.total_user_power_amount }}
              </div>
            </div>
          </div>
          <div class="flex items-center p-[12px]">
            <div class="text-[13px] font-bold">{{ $t('globalNftCount') }}</div>
            <div class="flex flex-1 items-center justify-end gap-[8px]">
              <div class="text-[13px]">
                {{ state.queryUser3.total_user_nft_amount }}
              </div>
            </div>
          </div>
          <div class="flex items-center p-[12px]">
            <div class="text-[13px] font-bold">{{ $t('totalLp') }}</div>
            <div class="flex flex-1 items-center justify-end gap-[8px]">
              <div class="text-[13px]">
                {{ state.totalLp }}
              </div>
            </div>
          </div>
          <div class="flex items-center p-[12px]">
            <div class="text-[13px] font-bold">{{ $t('totalNodeAmount') }}</div>
            <div class="flex flex-1 items-center justify-end gap-[8px]">
              <div class="text-[13px]">
                {{ state.totalNodeAmount }}
              </div>
            </div>
          </div>
          <div class="flex items-center p-[12px]" @click="router.push('/user/crowdfunding/create')">
            <div class="text-[13px] font-bold">{{ $t('nodeManagement') }}</div>
            <div class="flex flex-1 items-center justify-end gap-[8px]">
              <van-icon name="arrow" />
            </div>
          </div>
          <div class="flex items-center p-[12px]" @click="handleNodeApply">
            <div class="text-[13px] font-bold">{{ $t('nodeApply') }}</div>
            <div class="flex flex-1 items-center justify-end gap-[8px]">
              <van-icon name="arrow" />
            </div>
          </div>
        </div>
      </van-tab>
    </van-tabs>
  </div>
  <BottomBar />
</template>

<script setup lang="ts">
import { useSessionStorage } from '@vueuse/core';
import { showToast } from 'vant';
import { computed, onBeforeMount, reactive } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';

defineOptions({
  name: 'User',
});

import {
  applyCrowdfundingApi,
  checkCrowdfundingApi,
  fetchDividendApi,
  fetchTokenPool,
  getApplicationDetailApi,
  queryUser2Api,
  queryUser3Api,
  setTeamCodeApi,
} from '@/app/api';
import { closeLoading, showLoading } from '@/app/components';
import { useClipboard } from '@/app/hooks';
import { useStore } from '@/app/store';
import { formatWallet } from '@/app/utils';
import {
  fetchCrowdfundingInfo,
  fetchSalePhases,
  fetchStakingPools,
  fetchVestingInfo,
  getBioneoBalance,
  movePointLeft,
} from '@/web3';

const store = useStore();
const { copy } = useClipboard();
const { t } = useI18n();
const router = useRouter();

const state = reactive({
  receivedReward: 0,
  pendingReward: 0,
  lpBalance: 0,
  bioneoBalance: '0',
  crowdfundingEligibility: false,
  phaseId: null,
  crowdfundingStatus: null,
  queryUser2: {
    direct_under_user: [],
  },
  queryUser3: null,
  dividend: null,
  tokenPool: null,
  teamCode: '',
  totalLp: 0,
  totalNodeAmount: 0,
  activeTab: 0,
});

const hasVesting = useSessionStorage('hasVesting', false);
const getLevelInfo = (level?) => {
  const userLevel = level ?? store.userInfo?.level;

  const map = {
    0: {
      text: t('level0'),
      icon: require('./images/V0.png'),
      nextLevelDesc: t('nextLevelDesc0'),
    },
    1: {
      text: t('level1'),
      icon: require('./images/V1.png'),
      nextLevelDesc: t('nextLevelDesc1'),
    },
    2: {
      text: t('level2'),
      icon: require('./images/V2.png'),
      nextLevelDesc: t('nextLevelDesc2'),
    },
    3: {
      text: t('level3'),
      icon: require('./images/V3.png'),
      nextLevelDesc: t('nextLevelDesc3'),
    },
    4: {
      text: t('level4'),
      icon: require('./images/V4.png'),
      nextLevelDesc: t('nextLevelDesc4'),
    },
    5: {
      text: t('level5'),
      icon: require('./images/V5.png'),
    },
  };
  return (
    map[userLevel] || {
      text: t('levelNone'),
      icon: require('./images/V0.png'),
      nextLevelDesc: t('levelNoneDesc'),
    }
  );
};

const handleNodeApply = async () => {
  router.push('/user/crowdfunding/apply');
};

const handleHasVesting = async () => {
  const res = await fetchVestingInfo();
  hasVesting.value = res.some(
    (item) => item.beneficiary.toLowerCase() === store.walletAddress.toLowerCase(),
  );
  console.log('🚀 ~ handleHasVesting ~ state.hasVesting:', hasVesting.value);
};

onBeforeMount(async () => {
  await store.connectWallet();
  fetchData();
});

const totalDividend = computed(() => {
  if (!state.dividend || !state.tokenPool) {
    return 0;
  }

  return (
    state.dividend.lp_withdraw_bio_month +
    state.dividend.vip_buy_bio_month +
    state.dividend.node_withdraw_bio_month +
    state.dividend.vip_withdraw_bio_month +
    9625
  ).toFixed(9);
});

const queryUserData = async () => {
  const [res, res2, res3, res4] = await Promise.all([
    fetchDividendApi({
      sol_address: store.walletAddress,
    }),
    queryUser2Api({
      sol_address: store.walletAddress,
    }),
    queryUser3Api({
      sol_address: store.walletAddress,
    }),
    fetchTokenPool({
      sol_address: store.walletAddress,
    }),
  ]);

  state.dividend = res.data;
  state.queryUser2 = res2.data;
  state.queryUser3 = res3.data;
  state.tokenPool = res4.data;

  console.log('🚀 ~ queryUserData ~ res:', res2.data, res3.data);
};

const loadStakingInfo = async () => {
  if (!store.userInfo?.white_state) {
    return;
  }

  const [res, res2] = await Promise.all([fetchStakingPools(), fetchSalePhases()]);

  const salePhases = res2;
  state.totalNodeAmount = salePhases.reduce((acc, item) => acc + Number(item.soldShares), 0);
  const totalLp = res.pools.reduce((acc, item) => acc + Number(item.totalShares), 0);
  state.totalLp = movePointLeft(totalLp, 9);
  console.log('🚀 ~ loadStakingInfo ~ res:', res, totalLp);
};

const fetchData = async () => {
  queryUserData();
  const [bioneoBalance] = await Promise.all([
    getBioneoBalance(),
    checkCrowdfunding(),
    getApplicationDetail(),
    handleHasVesting(),
    loadStakingInfo(),
  ]);

  state.bioneoBalance = bioneoBalance.toFixed(9);
};

const checkCrowdfunding = async () => {
  try {
    const crowdfundingInfo = await fetchCrowdfundingInfo();
    state.phaseId = crowdfundingInfo.phaseCount;

    const res = await checkCrowdfundingApi({
      sol_address: store.walletAddress,
      phase_id: crowdfundingInfo.phaseCount,
    });
    state.crowdfundingEligibility = res.data;
  } catch (error) {
    console.log('🚀 ~ checkCrowdfunding ~ error:', error);
  }
};

const handleApplyCrowdfunding = async () => {
  try {
    showLoading();
    await applyCrowdfundingApi({
      sol_address: store.walletAddress,
      phase_id: state.phaseId,
    });
    await getApplicationDetail();
    showToast(t('success'));
  } catch (error) {
    console.log('🚀 ~ handleApplyCrowdfunding ~ error:', error);
  } finally {
    closeLoading();
  }
};

const getApplicationDetail = async () => {
  const res = await getApplicationDetailApi({
    sol_address: store.walletAddress,
  });
  const find = res.data.find((item) => item.phase_id === state.phaseId);
  state.crowdfundingStatus = find?.status;
};

const getCrowdfundingStatus = (status) => {
  console.log('🚀 ~ getCrowdfundingStatus ~ status:', status);
  const statusMap = {
    0: t('pending'),
    1: t('approved'),
    2: t('rejected'),
  };

  if (status === undefined) {
    return state.crowdfundingEligibility ? t('eligible') : t('notEligible');
  }

  return statusMap[status];
};

const handleSetTeamCode = async () => {
  try {
    showLoading();
    await setTeamCodeApi({ team_parent_code: state.teamCode, sol_address: store.walletAddress });
    await queryUserData();
    await store.fetchUserInfo();
    showToast(t('success'));
    closeLoading();
  } catch (error) {
    console.log('🚀 ~ handleSetTeamCode ~ error:', error);
  } finally {
    closeLoading();
  }
};
</script>

<style lang="less" scoped>
.user-fields {
  z-index: 1;
  padding: 0;
  margin: 12px 16px;
  overflow: hidden;
  color: black;
  border: 1px solid #ffffff0d;
  border-radius: 12px;

  .field-item {
    display: flex;
    gap: 6px;
    align-items: center;
    padding: 15px 12px;

    img {
      width: 16px;
    }

    .copy-icon {
      width: 14px;
    }

    &.item-title {
      font-size: 13px;
      font-weight: bold;
      color: #001f4dcc;
    }

    &.sub-item {
      padding: 9px 24px;
      background: rgb(255 255 255 / 90%);
    }

    &:last-child {
      border-bottom: none;
    }

    .item-label {
      flex: 1;
      font-size: 13px;
      font-weight: bold;
    }

    .item-value {
      font-size: 13px;
      color: var(--van-black);
    }
  }
}

.van-field {
  border-radius: 100px;
}
</style>
