<template>
  <el-dialog
    v-model="visible"
    :title="$t('dialog.analytics.title')"
    width="90%"
    class="analytics-dialog"
    :close-on-click-modal="false"
    @close="handleClose"
  >
    <div v-loading="loading" class="analytics-container">
      <!-- 顶部统计卡片 -->
      <div v-if="!loading && analyticsData" class="stats-cards">
        <div class="stat-card blue-gradient">
          <div class="stat-icon">
            <el-icon><Document /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-label">{{ $t('dialog.analytics.totalCodeLines') }}</div>
            <div class="stat-value">{{ formatNumber(analyticsData.summary.total_accepted_lines) }}</div>
          </div>
        </div>

        <div class="stat-card green-gradient">
          <div class="stat-icon">
            <el-icon><TrendCharts /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-label">{{ $t('dialog.analytics.avgDailyCodeLines') }}</div>
            <div class="stat-value">{{ analyticsData.summary.avg_daily_accepted_lines.toFixed(1) }}</div>
          </div>
        </div>

        <div class="stat-card orange-gradient">
          <div class="stat-icon">
            <el-icon><ChatLineRound /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-label">{{ $t('dialog.analytics.totalSessions') }}</div>
            <div class="stat-value">{{ analyticsData.summary.total_sessions }}</div>
          </div>
        </div>

        <div class="stat-card purple-gradient">
          <div class="stat-icon">
            <el-icon><Coin /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-label">{{ $t('dialog.analytics.totalTokenConsumption') }}</div>
            <div class="stat-value">{{ formatNumber(analyticsData.summary.total_tokens / 100) }}</div>
          </div>
        </div>

        <div class="stat-card teal-gradient">
          <div class="stat-icon">
            <el-icon><Cpu /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-label">{{ $t('dialog.analytics.primaryModel') }}</div>
            <div class="stat-value small" :title="analyticsData.summary.primary_model">{{ formatModelName(analyticsData.summary.primary_model) }}</div>
          </div>
        </div>

        <div class="stat-card red-gradient">
          <div class="stat-icon">
            <el-icon><Tools /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-label">{{ $t('dialog.analytics.primaryTool') }}</div>
            <div class="stat-value small" :title="analyticsData.summary.primary_tool">{{ formatToolName(analyticsData.summary.primary_tool) }}</div>
          </div>
        </div>
      </div>

      <!-- 新增：AI 贡献统计卡片 -->
      <div v-if="!loading && analyticsData && hasPercentCodeWritten" class="ai-contribution-section">
        <h3 class="section-title">
          <el-icon><Cpu /></el-icon>
          {{ $t('dialog.analytics.aiCodeContribution') }}
        </h3>
        <div class="ai-contribution-cards">
          <div class="contribution-card ai-percent">
            <div class="contribution-ring">
              <svg viewBox="0 0 100 100">
                <circle class="bg" cx="50" cy="50" r="40" />
                <circle class="progress" cx="50" cy="50" r="40" 
                  :style="{ strokeDasharray: `${aiContributionPercent * 2.51} 251` }" />
              </svg>
              <div class="percent-text">{{ aiContributionPercent.toFixed(1) }}%</div>
            </div>
            <div class="contribution-label">{{ $t('dialog.analytics.aiCodePercentage') }}</div>
          </div>
          
          <div class="contribution-breakdown">
            <div class="breakdown-item">
              <div class="breakdown-bar autocomplete" :style="{ width: autocompletePercent + '%' }"></div>
              <div class="breakdown-info">
                <span class="breakdown-name">{{ $t('dialog.analytics.autocomplete') }}</span>
                <span class="breakdown-value">{{ formatBytes(analyticsData.percent_code_written.codeium_bytes_by_autocomplete) }}</span>
              </div>
            </div>
            <div class="breakdown-item">
              <div class="breakdown-bar cascade" :style="{ width: cascadePercent + '%' }"></div>
              <div class="breakdown-info">
                <span class="breakdown-name">Cascade</span>
                <span class="breakdown-value">{{ formatBytes(analyticsData.percent_code_written.codeium_bytes_by_cascade) }}</span>
              </div>
            </div>
            <div class="breakdown-item">
              <div class="breakdown-bar command" :style="{ width: commandPercent + '%' }"></div>
              <div class="breakdown-info">
                <span class="breakdown-name">{{ $t('dialog.analytics.command') }}</span>
                <span class="breakdown-value">{{ formatBytes(analyticsData.percent_code_written.codeium_bytes_by_command) }}</span>
              </div>
            </div>
            <div class="breakdown-item">
              <div class="breakdown-bar user" :style="{ width: userPercent + '%' }"></div>
              <div class="breakdown-info">
                <span class="breakdown-name">{{ $t('dialog.analytics.userWritten') }}</span>
                <span class="breakdown-value">{{ formatBytes(analyticsData.percent_code_written.user_bytes) }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 新增：补全统计卡片 -->
      <div v-if="!loading && analyticsData && hasCompletionStats" class="completion-stats-section">
        <h3 class="section-title">
          <el-icon><Finished /></el-icon>
          {{ $t('dialog.analytics.completionStats') }}
        </h3>
        <div class="completion-stats-cards">
          <div class="mini-stat-card">
            <div class="mini-stat-value success">{{ formatNumber(analyticsData.completion_stats.num_acceptances) }}</div>
            <div class="mini-stat-label">{{ $t('dialog.analytics.acceptanceCount') }}</div>
          </div>
          <div class="mini-stat-card">
            <div class="mini-stat-value warning">{{ formatNumber(analyticsData.completion_stats.num_rejections) }}</div>
            <div class="mini-stat-label">{{ $t('dialog.analytics.rejectionCount') }}</div>
          </div>
          <div class="mini-stat-card">
            <div class="mini-stat-value primary">{{ analyticsData.completion_stats.acceptance_rate.toFixed(1) }}%</div>
            <div class="mini-stat-label">{{ $t('dialog.analytics.acceptanceRate') }}</div>
          </div>
          <div class="mini-stat-card">
            <div class="mini-stat-value info">{{ formatNumber(analyticsData.completion_stats.num_lines_accepted) }}</div>
            <div class="mini-stat-label">{{ $t('dialog.analytics.acceptedLines') }}</div>
          </div>
          <div class="mini-stat-card">
            <div class="mini-stat-value">{{ analyticsData.completion_stats.active_developer_days }}</div>
            <div class="mini-stat-label">{{ $t('dialog.analytics.activeDays') }}</div>
          </div>
        </div>
      </div>

      <!-- 新增：Chat 统计卡片 -->
      <div v-if="!loading && analyticsData && hasChatStats" class="chat-stats-section">
        <h3 class="section-title">
          <el-icon><ChatLineRound /></el-icon>
          {{ $t('dialog.analytics.chatStats') }}
        </h3>
        <div class="chat-stats-cards">
          <div class="mini-stat-card">
            <div class="mini-stat-value primary">{{ formatNumber(analyticsData.chat_stats.chats_sent) }}</div>
            <div class="mini-stat-label">{{ $t('dialog.analytics.messagesSent') }}</div>
          </div>
          <div class="mini-stat-card">
            <div class="mini-stat-value success">{{ formatNumber(analyticsData.chat_stats.chats_accepted) }}</div>
            <div class="mini-stat-label">{{ $t('dialog.analytics.suggestionsAdopted') }}</div>
          </div>
          <div class="mini-stat-card">
            <div class="mini-stat-value info">{{ formatNumber(analyticsData.chat_stats.chat_code_blocks_used) }}</div>
            <div class="mini-stat-label">{{ $t('dialog.analytics.codeBlocksUsed') }}</div>
          </div>
          <div class="mini-stat-card">
            <div class="mini-stat-value warning">{{ formatNumber(analyticsData.chat_stats.function_explain_count) }}</div>
            <div class="mini-stat-label">{{ $t('dialog.analytics.functionExplain') }}</div>
          </div>
          <div class="mini-stat-card">
            <div class="mini-stat-value">{{ formatNumber(analyticsData.chat_stats.function_refactor_count) }}</div>
            <div class="mini-stat-label">{{ $t('dialog.analytics.functionRefactor') }}</div>
          </div>
          <div class="mini-stat-card">
            <div class="mini-stat-value danger">{{ formatNumber(analyticsData.chat_stats.function_unit_tests_count) }}</div>
            <div class="mini-stat-label">{{ $t('dialog.analytics.unitTests') }}</div>
          </div>
        </div>
      </div>

      <!-- 图表区域 -->
      <div v-if="!loading && analyticsData" class="charts-container">
        <!-- 每日代码行数趋势 -->
        <el-card class="chart-card" shadow="hover">
          <template #header>
            <div class="card-header">
              <el-icon class="header-icon"><TrendCharts /></el-icon>
              <span>每日代码行数趋势</span>
            </div>
          </template>
          <div ref="dailyChartRef" class="chart"></div>
        </el-card>

        <!-- 工具使用分布 -->
        <el-card class="chart-card" shadow="hover">
          <template #header>
            <div class="card-header">
              <el-icon class="header-icon"><Tools /></el-icon>
              <span>工具使用分布</span>
            </div>
          </template>
          <div ref="toolChartRef" class="chart"></div>
        </el-card>

        <!-- 模型使用分布 -->
        <el-card class="chart-card" shadow="hover">
          <template #header>
            <div class="card-header">
              <el-icon class="header-icon"><Cpu /></el-icon>
              <span>模型使用分布</span>
            </div>
          </template>
          <div ref="modelChartRef" class="chart"></div>
        </el-card>

        <!-- Token消耗趋势 -->
        <el-card class="chart-card" shadow="hover">
          <template #header>
            <div class="card-header">
              <el-icon class="header-icon"><Coin /></el-icon>
              <span>Token消耗趋势</span>
            </div>
          </template>
          <div ref="tokenChartRef" class="chart"></div>
        </el-card>

        <!-- 按语言的补全统计 -->
        <el-card v-if="analyticsData.completions_by_language?.length > 0" class="chart-card" shadow="hover">
          <template #header>
            <div class="card-header">
              <el-icon class="header-icon"><Document /></el-icon>
              <span>按语言补全统计</span>
            </div>
          </template>
          <div ref="languageChartRef" class="chart"></div>
        </el-card>

        <!-- 按模型的Chat统计 -->
        <el-card v-if="analyticsData.chats_by_model?.length > 0" class="chart-card" shadow="hover">
          <template #header>
            <div class="card-header">
              <el-icon class="header-icon"><ChatLineRound /></el-icon>
              <span>按模型Chat统计</span>
            </div>
          </template>
          <div ref="chatModelChartRef" class="chart"></div>
        </el-card>

        <!-- 每日补全趋势 -->
        <el-card v-if="analyticsData.completions_by_day?.length > 0" class="chart-card" shadow="hover">
          <template #header>
            <div class="card-header">
              <el-icon class="header-icon"><Finished /></el-icon>
              <span>每日补全趋势</span>
            </div>
          </template>
          <div ref="completionsByDayChartRef" class="chart"></div>
        </el-card>

        <!-- 每日Chat趋势 -->
        <el-card v-if="analyticsData.chats_by_day?.length > 0" class="chart-card" shadow="hover">
          <template #header>
            <div class="card-header">
              <el-icon class="header-icon"><ChatLineRound /></el-icon>
              <span>每日Chat趋势</span>
            </div>
          </template>
          <div ref="chatsByDayChartRef" class="chart"></div>
        </el-card>
      </div>

      <!-- 空状态 -->
      <el-empty v-if="!loading && !analyticsData" description="暂无数据" />
    </div>

    <template #footer>
      <el-button @click="handleClose">关闭</el-button>
      <el-button type="primary" @click="handleRefresh" :loading="loading">
        <el-icon class="el-icon--left"><Refresh /></el-icon>刷新
      </el-button>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, watch, nextTick, computed } from 'vue';
