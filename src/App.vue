<script setup lang="ts">
import { computed, ref } from "vue";
import type { Component } from "vue";
import AppSidebar from "@/components/layout/AppSidebar.vue";
import AppNotifications from "@/components/layout/AppNotifications.vue";
import { TooltipProvider } from "@/components/ui/tooltip";
import { useI18n } from "@/i18n";
import ConfigPage from "@/pages/ConfigPage.vue";
import DisplayPage from "@/pages/DisplayPage.vue";
import HomePage from "@/pages/HomePage.vue";
import PluginsPage from "@/pages/PluginsPage.vue";
import LogsPage from "@/pages/LogsPage.vue";
import SettingsPage from "@/pages/SettingsPage.vue";
import { navSections } from "@/lib/navigation";

const { t } = useI18n();

const pages: Record<string, Component> = {
  home: HomePage,
  config: ConfigPage,
  display: DisplayPage,
  plugins: PluginsPage,
  logs: LogsPage,
  settings: SettingsPage,
};

const activeSection = ref(navSections[0].id);
const activeSectionMeta = computed(() =>
  navSections.find((section) => section.id === activeSection.value),
);
const activePage = computed(() => pages[activeSection.value]);
</script>

<template>
  <TooltipProvider>
    <div class="app-shell">
      <AppSidebar v-model="activeSection" />

      <main class="flex min-w-0 flex-1 flex-col">
        <header class="app-header">
          <h1 class="app-header-title">
            {{ activeSectionMeta ? t(activeSectionMeta.labelKey) : "" }}
          </h1>
        </header>

        <div class="app-content">
          <Transition name="page" mode="out-in" appear>
            <component :is="activePage" :key="activeSection" />
          </Transition>
        </div>
      </main>
    </div>
    <AppNotifications />
  </TooltipProvider>
</template>
