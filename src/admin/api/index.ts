import request from '@/admin/utils/request';

export function loginApi(data) {
  return request({
    url: '/manage/login',
    method: 'post',
    data,
  });
}

export function fetchUserInfoApi() {
  return request({
    url: '/mock/user/info',
    method: 'get',
  });
}

export function fetchUserListApi(data) {
  return request({
    url: '/manage/userlist',
    method: 'post',
    data,
  });
}

export function updatePasswordApi(data) {
  return request({
    url: '/manage/updatepassword',
    method: 'post',
    data,
  });
}

export function fetchCrowdfundingApplyListApi(data) {
  return request({
    url: '/manage/applicationlist',
    method: 'post',
    data,
  });
}

export function reviewCrowdfundingApplyApi(data) {
  return request({
    url: '/manage/applications/review',
    method: 'post',
    data,
  });
}

export function fetchUserPurchaseListApi(data) {
  return request({
    url: '/manage/userpurchaselist',
    method: 'post',
    data,
  });
}