import { ElMessage } from 'element-plus';
import { 
  Document, 
  TrendCharts, 
  ChatLineRound, 
  Coin, 
  Cpu, 
  Tools, 
  Refresh,
  Finished
} from '@element-plus/icons-vue';
import * as echarts from 'echarts';
import type { ECharts } from 'echarts';
import { analyticsApi } from '@/api';
import type { AnalyticsData } from '@/types/analytics';

interface Props {
  modelValue: boolean;
  accountId: string;
  accountEmail: string;
}

const props = defineProps<Props>();
const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void;
}>();

const visible = ref(false);
const loading = ref(false);
const analyticsData = ref<AnalyticsData | null>(null);

// ===== 新增计算属性 =====

// 是否有代码贡献百分比数据
const hasPercentCodeWritten = computed(() => {
  const pcw = analyticsData.value?.percent_code_written;
  return pcw && pcw.total_bytes && pcw.total_bytes > 0;
});

// 是否有补全统计数据
const hasCompletionStats = computed(() => {
  const stats = analyticsData.value?.completion_stats;
  return stats && (stats.num_acceptances > 0 || stats.num_rejections > 0);
});

// 是否有 Chat 统计数据
const hasChatStats = computed(() => {
  const stats = analyticsData.value?.chat_stats;
  return stats && stats.chats_sent > 0;
});

