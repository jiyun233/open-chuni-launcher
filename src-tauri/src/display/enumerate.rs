use crate::model::display::MonitorInfo;
#[cfg(windows)]
use crate::model::display::DisplayModeInfo;

#[cfg(windows)]
pub(super) fn to_wide(value: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    OsStr::new(value).encode_wide().chain(Some(0)).collect()
}

#[cfg(windows)]
pub(super) fn wide_to_string(buffer: &[u16]) -> String {
    let len = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());
    String::from_utf16_lossy(&buffer[..len])
}

#[cfg(windows)]
unsafe fn enum_modes(device_wide: &[u16]) -> Vec<DisplayModeInfo> {
    use windows_sys::Win32::Graphics::Gdi::{
        DM_INTERLACED, DEVMODEW, EnumDisplaySettingsExW,
    };

    let mut modes: Vec<DisplayModeInfo> = Vec::new();
    let mut index = 0u32;
    loop {
        let mut mode: DEVMODEW = DEVMODEW::default();
        mode.dmSize = size_of::<DEVMODEW>() as u16;
        if EnumDisplaySettingsExW(device_wide.as_ptr(), index, &mut mode, 0) == 0 {
            break;
        }
        index += 1;
        if index > 4096 {
            break;
        }
        if mode.dmPelsWidth == 0 || mode.dmPelsHeight == 0 {
            continue;
        }
        if unsafe { mode.Anonymous2.dmDisplayFlags } & DM_INTERLACED != 0 {
            continue;
        }
        let entry = DisplayModeInfo {
            width: mode.dmPelsWidth,
            height: mode.dmPelsHeight,
            refresh_rate: mode.dmDisplayFrequency,
        };
        if !modes.contains(&entry) {
            modes.push(entry);
        }
    }
    modes.sort_by(|a, b| {
        (b.width * b.height)
            .cmp(&(a.width * a.height))
            .then(b.width.cmp(&a.width))
            .then(b.refresh_rate.cmp(&a.refresh_rate))
    });
    modes.retain(|mode| mode.refresh_rate > 1); // 过滤 0/1 Hz 的无效条目
    modes
}

#[cfg(windows)]
pub fn list_monitors() -> Vec<MonitorInfo> {
    use std::ptr;

    use windows_sys::Win32::Graphics::Gdi::{
        EnumDisplayDevicesW, EnumDisplaySettingsExW, DEVMODEW, DISPLAY_DEVICEW,
        DISPLAY_DEVICE_ATTACHED_TO_DESKTOP, DISPLAY_DEVICE_PRIMARY_DEVICE, ENUM_CURRENT_SETTINGS,
    };

    let mut monitors = Vec::new();
    let mut device_index = 0u32;
    unsafe {
        loop {
            let mut adapter: DISPLAY_DEVICEW = DISPLAY_DEVICEW::default();
            adapter.cb = size_of::<DISPLAY_DEVICEW>() as u32;
            if EnumDisplayDevicesW(ptr::null(), device_index, &mut adapter, 0) == 0 {
                break;
            }
            device_index += 1;
            if adapter.StateFlags & DISPLAY_DEVICE_ATTACHED_TO_DESKTOP == 0 {
                continue;
            }
            let device = wide_to_string(&adapter.DeviceName);
            if device.is_empty() {
                continue;
            }

            let mut monitor_device: DISPLAY_DEVICEW = DISPLAY_DEVICEW::default();
            monitor_device.cb = size_of::<DISPLAY_DEVICEW>() as u32;
            let adapter_string = wide_to_string(&adapter.DeviceString);
            let name = if EnumDisplayDevicesW(
                to_wide(&device).as_ptr(),
                0,
                &mut monitor_device,
                0,
            ) != 0
            {
                let friendly = wide_to_string(&monitor_device.DeviceString);
                if friendly.is_empty() {
                    adapter_string
                } else {
                    friendly
                }
            } else {
                adapter_string
            };

            let device_wide = to_wide(&device);
            let mut current: DEVMODEW = DEVMODEW::default();
            current.dmSize = size_of::<DEVMODEW>() as u16;
            let has_current = EnumDisplaySettingsExW(
                device_wide.as_ptr(),
                ENUM_CURRENT_SETTINGS,
                &mut current,
                0,
            ) != 0;

            let modes = enum_modes(&device_wide);
            let (width, height, refresh_rate) = if has_current {
                (
                    current.dmPelsWidth,
                    current.dmPelsHeight,
                    current.dmDisplayFrequency,
                )
            } else {
                modes
                    .first()
                    .map(|mode| (mode.width, mode.height, mode.refresh_rate))
                    .unwrap_or((0, 0, 0))
            };

            monitors.push(MonitorInfo {
                index: device_index,
                device,
                name,
                is_primary: adapter.StateFlags & DISPLAY_DEVICE_PRIMARY_DEVICE != 0,
                width,
                height,
                refresh_rate,
                modes,
            });
        }
    }
    monitors.sort_by_key(|monitor| !monitor.is_primary);
    for (position, monitor) in monitors.iter_mut().enumerate() {
        monitor.index = position as u32;
    }
    monitors
}

#[cfg(not(windows))]
pub fn list_monitors() -> Vec<MonitorInfo> {
    Vec::new()
}