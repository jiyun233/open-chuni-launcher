<script setup lang="ts">
import { computed } from "vue";
import {
  AppWindow,
  Coins,
  Contact,
  Folder,
  FolderOpen,
  Frame,
  Gauge,
  HardDrive,
  IdCard,
  Keyboard,
  Maximize2,
  Monitor,
  Network,
  Server,
  Wrench,
} from "@lucide/vue";
import SettingCard from "@/components/settings/SettingCard.vue";
import SettingKey from "@/components/settings/controls/SettingKey.vue";
import SettingPath from "@/components/settings/controls/SettingPath.vue";
import SettingSelect from "@/components/settings/controls/SettingSelect.vue";
import SettingSwitch from "@/components/settings/controls/SettingSwitch.vue";
import { Input } from "@/components/ui/input";
import { useSettings } from "@/composables/useSettings";
import { useI18n } from "@/i18n";
import { SERVER_PRESETS } from "@/lib/servers";

import { DISPLAY_MODE_PRESETS, WINDOW_MODE_PRESETS } from "@/lib/segatools";

const { t } = useI18n();
const { settings } = useSettings();

const serverPresetOptions = computed(() => [
  ...SERVER_PRESETS.map((preset) => ({ value: preset.id, label: preset.label })),
  { value: "custom", label: t("config.server.custom") },
]);

const displayModeOptions = computed(() => [
  ...DISPLAY_MODE_PRESETS.map((preset) => ({
    value: preset.id,
    label: t(`config.display.${preset.id}`),
  })),
  { value: "custom", label: t("config.display.custom") },
]);

const windowModeOptions = computed(() => [
  ...WINDOW_MODE_PRESETS.map((preset) => ({
    value: preset.id,
    label: t(`config.window.${preset.id}`),
  })),
  { value: "custom", label: t("config.window.custom") },
]);

/** 0/1 二值开关：值 0/1，label 分别为 off/on 的描述 */
function binaryOptions(offKey: string, onKey: string) {
  return [
    { value: "0", label: t(offKey) },
    { value: "1", label: t(onKey) },
  ];
}

// 0/1 开关在设置里存为数字，选择器需要字符串，这里做双向转换。
const dipsw1 = computed<string>({
  get: () => String(settings.value.gpio.dipsw1),
  set: (value) => {
    settings.value.gpio.dipsw1 = Number(value);
  },
});
const dipsw2 = computed<string>({
  get: () => String(settings.value.gpio.dipsw2),
  set: (value) => {
    settings.value.gpio.dipsw2 = Number(value);
  },
});
const dipsw3 = computed<string>({
  get: () => String(settings.value.gpio.dipsw3),
  set: (value) => {
    settings.value.gpio.dipsw3 = Number(value);
  },
});
const windowed = computed<string>({
  get: () => String(settings.value.gfx.windowed),
  set: (value) => {
    settings.value.gfx.windowed = Number(value);
  },
});
const framed = computed<string>({
  get: () => String(settings.value.gfx.framed),
  set: (value) => {
    settings.value.gfx.framed = Number(value);
  },
});
</script>

