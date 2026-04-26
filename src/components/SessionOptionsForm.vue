<template>
  <div class="session-options-form">
    <n-tabs type="segment" animated size="small">
      <!-- Connection Tab -->
      <n-tab-pane name="connection">
        <template #tab>
          <div class="tab-label">
            <n-icon :size="16"><WifiOutline /></n-icon>
            <span>{{ t('options.connection') }}</span>
          </div>
        </template>
        <n-form label-placement="left" label-width="120" size="small" class="options-form">
          <n-form-item :label="t('options.serial')">
            <n-tooltip trigger="hover">
              <template #trigger>
                <n-input
                  v-model:value="options.serial"
                  :placeholder="t('options.serialPlaceholder')"
                  clearable
                />
              </template>
              {{ t('options.serialHint') }}
            </n-tooltip>
          </n-form-item>
          <n-form-item :label="t('options.tcpip')">
            <n-tooltip trigger="hover">
              <template #trigger>
                <n-input
                  v-model:value="options.tcpip"
                  :placeholder="t('options.tcpipPlaceholder')"
                  clearable
                />
              </template>
              {{ t('options.tcpipHint') }}
            </n-tooltip>
          </n-form-item>
          <n-form-item :label="t('options.portRange')">
            <n-tooltip trigger="hover">
              <template #trigger>
                <n-input
                  v-model:value="options.portRange"
                  :placeholder="t('options.portRangePlaceholder')"
                  clearable
                />
              </template>
              {{ t('options.portRangeHint') }}
            </n-tooltip>
          </n-form-item>
        </n-form>
      </n-tab-pane>

      <!-- Video Tab -->
      <n-tab-pane name="video">
        <template #tab>
          <div class="tab-label">
            <n-icon :size="16"><VideocamOutline /></n-icon>
            <span>{{ t('options.video') }}</span>
          </div>
        </template>
        <n-form label-placement="left" label-width="120" size="small" class="options-form">
          <n-form-item :label="t('options.videoCodec')">
            <n-tooltip trigger="hover">
              <template #trigger>
                <n-select
                  v-model:value="options.videoCodec"
                  :options="videoCodecOptions"
                  :placeholder="t('options.auto')"
                  clearable
                />
              </template>
              {{ t('options.videoCodecHint') }}
            </n-tooltip>
          </n-form-item>
          <n-form-item :label="t('options.videoBitrate')">
            <n-tooltip trigger="hover">
              <template #trigger>
                <n-input-number
                  v-model:value="options.videoBitrate"
                  :min="1"
                  :max="50"
                  placeholder="8"
                />
              </template>
              {{ t('options.videoBitrateHint') }}
            </n-tooltip>
          </n-form-item>
          <n-form-item :label="t('options.maxSize')">
            <n-tooltip trigger="hover">
              <template #trigger>
                <n-select
                  v-model:value="maxSizeValue"
                  :options="maxSizeOptions"
                  :placeholder="t('options.noLimit')"
                  clearable
                  @update:value="updateMaxSize"
                />
              </template>
              {{ t('options.maxSizeHint') }}
            </n-tooltip>
          </n-form-item>
          <n-form-item :label="t('options.maxFps')">
            <n-tooltip trigger="hover">
              <template #trigger>
                <n-select
                  v-model:value="maxFpsValue"
                  :options="maxFpsOptions"
                  :placeholder="t('options.noLimit')"
                  clearable
                  @update:value="updateMaxFps"
                />
              </template>
              {{ t('options.maxFpsHint') }}
            </n-tooltip>
          </n-form-item>
          <n-form-item :label="t('options.crop')">
            <n-tooltip trigger="hover">
              <template #trigger>
                <n-input
                  v-model:value="options.crop"
                  :placeholder="t('options.cropPlaceholder')"
                  clearable
                />
              </template>
              {{ t('options.cropHint') }}
            </n-tooltip>
          </n-form-item>
        </n-form>
      </n-tab-pane>

      <!-- Audio Tab -->
      <n-tab-pane name="audio">
        <template #tab>
          <div class="tab-label">
            <n-icon :size="16"><VolumeHighOutline /></n-icon>
            <span>{{ t('options.audio') }}</span>
          </div>
        </template>
        <n-form label-placement="left" label-width="120" size="small" class="options-form">
          <n-form-item :label="t('options.audioEnabled')">
            <n-tooltip trigger="hover">
              <template #trigger>
                <n-switch v-model:value="audioEnabled" />
              </template>
              {{ t('options.audioEnabledHint') }}
            </n-tooltip>
          </n-form-item>
          <n-form-item :label="t('options.audioCodec')">
            <n-tooltip trigger="hover">
              <template #trigger>
                <n-select
                  v-model:value="options.audioCodec"
                  :options="audioCodecOptions"
                  :placeholder="t('options.auto')"
                  clearable
                  :disabled="!audioEnabled"
                />
              </template>
              {{ t('options.audioCodecHint') }}
            </n-tooltip>
          </n-form-item>
          <n-form-item :label="t('options.audioBitrate')">
            <n-tooltip trigger="hover">
              <template #trigger>
                <n-input-number
                  v-model:value="options.audioBitrate"
                  :min="1"
                  :max="512"
                  placeholder="128"
                  :disabled="!audioEnabled"
                />
              </template>
              {{ t('options.audioBitrateHint') }}
            </n-tooltip>
          </n-form-item>
          <n-form-item :label="t('options.audioSource')">
            <n-tooltip trigger="hover">
              <template #trigger>
                <n-select
                  v-model:value="options.audioSource"
                  :options="audioSourceOptions"
                  :placeholder="t('options.auto')"
                  clearable
                  :disabled="!audioEnabled"
                />
              </template>
              {{ t('options.audioSourceHint') }}
            </n-tooltip>
          </n-form-item>
        </n-form>
      </n-tab-pane>

      <!-- Recording Tab -->
      <n-tab-pane name="recording">
        <template #tab>
          <div class="tab-label">
            <n-icon :size="16"><RadioOutline /></n-icon>
            <span>{{ t('options.recording') }}</span>
          </div>
        </template>
        <n-form label-placement="left" label-width="120" size="small" class="options-form">
          <n-form-item :label="t('options.recordFile')">
            <n-tooltip trigger="hover">
              <template #trigger>
                <n-input
                  v-model:value="options.record"
                  :placeholder="t('options.recordFilePlaceholder')"
                  clearable
                />
              </template>
              {{ t('options.recordFileHint') }}
            </n-tooltip>
          </n-form-item>
          <n-form-item :label="t('options.recordFormat')">
            <n-tooltip trigger="hover">
              <template #trigger>
                <n-select
                  v-model:value="options.recordFormat"
                  :options="recordFormatOptions"
                  :placeholder="t('options.autoDetect')"
                  clearable
                />
              </template>
              {{ t('options.recordFormatHint') }}
            </n-tooltip>
          </n-form-item>
        </n-form>
      </n-tab-pane>

      <!-- Window Tab -->
      <n-tab-pane name="window">
        <template #tab>
          <div class="tab-label">
            <n-icon :size="16"><BrowsersOutline /></n-icon>
            <span>{{ t('options.window') }}</span>
          </div>
        </template>
        <n-form label-placement="left" label-width="120" size="small" class="options-form">
          <n-form-item :label="t('options.fullscreen')">
            <n-tooltip trigger="hover">
              <template #trigger>
                <n-switch v-model:value="options.fullscreen" />
              </template>
              {{ t('options.fullscreenHint') }}
            </n-tooltip>
          </n-form-item>
          <n-form-item :label="t('options.alwaysOnTop')">
            <n-tooltip trigger="hover">
              <template #trigger>
                <n-switch v-model:value="options.alwaysOnTop" />
              </template>
              {{ t('options.alwaysOnTopHint') }}
            </n-tooltip>
          </n-form-item>
          <n-form-item :label="t('options.borderless')">
            <n-tooltip trigger="hover">
              <template #trigger>
                <n-switch v-model:value="options.borderless" />
              </template>
              {{ t('options.borderlessHint') }}
            </n-tooltip>
          </n-form-item>
          <n-form-item :label="t('options.windowTitle')">
            <n-tooltip trigger="hover">
              <template #trigger>
                <n-input
                  v-model:value="options.windowTitle"
                  :placeholder="t('options.windowTitlePlaceholder')"
                  clearable
                />
              </template>
              {{ t('options.windowTitleHint') }}
            </n-tooltip>
          </n-form-item>
        </n-form>
      </n-tab-pane>

      <!-- Device Tab -->
      <n-tab-pane name="device">
        <template #tab>
          <div class="tab-label">
            <n-icon :size="16"><PhonePortraitOutline /></n-icon>
            <span>{{ t('options.device') }}</span>
          </div>
        </template>
        <n-form label-placement="left" label-width="120" size="small" class="options-form">
          <n-form-item :label="t('options.turnScreenOff')">
            <n-tooltip trigger="hover">
              <template #trigger>
                <n-switch v-model:value="options.turnScreenOff" />
              </template>
              {{ t('options.turnScreenOffHint') }}
            </n-tooltip>
          </n-form-item>
          <n-form-item :label="t('options.stayAwake')">
            <n-tooltip trigger="hover">
              <template #trigger>
                <n-switch v-model:value="options.stayAwake" />
              </template>
              {{ t('options.stayAwakeHint') }}
            </n-tooltip>
          </n-form-item>
          <n-form-item :label="t('options.showTouches')">
            <n-tooltip trigger="hover">
              <template #trigger>
                <n-switch v-model:value="options.showTouches" />
              </template>
              {{ t('options.showTouchesHint') }}
            </n-tooltip>
          </n-form-item>
        </n-form>
      </n-tab-pane>

      <!-- Input Tab -->
      <n-tab-pane name="input">
        <template #tab>
          <div class="tab-label">
            <n-icon :size="16"><GameControllerOutline /></n-icon>
            <span>{{ t('options.input') }}</span>
          </div>
        </template>
        <n-form label-placement="left" label-width="120" size="small" class="options-form">
          <n-form-item :label="t('options.keyboardMode')">
            <n-tooltip trigger="hover">
              <template #trigger>
                <n-select
                  v-model:value="options.keyboard"
                  :options="keyboardOptions"
                  :placeholder="t('options.sdkDefault')"
                  clearable
                />
              </template>
              {{ t('options.keyboardModeHint') }}
            </n-tooltip>
          </n-form-item>
          <n-form-item :label="t('options.mouseMode')">
            <n-tooltip trigger="hover">
              <template #trigger>
                <n-select
                  v-model:value="options.mouse"
                  :options="mouseOptions"
                  :placeholder="t('options.sdkDefault')"
                  clearable
                />
              </template>
              {{ t('options.mouseModeHint') }}
            </n-tooltip>
          </n-form-item>
          <n-form-item :label="t('options.shortcutMod')">
            <n-tooltip trigger="hover">
              <template #trigger>
                <n-select
                  v-model:value="options.shortcutMod"
                  :options="shortcutModOptions"
                  placeholder="Alt"
                  clearable
                />
              </template>
              {{ t('options.shortcutModHint') }}
            </n-tooltip>
          </n-form-item>
        </n-form>
      </n-tab-pane>
    </n-tabs>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { NTabs, NTabPane, NForm, NFormItem, NSelect, NInputNumber, NInput, NSwitch, NTooltip, NIcon } from 'naive-ui';
