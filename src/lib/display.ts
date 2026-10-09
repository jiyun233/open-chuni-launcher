/** 显示器信息（来自后端 list_monitors 命令） */
export interface DisplayModeInfo {
  width: number;
  height: number;
  refreshRate: number;
}

export interface MonitorInfo {
  /** 对应 segatools [gfx] monitor 的编号 */
  index: number;
  /** 设备名（如 \\.\DISPLAY1） */
  device: string;
  /** 显示器友好名称 */
  name: string;
  isPrimary: boolean;
  width: number;
  height: number;
  refreshRate: number;
  /** 支持的显示模式（已去重排序） */
  modes: DisplayModeInfo[];
}

export interface DisplayConfig {
  /** 启动游戏前切换到下方分辨率与刷新率，游戏退出后自动恢复 */
  enabled: boolean;
  /** 游戏使用的显示器编号（segatools [gfx] monitor） */
  monitor: number;
  /** 启动时应用的分辨率；0 = 不修改 */
  width: number;
  height: number;
  /** 启动时应用的刷新率（Hz）；0 = 保持当前 */
  refreshRate: number;
}

export const DEFAULT_DISPLAY: DisplayConfig = {
  enabled: false,
  monitor: 0,
  width: 0,
  height: 0,
  refreshRate: 0,
};
