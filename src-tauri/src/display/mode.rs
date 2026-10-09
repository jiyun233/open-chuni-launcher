#[cfg(windows)]
use crate::model::display::MonitorInfo;

#[cfg(windows)]
pub struct DisplayGuard {
    device: Vec<u16>,
    original: windows_sys::Win32::Graphics::Gdi::DEVMODEW,
    restored: bool,
}

#[cfg(windows)]
impl DisplayGuard {
    pub fn restore(&mut self) {
        use std::ptr;

        use windows_sys::Win32::Graphics::Gdi::ChangeDisplaySettingsExW;

        if self.restored {
            return;
        }
        self.restored = true;
        unsafe {
            ChangeDisplaySettingsExW(
                self.device.as_ptr(),
                &mut self.original,
                ptr::null_mut(),
                0,
                ptr::null(),
            );
        }
    }
}

#[cfg(windows)]
impl Drop for DisplayGuard {
    fn drop(&mut self) {
        self.restore();
    }
}

#[cfg(windows)]
pub fn apply_mode(
    monitor: u32,
    width: u32,
    height: u32,
    refresh_rate: u32,
) -> Result<Option<DisplayGuard>, String> {
    use std::ptr;

    use windows_sys::Win32::Graphics::Gdi::{
        ChangeDisplaySettingsExW, EnumDisplaySettingsExW, CDS_FULLSCREEN, CDS_TEST, DEVMODEW,
        DISP_CHANGE_SUCCESSFUL, DM_DISPLAYFREQUENCY, DM_PELSHEIGHT, DM_PELSWIDTH,
        ENUM_CURRENT_SETTINGS,
    };

    use super::enumerate::{list_monitors, to_wide};

    let target: MonitorInfo = list_monitors()
        .into_iter()
        .find(|info| info.index == monitor)
        .ok_or_else(|| format!("未找到显示器 {monitor}"))?;
    if width == 0 || height == 0 {
        return Ok(None);
    }
    if width == target.width
        && height == target.height
        && (refresh_rate == 0 || refresh_rate == target.refresh_rate)
    {
        return Ok(None);
    }
    let supported = target.modes.iter().any(|mode| {
        mode.width == width
            && mode.height == height
            && (refresh_rate == 0 || mode.refresh_rate == refresh_rate)
    });
    if !supported {
        return Err(format!(
            "显示器 {monitor} 不支持 {width}×{height} @ {refresh_rate} Hz"
        ));
    }

    let device_wide = to_wide(&target.device);
    let mut current: DEVMODEW = DEVMODEW::default();
    current.dmSize = size_of::<DEVMODEW>() as u16;
    unsafe {
        if EnumDisplaySettingsExW(device_wide.as_ptr(), ENUM_CURRENT_SETTINGS, &mut current, 0)
            == 0
        {
            return Err("读取显示器当前模式失败".to_string());
        }
        let mut mode = current;
        mode.dmFields |= DM_PELSWIDTH | DM_PELSHEIGHT;
        mode.dmPelsWidth = width;
        mode.dmPelsHeight = height;
        if refresh_rate > 0 {
            mode.dmFields |= DM_DISPLAYFREQUENCY;
            mode.dmDisplayFrequency = refresh_rate;
        }
        let test = ChangeDisplaySettingsExW(
            device_wide.as_ptr(),
            &mut mode,
            ptr::null_mut(),
            CDS_TEST,
            ptr::null(),
        );
        if test != DISP_CHANGE_SUCCESSFUL {
            return Err(format!("显示模式测试失败（错误码 {test}）"));
        }
        let applied = ChangeDisplaySettingsExW(
            device_wide.as_ptr(),
            &mut mode,
            ptr::null_mut(),
            CDS_FULLSCREEN,
            ptr::null(),
        );
        if applied != DISP_CHANGE_SUCCESSFUL {
            return Err(format!("切换显示模式失败（错误码 {applied}）"));
        }
    }
    Ok(Some(DisplayGuard {
        device: device_wide,
        original: current,
        restored: false,
    }))
}

#[cfg(not(windows))]
pub struct DisplayGuard;

#[cfg(not(windows))]
pub fn apply_mode(
    _monitor: u32,
    _width: u32,
    _height: u32,
    _refresh_rate: u32,
) -> Result<Option<DisplayGuard>, String> {
    Err("显示器切换仅支持 Windows".to_string())
}