import { 
  WifiOutline,
  VideocamOutline,
  VolumeHighOutline,
  RadioOutline,
  BrowsersOutline,
  PhonePortraitOutline,
  GameControllerOutline
} from '@vicons/ionicons5';
import type { SessionOptions } from '../types/session';

const { t } = useI18n();

const options = defineModel<SessionOptions>({ default: () => ({}) });

// Audio enabled computed
const audioEnabled = computed({
  get: () => options.value.audio !== false,
  set: (val: boolean) => {
    options.value.audio = val;
    if (!val) {
      options.value.audioCodec = undefined;
      options.value.audioBitrate = undefined;
      options.value.audioSource = undefined;
    }
  }
});

// Max size handling (convert number to select value)
const maxSizeValue = computed({
  get: () => options.value.maxSize?.toString() ?? null,
  set: () => {} // Handled by updateMaxSize
});
function updateMaxSize(val: string | null) {
  options.value.maxSize = val ? parseInt(val, 10) : undefined;
}

// Max FPS handling
const maxFpsValue = computed({
  get: () => options.value.maxFps?.toString() ?? null,
  set: () => {}
});
function updateMaxFps(val: string | null) {
  options.value.maxFps = val ? parseInt(val, 10) : undefined;
}

// Select options
const videoCodecOptions = [
  { label: 'Auto', value: 'auto' },
  { label: 'H.264', value: 'h264' },
  { label: 'H.265 (HEVC)', value: 'h265' },
];

