<template>
  <div class="basic-table-page">
    <a-card>
      <a-form layout="inline" :model="searchParams" class="basic-table-form" @finish="handleSearch">
        <a-form-item label="钱包地址" name="sol_address">
          <a-input
            v-model:value="searchParams.sol_address"
            allow-clear
            placeholder="请输入钱包地址"
          />
        </a-form-item>

        <a-form-item>
          <a-button type="primary" html-type="submit">
            <template #icon>
              <SearchOutlined />
            </template>
            搜索
          </a-button>
        </a-form-item>
      </a-form>
    </a-card>

    <a-card>
      <a-table v-bind="tableProps"></a-table>
    </a-card>

    <UserInfoModal ref="userInfoModalRef" />
  </div>
</template>

<script setup lang="ts">
import { SearchOutlined } from '@ant-design/icons-vue';

import { fetchUserListApi } from '@/admin/api';
import { useTable } from '@/admin/hooks';

import UserInfoModal from './UserInfoModal.vue';

const columns = [
  {
    title: 'ID',
    dataIndex: 'id',
    sorter: true,
    width: 100,
  },
  {
    title: '钱包地址',
    dataIndex: 'sol_address',
    width: 200,
  },
  {
    title: 'NFT数量',
    dataIndex: 'nft_count',
    width: 150,
  },
  {
    title: '邀请码',
    dataIndex: 'user_code',
    isLink: true,
    width: 200,
  },
  {
    title: '上级地址',
    dataIndex: 'parent_address',
    width: 200,
  },
  {
    title: '创建时间',
    dataIndex: 'create_time',
    width: 200,
  },
];

const { tableProps, searchParams, handleSearch } = useTable({
  columns,
  api: fetchUserListApi,
  defaultSearchParams: {},
});
</script>
