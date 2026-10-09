import type { ServerConfig } from "@/lib/servers";
import { resolveServerDns } from "@/lib/servers";
import type { DisplayConfig } from "@/lib/display";

export interface VfsConfig {
  option: string;
  amfs: string;
  appdata: string;
}

export const DEFAULT_VFS: VfsConfig = {
  option: "option",
  amfs: "amfs",
  appdata: "appdata",
};

export type DisplayModeId = "fps120" | "fps60" | "custom";

export interface DisplayModePreset {
  id: Exclude<DisplayModeId, "custom">;
  dipsw1: number;
  dipsw2: number;
  dipsw3: number;
}

export const DISPLAY_MODE_PRESETS: DisplayModePreset[] = [
  { id: "fps120", dipsw1: 1, dipsw2: 0, dipsw3: 0 },
  { id: "fps60", dipsw1: 1, dipsw2: 1, dipsw3: 1 },
];

export interface GpioConfig {
  preset: DisplayModeId;
  dipsw1: number;
  dipsw2: number;
  dipsw3: number;
}

export const DEFAULT_GPIO: GpioConfig = {
  preset: "fps120",
  dipsw1: 1,
  dipsw2: 0,
  dipsw3: 0,
};

export function resolveGpio(config: GpioConfig): {
  dipsw1: number;
  dipsw2: number;
  dipsw3: number;
} {
  const preset = DISPLAY_MODE_PRESETS.find((item) => item.id === config.preset);
  if (config.preset !== "custom" && preset) {
    return { dipsw1: preset.dipsw1, dipsw2: preset.dipsw2, dipsw3: preset.dipsw3 };
  }
  return { dipsw1: config.dipsw1, dipsw2: config.dipsw2, dipsw3: config.dipsw3 };
}

export type WindowModeId = "fullscreen" | "borderless" | "windowed" | "custom";

export interface WindowModePreset {
  id: Exclude<WindowModeId, "custom">;
  windowed: number;
  framed: number;
}

export const WINDOW_MODE_PRESETS: WindowModePreset[] = [
  { id: "fullscreen", windowed: 0, framed: 0 },
  { id: "borderless", windowed: 1, framed: 0 },
  { id: "windowed", windowed: 1, framed: 1 },
];

export interface GfxConfig {
  preset: WindowModeId;
  windowed: number;
  framed: number;
}

export const DEFAULT_GFX: GfxConfig = {
  preset: "fullscreen",
  windowed: 0,
  framed: 0,
};

export function resolveGfx(config: GfxConfig): {
  windowed: number;
  framed: number;
} {
  const preset = WINDOW_MODE_PRESETS.find((item) => item.id === config.preset);
  if (config.preset !== "custom" && preset) {
    return { windowed: preset.windowed, framed: preset.framed };
  }
  return { windowed: config.windowed, framed: config.framed };
}

export interface AimeConfig {
  enable: boolean;
}

export const DEFAULT_AIME: AimeConfig = { enable: true };

export interface Io3Config {
  test: string;
  service: string;
  coin: string;
}

export const DEFAULT_IO3: Io3Config = {
  test: "0x31",
  service: "0x32",
  coin: "0x33",
};

export interface SegatoolsPatch {
  dns?: { default: string; aimedb?: string };
  keychip?: { id: string };
  vfs?: { option: string; amfs: string; appdata: string };
  gpio?: { dipsw1: number; dipsw2: number; dipsw3: number };
  gfx?: { windowed: number; framed: number; monitor: number };
  aime?: { enable: boolean };
  io3?: { test: string; service: string; coin: string };
}

export function buildSegatoolsPatch(
  server: ServerConfig,
  vfs: VfsConfig,
  gpio: GpioConfig,
  gfx: GfxConfig,
  aime: AimeConfig,
  io3: Io3Config,
  display: DisplayConfig,
): SegatoolsPatch {
  const dns = resolveServerDns(server);
  const gpioValues = resolveGpio(gpio);
  const gfxValues = resolveGfx(gfx);

  return {
    dns: {
      default: dns.dnsDefault,
      ...(dns.dnsAimeDb ? { aimedb: dns.dnsAimeDb } : {}),
    },
    keychip: { id: server.keychip.trim() },
    vfs: {
      option: vfs.option.trim(),
      amfs: vfs.amfs.trim(),
      appdata: vfs.appdata.trim(),
    },
    gpio: gpioValues,
    // 显示器编号统一由显示器页的 display.monitor 提供
    gfx: { ...gfxValues, monitor: display.monitor },
    aime: { enable: aime.enable },
    io3: {
      test: io3.test.trim(),
      service: io3.service.trim(),
      coin: io3.coin.trim(),
    },
  };
}