const audioCodecOptions = [
  { label: 'Auto', value: 'auto' },
  { label: 'OPUS', value: 'opus' },
  { label: 'AAC', value: 'aac' },
  { label: 'FLAC', value: 'flac' },
  { label: 'Raw', value: 'raw' },
];

const audioSourceOptions = [
  { label: 'Auto', value: 'auto' },
  { label: 'Output', value: 'output' },
  { label: 'Mic', value: 'mic' },
];

const maxSizeOptions = [
  { label: 'No limit', value: '0' },
  { label: '480p (854x480)', value: '480' },
  { label: '720p (1280x720)', value: '720' },
  { label: '1080p (1920x1080)', value: '1080' },
  { label: '1440p (2560x1440)', value: '1440' },
  { label: '4K (3840x2160)', value: '2160' },
];

const maxFpsOptions = [
  { label: 'No limit', value: '0' },
  { label: '15 FPS', value: '15' },
  { label: '30 FPS', value: '30' },
  { label: '60 FPS', value: '60' },
  { label: '90 FPS', value: '90' },
  { label: '120 FPS', value: '120' },
];

const recordFormatOptions = [
  { label: 'Auto detect', value: 'auto' },
  { label: 'MP4', value: 'mp4' },
  { label: 'MKV', value: 'mkv' },
  { label: 'WebM', value: 'webm' },
];

const keyboardOptions = [
  { label: 'SDK (Default)', value: 'sdk' },
  { label: 'AOA', value: 'aoa' },
  { label: 'Disabled', value: 'disabled' },
];

const mouseOptions = [
  { label: 'SDK (Default)', value: 'sdk' },
  { label: 'AOA', value: 'aoa' },
  { label: 'Disabled', value: 'disabled' },
];

const shortcutModOptions = [
  { label: 'Alt (Default)', value: 'alt' },
  { label: 'Super/Win', value: 'super' },
  { label: 'Alt+Super', value: 'alt,super' },
  { label: 'Ctrl', value: 'ctrl' },
];
</script>

<style scoped>
.session-options-form {
  min-height: 200px;
}

.options-form {
  padding: 12px 4px;
}

.tab-label {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
}

:deep(.n-form-item) {
  margin-bottom: 14px;
}

:deep(.n-form-item-label) {
  font-size: 13px;
  font-weight: 500;
}

:deep(.n-tabs-tab) {
  padding: 8px 12px !important;
}
</style>