// AI 代码贡献百分比 (API 返回的是小数形式 0.9999，需要乘以 100)
const aiContributionPercent = computed(() => {
  const raw = analyticsData.value?.percent_code_written?.percent_code_written || 0;
  return raw * 100;
});

// 自动补全占比
const autocompletePercent = computed(() => {
  const pcw = analyticsData.value?.percent_code_written;
  if (!pcw || pcw.total_bytes === 0) return 0;
  return (pcw.codeium_bytes_by_autocomplete / pcw.total_bytes) * 100;
});

// Cascade 占比
const cascadePercent = computed(() => {
  const pcw = analyticsData.value?.percent_code_written;
  if (!pcw || pcw.total_bytes === 0) return 0;
  return (pcw.codeium_bytes_by_cascade / pcw.total_bytes) * 100;
});

// 命令占比
const commandPercent = computed(() => {
  const pcw = analyticsData.value?.percent_code_written;
  if (!pcw || pcw.total_bytes === 0) return 0;
  return (pcw.codeium_bytes_by_command / pcw.total_bytes) * 100;
});

// 用户编写占比
const userPercent = computed(() => {
  const pcw = analyticsData.value?.percent_code_written;
  if (!pcw || pcw.total_bytes === 0) return 0;
  return (pcw.user_bytes / pcw.total_bytes) * 100;
});

// 图表引用
const dailyChartRef = ref<HTMLElement>();
const toolChartRef = ref<HTMLElement>();
const modelChartRef = ref<HTMLElement>();
const tokenChartRef = ref<HTMLElement>();
const languageChartRef = ref<HTMLElement>();
const chatModelChartRef = ref<HTMLElement>();
const completionsByDayChartRef = ref<HTMLElement>();
const chatsByDayChartRef = ref<HTMLElement>();

// 图表实例
let dailyChart: ECharts | null = null;
let toolChart: ECharts | null = null;
let modelChart: ECharts | null = null;
let tokenChart: ECharts | null = null;
let languageChart: ECharts | null = null;
let chatModelChart: ECharts | null = null;
let completionsByDayChart: ECharts | null = null;
let chatsByDayChart: ECharts | null = null;

// 监听 modelValue 变化
watch(() => props.modelValue, (val) => {
  visible.value = val;
  if (val) {
    loadAnalytics();
  }
});

// 监听 visible 变化
watch(visible, (val) => {
  emit('update:modelValue', val);
  if (!val) {
    destroyCharts();
  }
});

// 加载分析数据
const loadAnalytics = async () => {
  loading.value = true;
  try {
    analyticsData.value = await analyticsApi.getAccountAnalytics(props.accountId);
    console.log('[AnalyticsDialog] Received analytics data:', analyticsData.value);
    console.log('[AnalyticsDialog] Daily cascade lines:', analyticsData.value?.daily_cascade_lines);
    console.log('[AnalyticsDialog] Tool usage:', analyticsData.value?.tool_usage);
    console.log('[AnalyticsDialog] Model usage summary:', analyticsData.value?.model_usage_summary);
    console.log('[AnalyticsDialog] Model usage details:', analyticsData.value?.model_usage_details);
    console.log('[AnalyticsDialog] Summary:', analyticsData.value?.summary);
    await nextTick();
    initCharts();
  } catch (error: any) {
    console.error('[AnalyticsDialog] Error loading analytics:', error);
    ElMessage.error(error || '加载分析数据失败');
  } finally {
    loading.value = false;
  }
};

