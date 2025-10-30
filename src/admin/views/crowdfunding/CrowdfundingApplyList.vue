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
      <a-table v-bind="tableProps">
        <template #bodyCell="{ column, record }">
          <template v-if="column.dataIndex === 'purchase_link'">
            <a @click="copy('https://www.bioneo.top/#/purchase?link=' + record.purchase_link)">
              {{ record.purchase_link }}
            </a>
          </template>
          <template v-if="column.dataIndex === 'status'">
            <a-tag
              v-if="record.status !== null"
              :color="record.status === 0 ? 'blue' : record.status === 1 ? 'green' : 'red'"
            >
              {{ record.status === 0 ? '待审核' : record.status === 1 ? '已通过' : '已拒绝' }}
            </a-tag>
          </template>
          <template v-if="column.dataIndex === 'action'">
            <a-button
              type="primary"
              size="small"
              :disabled="record.status !== 0"
              @click="handleApprove(record)"
            >
              通过
            </a-button>
            <a-button
              type="primary"
              danger
              size="small"
              :disabled="record.status !== 0"
              style="margin-left: 10px"
              @click="handleReject(record)"
            >
              拒绝
            </a-button>
          </template>
        </template>
      </a-table>
    </a-card>
  </div>
</template>

<script setup lang="ts">
import { SearchOutlined } from '@ant-design/icons-vue';

import { fetchCrowdfundingApplyListApi, reviewCrowdfundingApplyApi } from '@/admin/api';
import { useClipboard, useTable } from '@/admin/hooks';

const { copy } = useClipboard();

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
    title: '期数',
    dataIndex: 'phase_id',
    width: 150,
  },
  {
    title: '购买链接',
    dataIndex: 'purchase_link',
    width: 200,
    ellipsis: true,
  },
  {
    title: '状态',
    dataIndex: 'status',

    width: 200,
  },
  {
    title: '创建时间',
    dataIndex: 'created_time',
    width: 200,
  },
  {
    title: '操作',
    dataIndex: 'action',
    width: 150,
    fixed: 'right',
  },
];

const { tableProps, searchParams, handleSearch } = useTable({
  columns,
  api: fetchCrowdfundingApplyListApi,
  defaultSearchParams: {},
});

const handleApprove = async (record) => {
  await reviewCrowdfundingApplyApi({ app_id: record.id, approve: true });
  handleSearch();
};

const handleReject = async (record) => {
  await reviewCrowdfundingApplyApi({ app_id: record.id, approve: false });
  handleSearch();
};
</script>
