import {
  CalendarOutlined,
  DashboardOutlined,
  SettingOutlined,
  SmileOutlined,
  UserOutlined,
} from '@ant-design/icons-vue';
import { RouteRecordRaw, RouterView } from 'vue-router';

import { RoleName } from '@/admin/enums';
import { i18n } from '@/admin/locales';

const routes = [
  {
    path: '/user',
    component: require('@/admin/layouts/UserLayout.vue').default,
    hideInMenu: true,
    children: [
      {
        path: '/user/login',
        name: 'login',
        component: () => import('@/admin/views/login/Login.vue'),
      },
    ],
  },
  {
    path: '/',
    component: require('@/admin/layouts/BasicLayout.vue').default,
    meta: { roles: [RoleName.Admin, RoleName.User] },
    children: [
      {
        path: '/',
        redirect: '/userManagement/userList',
      },
      {
        path: '/dashboard',
        name: 'dashboard',
        meta: {
          icon: DashboardOutlined,
          get title() {
            return i18n.global.t('dashboard');
          },
        },
        hideInMenu: true,
        component: RouterView,
        children: [
          {
            path: '/dashboard/analysis',
            name: 'analysis',
            meta: { title: '分析' },
            component: () => import('@/admin/views/dashboard/Analysis.vue'),
          },
        ],
      },
      {
        path: '/userManagement',
        name: 'userManagement',
        meta: { icon: UserOutlined, title: '用户管理' },
        component: RouterView,
        children: [
          {
            path: '/userManagement/userList',
            name: 'userList',
            meta: { title: '用户列表' },
            component: () => import('@/admin/views/userManagement/UserList.vue'),
          },
        ],
      },
      {
        path: '/crowdfunding',
        name: 'crowdfunding',
        meta: { icon: SmileOutlined, title: '节点管理' },
        component: RouterView,
        children: [
          {
            path: '/crowdfunding/applyList',
            name: 'crowdfundingApplyList',
            meta: { title: '申请列表' },
            component: () => import('@/admin/views/crowdfunding/CrowdfundingApplyList.vue'),
          },
          {
            path: '/crowdfunding/purchaseList',
            name: 'crowdfundingPurchaseList',
            meta: { title: '购买列表' },
            component: () => import('@/admin/views/crowdfunding/CrowdfundingPurchaseList.vue'),
          },
        ],
      },
      {
        path: '/profile',
        name: 'profile',
        meta: {
          icon: SettingOutlined,
          title: 'profile',
        },
        component: () => import('@/admin/views/profile/Profile.vue'),
        hideInMenu: true,
      },
      {
        path: '/features',
        name: 'features',
        meta: { icon: CalendarOutlined, title: '功能' },
        hideInMenu: true,
        component: RouterView,
        children: [
          {
            path: '/features/icon',
            name: 'icon',
            meta: { title: '图标', icon: SmileOutlined },
            component: () => import('@/admin/views/features/Icon.vue'),
          },
        ],
      },
      {
        path: '/404',
        name: '404',
        hideInMenu: true,
        component: () => import('@/admin/views/404.vue'),
      },
      {
        path: ':pathMatch(.*)',
        redirect: '/404',
      },
    ],
  },
] as RouteRecordRaw[];

export default routes;