// 初始化所有图表
const initCharts = () => {
  if (!analyticsData.value) {
    console.error('[initCharts] No analytics data available');
    return;
  }

  console.log('[initCharts] Starting chart initialization');
  console.log('[initCharts] Data arrays lengths:', {
    daily_cascade_lines: analyticsData.value.daily_cascade_lines?.length || 0,
    tool_usage: analyticsData.value.tool_usage?.length || 0,
    model_usage_summary: analyticsData.value.model_usage_summary?.length || 0,
    model_usage_details: analyticsData.value.model_usage_details?.length || 0
  });

  // 使用 setTimeout 确保 DOM 已完全渲染
  setTimeout(() => {
    console.log('[initCharts] DOM refs status:', {
      dailyChartRef: !!dailyChartRef.value,
      toolChartRef: !!toolChartRef.value,
      modelChartRef: !!modelChartRef.value,
      tokenChartRef: !!tokenChartRef.value,
      languageChartRef: !!languageChartRef.value,
      chatModelChartRef: !!chatModelChartRef.value,
      completionsByDayChartRef: !!completionsByDayChartRef.value,
      chatsByDayChartRef: !!chatsByDayChartRef.value
    });

    initDailyChart();
    initToolChart();
    initModelChart();
    initTokenChart();
    initLanguageChart();
    initChatModelChart();
    initCompletionsByDayChart();
    initChatsByDayChart();
  }, 100);
};

// 初始化每日代码行数趋势图
const initDailyChart = () => {
  console.log('[initDailyChart] Starting initialization');

  if (!dailyChartRef.value) {
    console.error('[initDailyChart] Chart ref not found!');
    return;
  }

  if (!analyticsData.value) {
    console.error('[initDailyChart] No analytics data!');
    return;
  }

  if (!analyticsData.value.daily_cascade_lines || analyticsData.value.daily_cascade_lines.length === 0) {
    console.warn('[initDailyChart] No cascade lines data available');
    return;
  }

  console.log('[initDailyChart] Cascade lines data:', analyticsData.value.daily_cascade_lines);

  try {
    // 销毁旧图表实例
    if (dailyChart) {
      dailyChart.dispose();
    }

    dailyChart = echarts.init(dailyChartRef.value);
    console.log('[initDailyChart] ECharts instance created');

    const dates = analyticsData.value.daily_cascade_lines.map(s => s.date);
    const acceptedLines = analyticsData.value.daily_cascade_lines.map(s => s.accepted_lines);
    const suggestedLines = analyticsData.value.daily_cascade_lines.map(s => s.suggested_lines);

    console.log('[initDailyChart] Chart data prepared:', { dates, acceptedLines, suggestedLines });

    const option = {
      tooltip: {
        trigger: 'axis',
        backgroundColor: 'rgba(255, 255, 255, 0.9)',
        borderColor: '#eee',
        borderWidth: 1,
        textStyle: {
          color: '#333'
        },
        axisPointer: {
          type: 'line',
          lineStyle: {
            color: '#999',
            type: 'dashed'
          }
        }
      },
      legend: {
        data: ['接受的代码行数', '建议的代码行数'],
        top: 0,
        right: 10,
        itemGap: 20,
        textStyle: {
          color: '#666'
        }
      },
      grid: {
        left: '2%',
        right: '4%',
        bottom: '3%',
        top: '12%',
        containLabel: true
      },
      xAxis: {
        type: 'category',
        data: dates,
        boundaryGap: false,
        axisLabel: {
          rotate: 0,
          color: '#909399',
          formatter: (value: string) => {
            // 只显示月-日
            const date = new Date(value);
            return `${date.getMonth() + 1}-${date.getDate()}`;
          }
        },
        axisLine: {
          show: false
        },
        axisTick: {
          show: false
        },
        splitLine: {
          show: true,
          lineStyle: {
            color: '#f0f0f0',
            type: 'solid'
          }
        }
      },
      yAxis: {
        type: 'value',
        axisLabel: {
          color: '#909399'
        },
        axisLine: {
          show: false
        },
        axisTick: {
          show: false
        },
        splitLine: {
          lineStyle: {
            color: '#f0f0f0',
            type: 'dashed'
          }
        }
      },
      series: [
        {
          name: '接受的代码行数',
          type: 'line',
          data: acceptedLines,
          smooth: true,
          showSymbol: false,
          symbol: 'circle',
          symbolSize: 8,
          lineStyle: {
            width: 3,
            color: '#5b86e5',
            shadowColor: 'rgba(91, 134, 229, 0.3)',
            shadowBlur: 10,
            shadowOffsetY: 5
          },
          itemStyle: {
            color: '#5b86e5',
            borderWidth: 2,
            borderColor: '#fff'
          },
          areaStyle: {
            color: {
              type: 'linear',
              x: 0,
              y: 0,
              x2: 0,
              y2: 1,
              colorStops: [
                { offset: 0, color: 'rgba(91, 134, 229, 0.4)' },
                { offset: 1, color: 'rgba(91, 134, 229, 0.05)' }
              ]
            }
          }
        },
        {
          name: '建议的代码行数',
          type: 'line',
          data: suggestedLines,
          smooth: true,
          showSymbol: false,
          symbol: 'circle',
          symbolSize: 8,
          lineStyle: {
            width: 3,
            color: '#36d1dc',
            type: 'dashed'
          },
          itemStyle: {
            color: '#36d1dc',
            borderWidth: 2,
            borderColor: '#fff'
          },
          areaStyle: {
            opacity: 0
          }
        }
      ]
    };

    dailyChart.setOption(option);
    console.log('[initDailyChart] Chart option set successfully');

    // 强制重绘
    dailyChart.resize();
  } catch (error) {
    console.error('[initDailyChart] Error initializing chart:', error);
  }
};

