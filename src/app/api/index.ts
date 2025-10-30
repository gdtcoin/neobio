import request from '@/app/utils/request';

export function fetchCrowdfundingApplyListApi(data) {
  return request({
    url: '/v1/applicationlist',
    method: 'post',
    data,
  });
}

export function reviewCrowdfundingApplyApi(data) {
  return request({
    url: '/v1/applications/review',
    method: 'post',
    data,
  });
}

export const loginApi = (data) => {
  return request.post('/v1/logins', data);
};

export const bindApi = (data) => {
  return request.post('/v1/invite', data);
};

export const checkCrowdfundingApi = (data) => {
  // @ts-ignore
  return request({
    url: '/v1/check/eligibility',
    method: 'POST',
    data,
    loading: false,
    toast: false,
  });
};

export const applyCrowdfundingApi = (data) => {
  return request({
    url: '/v1/apply',
    method: 'POST',
    data,
  });
};

export const getApplicationDetailApi = (data) => {
  return request({
    url: '/v1/application/detail',
    method: 'POST',
    data,
  });
};

export const applyCrowdfunding = (data) => {
  return request({
    url: '/v1/order/create',
    method: 'POST',
    data,
  });
};

export const signMintNftApi = (data) => {
  return request({
    url: '/v1/mintnft/sign',
    method: 'POST',
    data,
  });
};

export const addStakeApi = (data) => {
  return request({
    url: '/v1/addstaking/sign',
    method: 'POST',
    data,
  });
};

export const getMintNftApi = (data) => {
  return request({
    url: '/v1/mintnft',
    method: 'POST',
    data,
  });
};

export const getGdtcPriceApi = (data?) => {
  return request({
    url: '/v1/price',
    method: 'POST',
    data,
  });
};

export const queryUser2Api = (data) => {
  return request({
    url: '/v1/queryuser_v2',
    method: 'POST',
    data,
  });
};

export const queryUser3Api = (data) => {
  return request({
    url: '/v1/queryuser_v3',
    method: 'POST',
    data,
  });
};

export const setTeamCodeApi = (data) => {
  return request({
    url: '/v1/teaminvite',
    method: 'POST',
    data,
  });
};

// business_type(1-节点提币,2-购买会员制,3-会员提币,4-LP提币
export const addDividendApi = (data) => {
  // @ts-ignore
  return request({
    url: '/v1/dividend/add',
    method: 'POST',
    toast: false,
    data,
  });
};

export const fetchDividendApi = (data) => {
  return request({
    url: '/v1/dividend/summary',
    method: 'POST',
    data,
  });
};

export const fetchTokenPool = (data) => {
  return request({
    url: '/v1/tokenpool_transfer_summary',
    method: 'POST',
    data,
  });
};
