<script setup lang="ts">
import { computed, onBeforeUnmount, ref, watch } from "vue";
import { ChevronsLeft, ChevronsRight, Disc3 } from "@lucide/vue";
import { Button } from "@/components/ui/button";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { useI18n } from "@/i18n";
import { navSections } from "@/lib/navigation";
import { cn } from "@/lib/utils";

const { t } = useI18n();

const active = defineModel<string>({ required: true });

const COLLAPSE_KEY = "sidebar-collapsed";
const NARROW_QUERY = "(max-width: 639.98px)";

const manualCollapsed = ref(localStorage.getItem(COLLAPSE_KEY) === "1");
const mediaQuery = window.matchMedia(NARROW_QUERY);
const narrow = ref(mediaQuery.matches);

const collapsed = computed(() => manualCollapsed.value || narrow.value);

const sidebarItemClass = computed(() =>
  collapsed.value
    ? "w-8 justify-center gap-0 px-0 text-sidebar-foreground/80 hover:bg-sidebar-accent hover:text-sidebar-foreground"
    : "w-full justify-start gap-2.5 px-2 text-sidebar-foreground/80 hover:bg-sidebar-accent hover:text-sidebar-foreground",
);

function toggleCollapsed() {
  manualCollapsed.value = !manualCollapsed.value;
}

watch(manualCollapsed, (value) => {
  localStorage.setItem(COLLAPSE_KEY, value ? "1" : "0");
});

function onNarrowChange(event: MediaQueryListEvent) {
  narrow.value = event.matches;
}

mediaQuery.addEventListener("change", onNarrowChange);
onBeforeUnmount(() => mediaQuery.removeEventListener("change", onNarrowChange));
</script>

<template>
  <aside
    :aria-label="t('sidebar.navLabel')"
    class="app-sidebar"
    :data-collapsed="collapsed || undefined"
  >
    <div class="flex h-12 shrink-0 items-center gap-2 px-3">
      <div class="sidebar-brand-icon">
        <Disc3 class="size-4.5" aria-hidden="true" />
      </div>
      <span class="sidebar-label text-sm font-semibold tracking-tight">
        Open Chunithm Launcher
      </span>
    </div>

    <nav class="sidebar-nav">
      <Tooltip v-for="item in navSections" :key="item.id" :disabled="!collapsed">
        <TooltipTrigger as-child>
          <Button
            variant="ghost"
            :class="cn(sidebarItemClass, active === item.id && 'bg-sidebar-accent text-sidebar-accent-foreground')"
            :aria-label="t(item.labelKey)"
            :aria-current="active === item.id ? 'page' : undefined"
            @click="active = item.id"
          >
            <component :is="item.icon" class="size-4.5 shrink-0" aria-hidden="true" />
            <span class="sidebar-label">{{ t(item.labelKey) }}</span>
          </Button>
        </TooltipTrigger>
        <TooltipContent side="right">{{ t(item.labelKey) }}</TooltipContent>
      </Tooltip>
    </nav>

    <div class="sidebar-footer">
      <Tooltip :disabled="!collapsed">
        <TooltipTrigger as-child>
          <Button
            variant="ghost"
            :class="sidebarItemClass"
            :aria-label="collapsed ? t('sidebar.expand') : t('sidebar.collapse')"
            @click="toggleCollapsed"
          >
            <ChevronsRight v-if="collapsed" class="size-4.5 shrink-0" aria-hidden="true" />
            <ChevronsLeft v-else class="size-4.5 shrink-0" aria-hidden="true" />
            <span class="sidebar-label">{{ t("sidebar.collapse") }}</span>
          </Button>
        </TooltipTrigger>
        <TooltipContent side="right">{{ t("sidebar.expand") }}</TooltipContent>
      </Tooltip>
    </div>
  </aside>
</template>