// 初始化工具使用分布图
const initToolChart = () => {
  console.log('[initToolChart] Starting initialization');

  if (!toolChartRef.value) {
    console.error('[initToolChart] Chart ref not found!');
    return;
  }

  if (!analyticsData.value) {
    console.error('[initToolChart] No analytics data!');
    return;
  }

  if (!analyticsData.value.tool_usage || analyticsData.value.tool_usage.length === 0) {
    console.warn('[initToolChart] No tool usage data available');
    return;
  }

  console.log('[initToolChart] Tool usage data:', analyticsData.value.tool_usage);

  try {
    // 销毁旧图表实例
    if (toolChart) {
      toolChart.dispose();
    }

    toolChart = echarts.init(toolChartRef.value);
    console.log('[initToolChart] ECharts instance created');

    const data = analyticsData.value.tool_usage.map(t => ({
      name: t.tool_name,
      value: t.count
    }));

    console.log('[initToolChart] Pie chart data:', data);

    const option = {
      tooltip: {
        trigger: 'item',
        formatter: '{b}: {c} ({d}%)',
        backgroundColor: 'rgba(255, 255, 255, 0.9)'
      },
      legend: {
        orient: 'vertical',
        right: 10,
        top: 'center',
        itemWidth: 10,
        itemHeight: 10,
        textStyle: {
          color: '#666',
          fontSize: 12
        }
      },
      series: [
        {
          name: '工具使用',
          type: 'pie',
          radius: ['40%', '70%'],
          center: ['40%', '50%'],
          avoidLabelOverlap: false,
          itemStyle: {
            borderRadius: 10,
            borderColor: '#fff',
            borderWidth: 2
          },
          label: {
            show: false,
            position: 'center'
          },
          emphasis: {
            label: {
              show: true,
              fontSize: 14,
              fontWeight: 'bold'
            },
            itemStyle: {
              shadowBlur: 10,
              shadowOffsetX: 0,
              shadowColor: 'rgba(0, 0, 0, 0.2)'
            }
          },
          labelLine: {
            show: false
          },
          data: data
        }
      ]
    };

    toolChart.setOption(option);
    console.log('[initToolChart] Chart option set successfully');

    // 强制重绘
    toolChart.resize();
  } catch (error) {
    console.error('[initToolChart] Error initializing chart:', error);
  }
};

// 初始化模型使用分布图
const initModelChart = () => {
  console.log('[initModelChart] Starting initialization');

  if (!modelChartRef.value) {
    console.error('[initModelChart] Chart ref not found!');
    return;
  }

  if (!analyticsData.value) {
    console.error('[initModelChart] No analytics data!');
    return;
  }

  if (!analyticsData.value.model_usage_summary || analyticsData.value.model_usage_summary.length === 0) {
    console.warn('[initModelChart] No model usage summary data available');
    return;
  }

  console.log('[initModelChart] Model usage summary:', analyticsData.value.model_usage_summary);

  try {
    // 销毁旧图表实例
    if (modelChart) {
      modelChart.dispose();
    }

    modelChart = echarts.init(modelChartRef.value);
    console.log('[initModelChart] ECharts instance created');

    const models = analyticsData.value.model_usage_summary.map(m => m.model_name);
    const counts = analyticsData.value.model_usage_summary.map(m => m.total_count);
    const percentages = analyticsData.value.model_usage_summary.map(m => m.percentage);

    console.log('[initModelChart] Bar chart data:', { models, counts, percentages });

    const option = {
      tooltip: {
        trigger: 'axis',
        axisPointer: {
          type: 'shadow'
        },
        backgroundColor: 'rgba(255, 255, 255, 0.9)',
        formatter: (params: any) => {
          const dataIndex = params[0].dataIndex;
          const modelName = models[dataIndex];
          const count = counts[dataIndex];
          const percentage = percentages[dataIndex];
          return `<div style="font-weight:bold;margin-bottom:5px">${modelName}</div>
                  使用次数: <span style="color:#409EFF">${count}</span><br/>
                  占比: <span style="color:#409EFF">${percentage.toFixed(2)}%</span>`;
        }
      },
      grid: {
        left: '3%',
        right: '12%',
        bottom: '3%',
        top: '3%',
        containLabel: true
      },
      xAxis: {
        type: 'value',
        axisLabel: {
          color: '#909399'
        },
        axisLine: {
          show: false
        },
        splitLine: {
          lineStyle: {
            color: '#f0f0f0',
            type: 'dashed'
          }
        }
      },
      yAxis: {
        type: 'category',
        data: models,
        axisLabel: {
          color: '#606266',
          width: 100,
          overflow: 'truncate'
        },
        axisLine: {
          lineStyle: {
            color: '#e0e0e0'
          }
        },
        axisTick: {
          show: false
        }
      },
      series: [
        {
          type: 'bar',
          data: counts,
          showBackground: true,
          backgroundStyle: {
            color: 'rgba(180, 180, 180, 0.1)',
            borderRadius: [0, 4, 4, 0]
          },
          itemStyle: {
            color: new echarts.graphic.LinearGradient(0, 0, 1, 0, [
              { offset: 0, color: '#36d1dc' },
              { offset: 1, color: '#5b86e5' }
            ]),
            borderRadius: [0, 4, 4, 0]
          },
          label: {
            show: true,
            position: 'right',
            formatter: (params: any) => {
              const percentage = percentages[params.dataIndex];
              return `${percentage.toFixed(1)}%`;
            },
            color: '#909399',
            fontSize: 12
          },
          barWidth: 20
        }
      ]
    };

    modelChart.setOption(option);
    console.log('[initModelChart] Chart option set successfully');

    // 强制重绘
    modelChart.resize();
  } catch (error) {
    console.error('[initModelChart] Error initializing chart:', error);
  }
};

