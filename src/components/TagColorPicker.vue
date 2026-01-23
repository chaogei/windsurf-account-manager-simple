<template>
  <div class="tag-color-picker">
    <div class="tags-list" v-if="tags.length > 0">
      <div 
        v-for="tag in tags" 
        :key="tag" 
        class="tag-item"
      >
        <span 
          class="tag-preview"
          :style="getTagPreviewStyle(tag)"
        >
          {{ tag }}
        </span>
        <el-color-picker
          v-model="tagColorMap[tag]"
          show-alpha
          :predefine="predefineColors"
          @change="(color: string | null) => handleColorChange(tag, color)"
        />
        <el-button
          v-if="getTagColor(tag)"
          :icon="Close"
          circle
          size="small"
          class="remove-color-btn"
          @click.stop="removeTagColor(tag)"
        />
      </div>
    </div>
    
    <div v-if="tags.length === 0" class="no-tags-hint">
      {{ $t('dialog.tagColorPicker.noTags') }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, watch } from 'vue';
import { Close } from '@element-plus/icons-vue';
import type { TagWithColor } from '@/types';

const props = defineProps<{
  tags: string[];
  tagColors: TagWithColor[];
}>();

const emit = defineEmits<{
  'update:tagColors': [colors: TagWithColor[]];
}>();

// 预定义颜色
const predefineColors = [
  '#ff4500',
  '#ff8c00',
  '#ffd700',
  '#90ee90',
  '#00ced1',
  '#1e90ff',
  '#c71585',
  '#ff69b4',
  '#ba55d3',
  '#cd5c5c',
  '#40e0d0',
  '#7b68ee',
  '#00fa9a',
  '#6495ed',
  '#dda0dd',
  '#a0522d',
  'rgba(255, 69, 0, 0.68)',
  'rgba(255, 120, 0, 0.8)',
  'rgba(0, 191, 255, 0.8)',
  'rgba(144, 238, 144, 0.8)',
];

// 标签颜色映射
const tagColorMap = reactive<Record<string, string>>({});

// 初始化标签颜色映射
function initTagColorMap() {
  props.tags.forEach(tag => {
    const existing = props.tagColors.find(tc => tc.name === tag);
    if (existing) {
      tagColorMap[tag] = existing.color;
    } else if (!tagColorMap[tag]) {
      tagColorMap[tag] = '';
    }
  });
}

// 监听 tags 和 tagColors 变化
watch([() => props.tags, () => props.tagColors], () => {
  initTagColorMap();
}, { immediate: true, deep: true });

// 获取标签颜色
function getTagColor(tagName: string): string | null {
  const tagWithColor = props.tagColors.find(t => t.name === tagName);
  return tagWithColor?.color || null;
}

// 获取标签预览样式
function getTagPreviewStyle(tagName: string): Record<string, string> {
  const color = getTagColor(tagName);
  if (!color) {
    return {
      backgroundColor: '#f0f2f5',
      color: '#606266',
      border: '1px solid #dcdfe6'
    };
  }
  
  // 解析颜色（支持hex和rgba）
  let r = 0, g = 0, b = 0, a = 1;
  
  if (color.startsWith('rgba')) {
    const rgbaMatch = color.match(/rgba?\((\d+),\s*(\d+),\s*(\d+)(?:,\s*([\d.]+))?\)/);
    if (rgbaMatch) {
      [, r, g, b] = rgbaMatch.map(Number);
      a = parseFloat(rgbaMatch[4] || '1');
    }
  } else if (color.startsWith('#')) {
    const hex = color.slice(1);
    if (hex.length === 6) {
      r = parseInt(hex.slice(0, 2), 16);
      g = parseInt(hex.slice(2, 4), 16);
      b = parseInt(hex.slice(4, 6), 16);
    } else if (hex.length === 8) {
      r = parseInt(hex.slice(0, 2), 16);
      g = parseInt(hex.slice(2, 4), 16);
      b = parseInt(hex.slice(4, 6), 16);
      a = parseInt(hex.slice(6, 8), 16) / 255;
    }
  }
  
  const bgAlpha = Math.min(a * 0.25, 0.35);
  const borderAlpha = Math.min(a * 0.5, 0.6);
  
  return {
    backgroundColor: `rgba(${r}, ${g}, ${b}, ${bgAlpha})`,
    border: `1px solid rgba(${r}, ${g}, ${b}, ${borderAlpha})`,
    color: `rgba(${r}, ${g}, ${b}, ${Math.max(a, 0.8)})`
  };
}

// 处理颜色变化
function handleColorChange(tagName: string, color: string | null) {
  if (!color) {
    removeTagColor(tagName);
    return;
  }
  
  const newColors = [...props.tagColors];
  const existingIndex = newColors.findIndex(t => t.name === tagName);
  
  if (existingIndex >= 0) {
    newColors[existingIndex] = { name: tagName, color };
  } else {
    newColors.push({ name: tagName, color });
  }
  
  emit('update:tagColors', newColors);
}

// 移除标签颜色
function removeTagColor(tagName: string) {
  const newColors = props.tagColors.filter(t => t.name !== tagName);
  tagColorMap[tagName] = '';
  emit('update:tagColors', newColors);
}

// 当标签列表变化时，清理已删除标签的颜色
watch(() => props.tags, (newTags) => {
  const validColors = props.tagColors.filter(tc => newTags.includes(tc.name));
  if (validColors.length !== props.tagColors.length) {
    emit('update:tagColors', validColors);
  }
  // 清理 tagColorMap 中不存在的标签
  Object.keys(tagColorMap).forEach(key => {
    if (!newTags.includes(key)) {
      delete tagColorMap[key];
    }
  });
}, { deep: true });
</script>

<style scoped>
.tag-color-picker {
  padding: 8px 0;
}

.tags-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.tag-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 6px 10px;
  background: #f8f9fa;
  border-radius: 8px;
  transition: all 0.2s ease;
}

.tag-item:hover {
  background-color: #f0f2f5;
}

.tag-preview {
  padding: 4px 12px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
  min-width: 60px;
  text-align: center;
  transition: all 0.2s ease;
}

.remove-color-btn {
  width: 20px !important;
  height: 20px !important;
  padding: 0 !important;
  margin-left: auto;
}

.remove-color-btn .el-icon {
  font-size: 10px;
}

.no-tags-hint {
  color: #909399;
  font-size: 13px;
  text-align: center;
  padding: 20px 0;
}

/* 颜色选择器样式调整 */
:deep(.el-color-picker__trigger) {
  width: 32px;
  height: 32px;
  border-radius: 6px;
}

:deep(.el-color-picker__color) {
  border-radius: 4px;
}

:deep(.el-color-picker__color-inner) {
  border-radius: 4px;
}

/* 深色模式 */
:root.dark .tag-item {
  background: #2a2a2a;
}

:root.dark .tag-item:hover {
  background-color: #333;
}

:root.dark .tag-preview {
  background-color: #262729;
  color: #cfd3dc;
  border-color: #4c4d4f;
}

:root.dark .no-tags-hint {
  color: #64748b;
}
</style>
