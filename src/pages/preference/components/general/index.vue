<script setup lang="ts">
import { disable, enable, isEnabled } from '@tauri-apps/plugin-autostart'
import { InputNumber, Select, Switch } from 'ant-design-vue'
import { computed, watch } from 'vue'

import MacosPermissions from './components/macos-permissions/index.vue'
import ThemeMode from './components/theme-mode/index.vue'

import ProList from '@/components/pro-list/index.vue'
import ProListItem from '@/components/pro-list-item/index.vue'
import { useGeneralStore } from '@/stores/general'

const generalStore = useGeneralStore()

watch(() => generalStore.app.autostart, async (value) => {
  const enabled = await isEnabled()

  if (value && !enabled) {
    return enable()
  }

  if (!value && enabled) {
    disable()
  }
}, { immediate: true })

// 计算推荐的 FPS 值
const recommendedFps = computed(() => {
  const throttleMs = generalStore.performance.mouseMoveThrottle
  if (throttleMs <= 0) return '∞'
  const fps = Math.round(1000 / throttleMs)
  return `${fps} FPS`
})
</script>

<template>
  <MacosPermissions />

  <ProList :title="$t('pages.preference.general.labels.appSettings')">
    <ProListItem :title="$t('pages.preference.general.labels.launchOnStartup')">
      <Switch v-model:checked="generalStore.app.autostart" />
    </ProListItem>

    <ProListItem
      :description="$t('pages.preference.general.hints.showTaskbarIcon')"
      :title="$t('pages.preference.general.labels.showTaskbarIcon')"
    >
      <Switch v-model:checked="generalStore.app.taskbarVisible" />
    </ProListItem>
  </ProList>

  <ProList :title="$t('pages.preference.general.labels.appearanceSettings')">
    <ThemeMode />

    <ProListItem :title="$t('pages.preference.general.labels.language')">
      <Select v-model:value="generalStore.appearance.language">
        <Select.Option value="zh-CN">
          简体中文
        </Select.Option>
        <Select.Option value="en-US">
          English
        </Select.Option>
        <Select.Option value="vi-VN">
          Tiếng Việt
        </Select.Option>
        <Select.Option value="pt-BR">
          Português
        </Select.Option>
      </Select>
    </ProListItem>
  </ProList>

  <ProList :title="$t('pages.preference.general.labels.updateSettings')">
    <ProListItem :title="$t('pages.preference.general.labels.autoCheckUpdate')">
      <Switch v-model:checked="generalStore.update.autoCheck" />
    </ProListItem>
  </ProList>

  <ProList :title="$t('pages.preference.general.labels.performanceSettings')">
    <ProListItem
      :description="$t('pages.preference.general.hints.mouseMoveThrottle')"
      :title="$t('pages.preference.general.labels.mouseMoveThrottle')"
    >
      <InputNumber
        v-model:value="generalStore.performance.mouseMoveThrottle"
        :max="100"
        :min="0"
        :step="1"
        style="width: 150px"
      >
        <template #addonAfter>
          ms ({{ recommendedFps }})
        </template>
      </InputNumber>
    </ProListItem>

    <ProListItem
      :description="$t('pages.preference.general.hints.mouseMoveThrottleOptimize')"
      :title="$t('pages.preference.general.labels.mouseMoveThrottleOptimize')"
    >
      <Switch v-model:checked="generalStore.performance.mouseMoveThrottleOptimize" />
    </ProListItem>
  </ProList>
</template>