// 初始化Token消耗趋势图
const initTokenChart = () => {
  console.log('[initTokenChart] Starting initialization');

  if (!tokenChartRef.value) {
    console.error('[initTokenChart] Chart ref not found!');
    return;
  }

  if (!analyticsData.value) {
    console.error('[initTokenChart] No analytics data!');
    return;
  }

  if (!analyticsData.value.model_usage_details || analyticsData.value.model_usage_details.length === 0) {
    console.warn('[initTokenChart] No model usage details data available');
    return;
  }

  console.log('[initTokenChart] Model usage details:', analyticsData.value.model_usage_details);

  try {
    // 销毁旧图表实例
    if (tokenChart) {
      tokenChart.dispose();
    }

    tokenChart = echarts.init(tokenChartRef.value);
    console.log('[initTokenChart] ECharts instance created');

    // 按日期聚合Token消耗（除以 100 进行单位转换）
    const tokenByDate = new Map<string, number>();
    analyticsData.value.model_usage_details.forEach(entry => {
      const current = tokenByDate.get(entry.date) || 0;
      tokenByDate.set(entry.date, current + (entry.token_usage / 100));
    });

    const dates = Array.from(tokenByDate.keys()).sort();
    const tokens = dates.map(date => {
      const value = tokenByDate.get(date) || 0;
      return Math.round(value * 10) / 10; // 保留 1 位小数
    });

    console.log('[initTokenChart] Token chart data (converted):', { dates, tokens });

    const option = {
      tooltip: {
        trigger: 'axis',
        backgroundColor: 'rgba(255, 255, 255, 0.9)',
        axisPointer: {
          type: 'line',
          lineStyle: {
            color: '#999',
            type: 'dashed'
          }
        }
      },
      grid: {
        left: '2%',
        right: '4%',
        bottom: '3%',
        top: '12%',
        containLabel: true
      },
      xAxis: {
        type: 'category',
        data: dates,
        boundaryGap: false,
        axisLabel: {
          rotate: 0,
          color: '#909399',
          formatter: (value: string) => {
            const date = new Date(value);
            return `${date.getMonth() + 1}-${date.getDate()}`;
          }
        },
        axisLine: {
          show: false
        },
        axisTick: {
          show: false
        },
        splitLine: {
          show: true,
          lineStyle: {
            color: '#f0f0f0',
            type: 'solid'
          }
        }
      },
      yAxis: {
        type: 'value',
        name: 'Tokens (K)',
        nameTextStyle: {
          color: '#909399',
          padding: [0, 0, 0, 20]
        },
        axisLabel: {
          color: '#909399'
        },
        axisLine: {
          show: false
        },
        splitLine: {
          lineStyle: {
            color: '#f0f0f0',
            type: 'dashed'
          }
        }
      },
      series: [
        {
          name: 'Tokens',
          type: 'line',
          data: tokens,
          smooth: true,
          showSymbol: false,
          symbol: 'circle',
          symbolSize: 8,
          lineStyle: {
            width: 3,
            color: '#8e2de2',
            shadowColor: 'rgba(142, 45, 226, 0.3)',
            shadowBlur: 10
          },
          itemStyle: {
            color: '#8e2de2',
            borderWidth: 2,
            borderColor: '#fff'
          },
          areaStyle: {
            color: {
              type: 'linear',
              x: 0,
              y: 0,
              x2: 0,
              y2: 1,
              colorStops: [
                { offset: 0, color: 'rgba(142, 45, 226, 0.4)' },
                { offset: 1, color: 'rgba(142, 45, 226, 0.05)' }
              ]
            }
          }
        }
      ]
    };

    tokenChart.setOption(option);
    console.log('[initTokenChart] Chart option set successfully');

    // 强制重绘
    tokenChart.resize();
  } catch (error) {
    console.error('[initTokenChart] Error initializing chart:', error);
  }
};

// 初始化按语言补全统计图
const initLanguageChart = () => {
  if (!languageChartRef.value || !analyticsData.value?.completions_by_language?.length) {
    return;
  }

  try {
    if (languageChart) {
      languageChart.dispose();
    }

    languageChart = echarts.init(languageChartRef.value);

    const data = analyticsData.value.completions_by_language;
    const languages = data.map(item => item.language_name);
    const acceptances = data.map(item => item.statistics.num_acceptances);

    const option = {
      tooltip: {
        trigger: 'axis',
        axisPointer: { type: 'shadow' }
      },
      grid: {
        left: '3%',
        right: '4%',
        bottom: '3%',
        top: '10%',
        containLabel: true
      },
      xAxis: {
        type: 'category',
        data: languages,
        axisLabel: {
          color: '#606266',
          rotate: 45,
          interval: 0
        }
      },
      yAxis: {
        type: 'value',
        name: '接受次数',
        axisLabel: { color: '#909399' }
      },
      series: [{
        type: 'bar',
        data: acceptances,
        itemStyle: {
          color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
            { offset: 0, color: '#11998e' },
            { offset: 1, color: '#38ef7d' }
          ]),
          borderRadius: [4, 4, 0, 0]
        }
      }]
    };

    languageChart.setOption(option);
    languageChart.resize();
  } catch (error) {
    console.error('[initLanguageChart] Error:', error);
  }
};

// 初始化按模型Chat统计图
const initChatModelChart = () => {
  if (!chatModelChartRef.value || !analyticsData.value?.chats_by_model?.length) {
    return;
  }

  try {
    if (chatModelChart) {
      chatModelChart.dispose();
    }

    chatModelChart = echarts.init(chatModelChartRef.value);

    const data = analyticsData.value.chats_by_model;
    const models = data.map(item => item.model_name);
    const counts = data.map(item => item.stats.chats_sent);

    const option = {
      tooltip: {
        trigger: 'item',
        formatter: '{b}: {c} ({d}%)'
      },
      legend: {
        type: 'scroll',
        orient: 'vertical',
        right: 10,
        top: 20,
        bottom: 20
      },
      series: [{
        type: 'pie',
        radius: ['40%', '70%'],
        center: ['40%', '50%'],
        avoidLabelOverlap: false,
        itemStyle: {
          borderRadius: 4,
          borderColor: '#fff',
          borderWidth: 2
        },
        label: {
          show: false
        },
        emphasis: {
          label: {
            show: true,
            fontSize: 14,
            fontWeight: 'bold'
          }
        },
        data: models.map((model, i) => ({
          name: model,
          value: counts[i]
        }))
      }]
    };

    chatModelChart.setOption(option);
    chatModelChart.resize();
  } catch (error) {
    console.error('[initChatModelChart] Error:', error);
  }
};

