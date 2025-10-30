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
  </div>
</template>

<script setup lang="ts">
import { SearchOutlined } from '@ant-design/icons-vue';
import dayjs from 'dayjs';

import { fetchUserPurchaseListApi } from '@/admin/api';
import { useTable } from '@/admin/hooks';

const columns = [
  {
    title: 'ID',
    dataIndex: 'id',
    sorter: true,
    width: 100,
  },
  {
    title: '钱包地址',
    dataIndex: 'user',
    isWallet: true,
    width: 200,
  },
  {
    title: '上级地址',
    dataIndex: 'superior_address',
    isWallet: true,
    width: 200,
  },
  {
    title: '期数',
    dataIndex: 'phase_id',
    width: 150,
  },
  {
    title: '购买份数',
    dataIndex: 'shares',
    width: 150,
    ellipsis: true,
  },
  {
    title: '代币数量',
    dataIndex: 'token_amount',
    customRender: ({ text }) => text / 10 ** 9,
    width: 150,
  },
  {
    title: '已领取代币',
    dataIndex: 'claimed_amount',
    customRender: ({ text }) => text / 10 ** 9,
    width: 150,
  },
  {
    title: '购买时间',
    dataIndex: 'purchase_time',
    width: 200,
    customRender: ({ text }) => dayjs(text).format('YYYY-MM-DD HH:mm:ss'),
  },
];

const { tableProps, searchParams, handleSearch } = useTable({
  columns,
  api: fetchUserPurchaseListApi,
  defaultSearchParams: {},
});
</script>
