<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Gauge, Info, Maximize2, Monitor, MonitorCog } from "@lucide/vue";
import SettingCard from "@/components/settings/SettingCard.vue";
import SettingSelect from "@/components/settings/controls/SettingSelect.vue";
import SettingSwitch from "@/components/settings/controls/SettingSwitch.vue";
import { Button } from "@/components/ui/button";
import { useSettings } from "@/composables/useSettings";
import { useI18n } from "@/i18n";
import type { MonitorInfo } from "@/lib/display";

const { t } = useI18n();
const { settings } = useSettings();

const isTauri = "__TAURI_INTERNALS__" in window;
const monitors = ref<MonitorInfo[]>([]);
const loading = ref(false);
const loadError = ref(false);

async function refresh() {
  loading.value = true;
  loadError.value = false;
  try {
    monitors.value = await invoke<MonitorInfo[]>("list_monitors");
    ensureValidSelection();
  } catch {
    loadError.value = true;
  } finally {
    loading.value = false;
  }
}

onMounted(() => {
  if (isTauri) refresh();
});

const selectedMonitor = computed(() =>
  monitors.value.find((monitor) => monitor.index === settings.value.display.monitor),
);

/**
 * 校正显示器编号与目标模式：
 * 编号失效回落主显示器；目标分辨率/刷新率不被支持时回落到该显示器当前模式。
 */
function ensureValidSelection() {
  const { display } = settings.value;
  if (monitors.value.length === 0) return;
  if (!monitors.value.some((monitor) => monitor.index === display.monitor)) {
    display.monitor = (monitors.value.find((monitor) => monitor.isPrimary) ?? monitors.value[0])
      .index;
  }
  const monitor = monitors.value.find((item) => item.index === display.monitor);
  if (!monitor) return;
  const resolutionSupported = monitor.modes.some(
    (mode) => mode.width === display.width && mode.height === display.height,
  );
  if (!resolutionSupported) {
    display.width = monitor.width;
    display.height = monitor.height;
    display.refreshRate = monitor.refreshRate;
    return;
  }
  const refreshSupported = monitor.modes.some(
    (mode) =>
      mode.width === display.width &&
      mode.height === display.height &&
      mode.refreshRate === display.refreshRate,
  );
  if (!refreshSupported) {
    display.refreshRate = monitor.modes
      .filter((mode) => mode.width === display.width && mode.height === display.height)
      .reduce((best, mode) => Math.max(best, mode.refreshRate), 0);
  }
}

watch(() => settings.value.display.monitor, ensureValidSelection);

const monitorValue = computed<string>({
  get: () => String(settings.value.display.monitor),
  set: (value) => {
    settings.value.display.monitor = Number(value);
  },
});

const monitorOptions = computed(() =>
  monitors.value.map((monitor) => ({
    value: String(monitor.index),
    label: monitor.isPrimary
      ? `#${monitor.index} ${monitor.name} · ${t("display.monitor.primary")}`
      : `#${monitor.index} ${monitor.name}`,
  })),
);

const resolutionOptions = computed(() => {
  const seen = new Set<string>();
  const options: Array<{ value: string; label: string }> = [];
  for (const mode of selectedMonitor.value?.modes ?? []) {
    const key = `${mode.width}x${mode.height}`;
    if (seen.has(key)) continue;
    seen.add(key);
    options.push({ value: key, label: `${mode.width} × ${mode.height}` });
  }
  return options;
});

const resolutionValue = computed<string>({
  get: () => `${settings.value.display.width}x${settings.value.display.height}`,
  set: (value) => {
    const [width, height] = value.split("x").map(Number);
    const display = settings.value.display;
    if (display.width === width && display.height === height) return;
    display.width = width;
    display.height = height;
    // 切换分辨率后取该分辨率下可用的最高刷新率
    display.refreshRate =
      selectedMonitor.value?.modes
        .filter((mode) => mode.width === width && mode.height === height)
        .reduce((best, mode) => Math.max(best, mode.refreshRate), 0) ?? 0;
  },
});

const refreshOptions = computed(() => {
  const display = settings.value.display;
  const rates = new Set<number>();
  for (const mode of selectedMonitor.value?.modes ?? []) {
    if (mode.width === display.width && mode.height === display.height) {
      rates.add(mode.refreshRate);
    }
  }
  return [...rates]
    .sort((a, b) => b - a)
    .map((rate) => ({ value: String(rate), label: `${rate} Hz` }));
});

const refreshValue = computed<string>({
  get: () => String(settings.value.display.refreshRate),
  set: (value) => {
    settings.value.display.refreshRate = Number(value);
  },
});

const currentModeLabel = computed(() => {
  const monitor = selectedMonitor.value;
  if (!monitor) return "";
  return t("display.info.hint", {
    width: monitor.width,
    height: monitor.height,
    refresh: monitor.refreshRate,
  });
});
</script>

<template>
  <div class="settings-page">
    <p v-if="!isTauri" class="py-8 text-center text-sm text-muted-foreground">
      {{ t("display.desktopOnly") }}
    </p>
    <p v-else-if="loading" class="py-8 text-center text-sm text-muted-foreground">
      {{ t("display.loading") }}
    </p>
    <div v-else-if="loadError" class="flex flex-col items-center gap-3 py-8">
      <p class="text-sm text-destructive">{{ t("display.loadError") }}</p>
      <Button variant="outline" size="sm" @click="refresh">{{ t("display.retry") }}</Button>
    </div>
    <p v-else-if="monitors.length === 0" class="py-8 text-center text-sm text-muted-foreground">
      {{ t("display.empty") }}
    </p>

    <template v-else>
      <!-- 显示器 -->
      <section class="settings-section" aria-labelledby="display-monitor-title">
        <h2 id="display-monitor-title" class="settings-section-title">
          {{ t("display.section.monitor") }}
        </h2>

        <SettingCard
          :icon="Monitor"
          :title="t('display.monitor.title')"
          :description="t('display.monitor.description')"
        >
          <SettingSelect
            v-model="monitorValue"
            :options="monitorOptions"
            class="w-64 [&>span]:min-w-0 [&>span]:truncate"
          />
        </SettingCard>

        <SettingCard
          :icon="Info"
          :title="t('display.info.title')"
          :description="t('display.info.device', { device: selectedMonitor?.device ?? '' })"
        >
          <p class="text-sm tabular-nums text-muted-foreground">{{ currentModeLabel }}</p>
        </SettingCard>
      </section>

      <!-- 启动时切换显示模式 -->
      <section class="settings-section" aria-labelledby="display-adjust-title">
        <h2 id="display-adjust-title" class="settings-section-title">
          {{ t("display.section.adjust") }}
        </h2>

        <SettingCard
          :icon="MonitorCog"
          :title="t('display.adjust.title')"
          :description="t('display.adjust.description')"
        >
          <SettingSwitch v-model="settings.display.enabled" />
        </SettingCard>

        <template v-if="settings.display.enabled">
          <SettingCard
            :icon="Maximize2"
            :title="t('display.adjust.resolution.title')"
            :description="t('display.adjust.resolution.description')"
          >
            <SettingSelect v-model="resolutionValue" :options="resolutionOptions" />
          </SettingCard>
          <SettingCard
            :icon="Gauge"
            :title="t('display.adjust.refresh.title')"
            :description="t('display.adjust.refresh.description')"
          >
            <SettingSelect v-model="refreshValue" :options="refreshOptions" />
          </SettingCard>
        </template>
      </section>
    </template>
  </div>
</template>