// 初始化每日补全趋势图
const initCompletionsByDayChart = () => {
  if (!completionsByDayChartRef.value || !analyticsData.value?.completions_by_day?.length) {
    return;
  }

  try {
    if (completionsByDayChart) {
      completionsByDayChart.dispose();
    }

    completionsByDayChart = echarts.init(completionsByDayChartRef.value);

    const data = analyticsData.value.completions_by_day;
    const dates = data.map(item => item.date);
    const acceptances = data.map(item => item.statistics?.num_acceptances || 0);
    const rejections = data.map(item => item.statistics?.num_rejections || 0);

    const option = {
      tooltip: {
        trigger: 'axis',
        axisPointer: { type: 'cross' }
      },
      legend: {
        data: ['接受', '拒绝'],
        top: 0
      },
      grid: {
        left: '3%',
        right: '4%',
        bottom: '3%',
        top: '15%',
        containLabel: true
      },
      xAxis: {
        type: 'category',
        data: dates,
        axisLabel: { color: '#909399', rotate: 45 }
      },
      yAxis: {
        type: 'value',
        axisLabel: { color: '#909399' }
      },
      series: [
        {
          name: '接受',
          type: 'bar',
          stack: 'total',
          data: acceptances,
          itemStyle: { color: '#67c23a' }
        },
        {
          name: '拒绝',
          type: 'bar',
          stack: 'total',
          data: rejections,
          itemStyle: { color: '#f56c6c' }
        }
      ]
    };

    completionsByDayChart.setOption(option);
    completionsByDayChart.resize();
  } catch (error) {
    console.error('[initCompletionsByDayChart] Error:', error);
  }
};

// 初始化每日Chat趋势图
const initChatsByDayChart = () => {
  if (!chatsByDayChartRef.value || !analyticsData.value?.chats_by_day?.length) {
    return;
  }

  try {
    if (chatsByDayChart) {
      chatsByDayChart.dispose();
    }

    chatsByDayChart = echarts.init(chatsByDayChartRef.value);

    const data = analyticsData.value.chats_by_day;
    const dates = data.map(item => item.date);
    const chatsSent = data.map(item => item.stats?.chats_sent || 0);

    const option = {
      tooltip: {
        trigger: 'axis',
        axisPointer: { type: 'line' }
      },
      grid: {
        left: '3%',
        right: '4%',
        bottom: '3%',
        top: '10%',
        containLabel: true
      },
      xAxis: {
        type: 'category',
        data: dates,
        axisLabel: { color: '#909399', rotate: 45 }
      },
      yAxis: {
        type: 'value',
        name: 'Chats',
        axisLabel: { color: '#909399' }
      },
      series: [{
        type: 'line',
        data: chatsSent,
        smooth: true,
        areaStyle: {
          color: {
            type: 'linear',
            x: 0, y: 0, x2: 0, y2: 1,
            colorStops: [
              { offset: 0, color: 'rgba(64, 158, 255, 0.4)' },
              { offset: 1, color: 'rgba(64, 158, 255, 0.05)' }
            ]
          }
        },
        lineStyle: { color: '#409EFF', width: 2 },
        itemStyle: { color: '#409EFF' }
      }]
    };

    chatsByDayChart.setOption(option);
    chatsByDayChart.resize();
  } catch (error) {
    console.error('[initChatsByDayChart] Error:', error);
  }
};

// 销毁所有图表
const destroyCharts = () => {
  if (dailyChart) {
    dailyChart.dispose();
    dailyChart = null;
  }
  if (toolChart) {
    toolChart.dispose();
    toolChart = null;
  }
  if (modelChart) {
    modelChart.dispose();
    modelChart = null;
  }
  if (tokenChart) {
    tokenChart.dispose();
    tokenChart = null;
  }
  if (languageChart) {
    languageChart.dispose();
    languageChart = null;
  }
  if (chatModelChart) {
    chatModelChart.dispose();
    chatModelChart = null;
  }
  if (completionsByDayChart) {
    completionsByDayChart.dispose();
    completionsByDayChart = null;
  }
  if (chatsByDayChart) {
    chatsByDayChart.dispose();
    chatsByDayChart = null;
  }
};

// 格式化数字
const formatNumber = (num: number): string => {
  if (num >= 1000000) {
    return (num / 1000000).toFixed(1) + 'M';
  } else if (num >= 1000) {
    return (num / 1000).toFixed(1) + 'K';
  }
  // 保留 1 位小数
  return num.toFixed(1);
};

// 格式化字节数
const formatBytes = (bytes: number): string => {
  if (bytes >= 1073741824) {
    return (bytes / 1073741824).toFixed(1) + ' GB';
  } else if (bytes >= 1048576) {
    return (bytes / 1048576).toFixed(1) + ' MB';
  } else if (bytes >= 1024) {
    return (bytes / 1024).toFixed(1) + ' KB';
  }
  return bytes + ' B';
};

// 格式化模型名称
const formatModelName = (name: string): string => {
  if (name.length > 15) {
    return name.substring(0, 12) + '...';
  }
  return name;
};

