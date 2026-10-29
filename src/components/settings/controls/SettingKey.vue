<script setup lang="ts">
import { ref } from "vue";
import { Keyboard } from "@lucide/vue";
import { useI18n } from "@/i18n";

const model = defineModel<string>({ required: true });

defineProps<{ ariaLabel?: string }>();

const { t } = useI18n();
const button = ref<HTMLButtonElement | null>(null);
const recording = ref(false);

function toVirtualKeyCode(code: string): number | undefined {
  // 字母键 KeyA-KeyZ → 0x41-0x5A
  const letter = /^Key([A-Z])$/.exec(code);
  if (letter) return 0x41 + (letter[1].charCodeAt(0) - 0x41);

  // 主键盘数字 Digit0-Digit9 → 0x30-0x39
  const digit = /^Digit([0-9])$/.exec(code);
  if (digit) return 0x30 + (digit[1].charCodeAt(0) - 0x30);

  // 小键盘数字 Numpad0-Numpad9 → 0x60-0x69
  const numpad = /^Numpad([0-9])$/.exec(code);
  if (numpad) return 0x60 + (numpad[1].charCodeAt(0) - 0x30);

  // 功能键 F1-F24 → 0x70-0x87
  const fn = /^F(\d{1,2})$/.exec(code);
  if (fn) {
    const n = Number(fn[1]);
    if (n >= 1 && n <= 24) return 0x70 + (n - 1);
  }

  const special: Record<string, number> = {
    Space: 0x20,
    Enter: 0x0d,
    NumpadEnter: 0x0d,
    Tab: 0x09,
    Backspace: 0x08,
    Delete: 0x2e,
    Insert: 0x2d,
    Home: 0x24,
    End: 0x23,
    PageUp: 0x21,
    PageDown: 0x22,
    ArrowUp: 0x26,
    ArrowDown: 0x28,
    ArrowLeft: 0x25,
    ArrowRight: 0x27,
    Minus: 0xbd,
    Equal: 0xbb,
    BracketLeft: 0xdb,
    BracketRight: 0xdd,
    Backslash: 0xdc,
    Semicolon: 0xba,
    Quote: 0xde,
    Comma: 0xbc,
    Period: 0xbe,
    Slash: 0xbf,
    Backquote: 0xc0,
    NumpadMultiply: 0x6a,
    NumpadAdd: 0x6b,
    NumpadSubtract: 0x6d,
    NumpadDecimal: 0x6e,
    NumpadDivide: 0x6f,
    CapsLock: 0x14,
    NumLock: 0x90,
    ScrollLock: 0x91,
  };
  return special[code];
}

function capture(event: KeyboardEvent) {
  if (event.key === "Escape") {
    recording.value = false;
    button.value?.blur();
    return;
  }

  if (["Shift", "Control", "Alt", "Meta"].includes(event.key)) return;
  const code = toVirtualKeyCode(event.code);
  if (code) {
    model.value = `${event.key} (0x${code.toString(16).toUpperCase().padStart(2, "0")})`;
    recording.value = false;
    button.value?.blur();
  }
}
</script>

<template>
  <button
    ref="button"
    type="button"
    class="key-capture"
    :data-recording="recording"
    :aria-label="ariaLabel"
    @focus="recording = true"
    @blur="recording = false"
    @keydown.prevent="capture"
  >
    <Keyboard class="size-4 shrink-0 text-muted-foreground" aria-hidden="true" />
    <span class="truncate font-mono">
      {{ recording ? t("config.keys.pressHint") : model || "—" }}
    </span>
  </button>
</template>
