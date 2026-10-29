<script setup lang="ts">
import { Disc3, Play, Square } from "@lucide/vue";
import { Button } from "@/components/ui/button";
import { useLaunch } from "@/composables/useLaunch";
import { useI18n } from "@/i18n";

const { t } = useI18n();
const { running, launch, stop } = useLaunch();
</script>

<template>
  <div class="relative h-full">
    <div class="page-center select-none flex-col gap-3">
      <Disc3 class="size-16 text-muted-foreground/40" aria-hidden="true" />
      <h2 class="text-2xl font-semibold tracking-tight">Open Chunithm Launcher</h2>
    </div>

    <div class="absolute bottom-8 right-8 flex items-center gap-3">
      <Button
        v-if="running"
        variant="outline"
        class="h-12! rounded-2xl! px-6! text-base!"
        @click="stop"
      >
        <Square class="size-5" aria-hidden="true" />
        {{ t("home.stop") }}
      </Button>
      <Button
        class="h-14! gap-3! rounded-2xl! px-10! text-lg! font-semibold shadow-lg shadow-primary/25"
        :disabled="running"
        @click="launch"
      >
        <Play class="size-6" aria-hidden="true" />
        {{ t("home.launch") }}
      </Button>
    </div>
  </div>
</template>