// 格式化工具名称
const formatToolName = (name: string): string => {
  // 移除常见的长前缀
  const cleanName = name.replace(/^mcp\d+_/, '').replace(/_exa$/, '');
  if (cleanName.length > 15) {
    return cleanName.substring(0, 12) + '...';
  }
  return cleanName;
};

// 刷新数据
const handleRefresh = () => {
  loadAnalytics();
};

// 关闭对话框
const handleClose = () => {
  visible.value = false;
};
</script>

<style scoped lang="scss">
.analytics-container {
  min-height: 400px;
  padding: 10px;
}

.stats-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 20px;
  margin-bottom: 24px;
}

.stat-card {
  display: flex;
  align-items: center;
  padding: 20px;
  border-radius: 12px;
  color: white;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  transition: transform 0.3s ease, box-shadow 0.3s ease;

  &:hover {
    transform: translateY(-4px);
    box-shadow: 0 8px 20px rgba(0, 0, 0, 0.15);
  }

  .stat-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 48px;
    height: 48px;
    border-radius: 12px;
    background: rgba(255, 255, 255, 0.2);
    font-size: 24px;
    margin-right: 16px;
  }

  .stat-content {
    flex: 1;
    overflow: hidden;
  }

  .stat-label {
    font-size: 13px;
    opacity: 0.9;
    margin-bottom: 4px;
    white-space: nowrap;
  }

  .stat-value {
    font-size: 24px;
    font-weight: bold;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;

    &.small {
      font-size: 18px;
    }
  }
}

/* 渐变背景色 */
.blue-gradient {
  background: linear-gradient(135deg, #36d1dc 0%, #5b86e5 100%);
}

.green-gradient {
  background: linear-gradient(135deg, #11998e 0%, #38ef7d 100%);
}

.orange-gradient {
  background: linear-gradient(135deg, #f2994a 0%, #f2c94c 100%);
}

.purple-gradient {
  background: linear-gradient(135deg, #8e2de2 0%, #4a00e0 100%);
}

.teal-gradient {
  background: linear-gradient(135deg, #00b09b 0%, #96c93d 100%);
}

.red-gradient {
  background: linear-gradient(135deg, #ff416c 0%, #ff4b2b 100%);
}

.charts-container {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 20px;
}

.chart-card {
  border-radius: 12px;
  border: none;
  
  :deep(.el-card__header) {
    padding: 15px 20px;
    border-bottom: 1px solid #f0f0f0;
  }

  .card-header {
    display: flex;
    align-items: center;
    font-weight: bold;
    font-size: 16px;
    color: #303133;

    .header-icon {
      margin-right: 8px;
      font-size: 18px;
      color: #409EFF;
    }
  }

  .chart {
    width: 100%;
    height: 350px;
  }
}

@media (max-width: 1200px) {
  .charts-container {
    grid-template-columns: 1fr;
  }
}

/* ===== 新增样式 ===== */

.section-title {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 16px;
  font-weight: 600;
  color: #303133;
  margin-bottom: 16px;
  padding-bottom: 8px;
  border-bottom: 2px solid #409EFF;
}

/* AI 贡献统计 */
.ai-contribution-section {
  margin-bottom: 24px;
  padding: 20px;
  background: linear-gradient(135deg, #f5f7fa 0%, #e8ecf1 100%);
  border-radius: 12px;
}

.ai-contribution-cards {
  display: flex;
  gap: 30px;
  align-items: center;
}

.contribution-card {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.contribution-ring {
  position: relative;
  width: 120px;
  height: 120px;

  svg {
    transform: rotate(-90deg);
    width: 100%;
    height: 100%;
  }

  circle {
    fill: none;
    stroke-width: 8;
    stroke-linecap: round;
  }

  .bg {
    stroke: #e0e0e0;
  }

  .progress {
    stroke: url(#gradient);
    stroke: #409EFF;
    transition: stroke-dasharray 0.5s ease;
  }

  .percent-text {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    font-size: 24px;
    font-weight: bold;
    color: #409EFF;
  }
}

.contribution-label {
  margin-top: 8px;
  font-size: 14px;
  color: #606266;
}

.contribution-breakdown {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.breakdown-item {
  position: relative;
}

.breakdown-bar {
  height: 24px;
  border-radius: 4px;
  transition: width 0.5s ease;
  min-width: 4px;

  &.autocomplete {
    background: linear-gradient(90deg, #36d1dc, #5b86e5);
  }

  &.cascade {
    background: linear-gradient(90deg, #11998e, #38ef7d);
  }

  &.command {
    background: linear-gradient(90deg, #f2994a, #f2c94c);
  }

  &.user {
    background: linear-gradient(90deg, #909399, #c0c4cc);
  }
}

.breakdown-info {
  display: flex;
  justify-content: space-between;
  margin-top: 4px;
  font-size: 12px;
  color: #606266;
}

/* 补全统计和 Chat 统计 */
.completion-stats-section,
.chat-stats-section {
  margin-bottom: 24px;
}

.completion-stats-cards,
.chat-stats-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
  gap: 16px;
}

.mini-stat-card {
  background: #fff;
  border-radius: 8px;
  padding: 16px;
  text-align: center;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
  transition: transform 0.2s ease, box-shadow 0.2s ease;

  &:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }
}

.mini-stat-value {
  font-size: 24px;
  font-weight: bold;
  color: #303133;
  margin-bottom: 4px;

  &.success {
    color: #67c23a;
  }

  &.warning {
    color: #e6a23c;
  }

  &.danger {
    color: #f56c6c;
  }

  &.primary {
    color: #409EFF;
  }

  &.info {
    color: #909399;
  }
}

.mini-stat-label {
  font-size: 12px;
  color: #909399;
}
</style>