<template>
  <div class="settings-page">
    <!-- 服务器 -->
    <section class="settings-section" aria-labelledby="config-server-title">
      <h2 id="config-server-title" class="settings-section-title">
        {{ t("config.section.server") }}
      </h2>

      <div class="config-server-card">
        <div class="config-server-header">
          <div class="config-server-icon">
            <Server class="size-5" aria-hidden="true" />
          </div>
          <div>
            <h3 class="text-base font-semibold">{{ t("config.server.title") }}</h3>
            <p class="text-xs text-muted-foreground">{{ t("config.server.description") }}</p>
          </div>
        </div>

        <div class="config-field">
          <p class="config-field-label">{{ t("config.server.preset") }}</p>
          <SettingSelect v-model="settings.server.preset" :options="serverPresetOptions" />
        </div>

        <template v-if="settings.server.preset === 'custom'">
          <div class="config-field">
            <label class="config-field-label" for="config-dns">{{ t("config.server.dns") }}</label>
            <Input
              id="config-dns"
              v-model="settings.server.customDns"
              placeholder="dns server host"
            />
          </div>
          <div class="config-field">
            <label class="config-field-label" for="config-aimedb">{{ t("config.server.aimedb") }}</label>
            <Input
              id="config-aimedb"
              v-model="settings.server.customAimeDb"
              placeholder="aimeDB server host"
            />
            <p class="config-field-hint">{{ t("config.server.aimedbHint") }}</p>
          </div>
        </template>

        <div class="config-divider" role="presentation" />

        <div class="config-field">
          <label class="config-field-label" for="config-keychip">{{ t("config.keychip.title") }}</label>
          <Input
            id="config-keychip"
            v-model="settings.server.keychip"
            class="font-mono"
            :placeholder="t('config.keychip.placeholder')"
          />
        </div>
      </div>
    </section>

    <!-- 显示与窗口 -->
    <section class="settings-section" aria-labelledby="config-display-title">
      <h2 id="config-display-title" class="settings-section-title">
        {{ t("config.section.display") }}
      </h2>

      <SettingCard
        :icon="Monitor"
        :title="t('config.display.title')"
        :description="t('config.display.description')"
      >
        <SettingSelect v-model="settings.gpio.preset" :options="displayModeOptions" />
      </SettingCard>

      <template v-if="settings.gpio.preset === 'custom'">
        <SettingCard
          :icon="Network"
          :title="t('config.display.dipsw1.title')"
          :description="t('config.display.dipsw1.description')"
        >
          <SettingSelect
            v-model="dipsw1"
            :options="binaryOptions('config.display.dipsw1.client', 'config.display.dipsw1.server')"
          />
        </SettingCard>
        <SettingCard
          :icon="Gauge"
          :title="t('config.display.dipsw2.title')"
          :description="t('config.display.dipsw2.description')"
        >
          <SettingSelect
            v-model="dipsw2"
            :options="binaryOptions('config.display.dipsw2.fps120', 'config.display.dipsw2.fps60')"
          />
        </SettingCard>
        <SettingCard
          :icon="IdCard"
          :title="t('config.display.dipsw3.title')"
          :description="t('config.display.dipsw3.description')"
        >
          <SettingSelect
            v-model="dipsw3"
            :options="binaryOptions('config.display.dipsw3.sp', 'config.display.dipsw3.cvt')"
          />
        </SettingCard>
      </template>

      <SettingCard
        :icon="AppWindow"
        :title="t('config.window.title')"
        :description="t('config.window.description')"
      >
        <SettingSelect v-model="settings.gfx.preset" :options="windowModeOptions" />
      </SettingCard>

      <template v-if="settings.gfx.preset === 'custom'">
        <SettingCard
          :icon="Maximize2"
          :title="t('config.window.windowed.title')"
          :description="t('config.window.windowed.description')"
        >
          <SettingSelect
            v-model="windowed"
            :options="binaryOptions('config.window.windowed.off', 'config.window.windowed.on')"
          />
        </SettingCard>
        <SettingCard
          :icon="Frame"
          :title="t('config.window.framed.title')"
          :description="t('config.window.framed.description')"
        >
          <SettingSelect
            v-model="framed"
            :options="binaryOptions('config.window.framed.off', 'config.window.framed.on')"
          />
        </SettingCard>
      </template>
    </section>

    <!-- 目录与读卡器 -->
    <section class="settings-section" aria-labelledby="config-storage-title">
      <h2 id="config-storage-title" class="settings-section-title">
        {{ t("config.section.storage") }}
      </h2>

      <SettingCard
        :icon="FolderOpen"
        :title="t('config.vfs.option.title')"
        :description="t('config.vfs.option.description')"
      >
        <SettingPath
          v-model="settings.vfs.option"
          :placeholder="t('config.vfs.placeholder')"
          :browse-label="t('config.vfs.browse')"
          editable
        />
      </SettingCard>

      <SettingCard
        :icon="Contact"
        :title="t('config.aime.title')"
        :description="t('config.aime.description')"
      >
        <SettingSwitch v-model="settings.aime.enable" />
      </SettingCard>

      <SettingCard
        :icon="HardDrive"
        :title="t('config.vfs.amfs.title')"
        :description="t('config.vfs.amfs.description')"
      >
        <SettingPath
          v-model="settings.vfs.amfs"
          :placeholder="t('config.vfs.placeholder')"
          :browse-label="t('config.vfs.browse')"
          editable
        />
      </SettingCard>

      <SettingCard
        :icon="Folder"
        :title="t('config.vfs.appdata.title')"
        :description="t('config.vfs.appdata.description')"
      >
        <SettingPath
          v-model="settings.vfs.appdata"
          :placeholder="t('config.vfs.placeholder')"
          :browse-label="t('config.vfs.browse')"
          editable
        />
      </SettingCard>
    </section>

    <!-- 按键 -->
    <section class="settings-section" aria-labelledby="config-keys-title">
      <h2 id="config-keys-title" class="settings-section-title">
        {{ t("config.section.keys") }}
      </h2>

      <SettingCard
        :icon="Keyboard"
        :title="t('config.io3.test.title')"
        :description="t('config.io3.test.description')"
      >
        <SettingKey
          v-model="settings.io3.test"
          :aria-label="t('config.io3.test.title')"
        />
      </SettingCard>
      <SettingCard
        :icon="Wrench"
        :title="t('config.io3.service.title')"
        :description="t('config.io3.service.description')"
      >
        <SettingKey
          v-model="settings.io3.service"
          :aria-label="t('config.io3.service.title')"
        />
      </SettingCard>
      <SettingCard
        :icon="Coins"
        :title="t('config.io3.coin.title')"
        :description="t('config.io3.coin.description')"
      >
        <SettingKey
          v-model="settings.io3.coin"
          :aria-label="t('config.io3.coin.title')"
        />
      </SettingCard>
    </section>
  </div>
</template>
