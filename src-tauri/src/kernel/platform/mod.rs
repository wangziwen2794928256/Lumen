use crate::{
    database::{DbError, Result},
    today::TodayService,
};
use std::{path::Path, sync::Arc};
pub mod linux;
pub mod macos;
pub mod media;
#[cfg(windows)]
mod windows;
#[derive(Clone, Copy, Debug)]
pub struct Adapter {
    pub name: &'static str,
    pub session_events: bool,
    pub attachment_viewer: bool,
}
pub const WINDOWS: Adapter = Adapter {
    name: "windows",
    session_events: true,
    attachment_viewer: true,
};
pub fn host() -> Adapter {
    #[cfg(windows)]
    {
        WINDOWS
    }
    #[cfg(target_os = "macos")]
    {
        macos::ADAPTER
    }
    #[cfg(target_os = "linux")]
    {
        linux::ADAPTER
    }
    #[cfg(not(any(windows, target_os = "macos", target_os = "linux")))]
    {
        Adapter {
            name: "unsupported",
            session_events: false,
            attachment_viewer: false,
        }
    }
}
pub fn install_system_events(
    window: &tauri::WebviewWindow,
    service: Arc<TodayService>,
) -> Result<()> {
    #[cfg(windows)]
    {
        windows::install(window, service)
    }
    #[cfg(not(windows))]
    {
        let _ = (window, service);
        Err(DbError::new(
            "NATIVE_HOOK_UNAVAILABLE",
            "此平台的系统事件 adapter 尚未实现，专注功能暂不可用。",
        ))
    }
}
// Only Kernel callers with a verified canonical attachment path may invoke this.
pub fn open_verified_attachment(path: &Path) -> Result<()> {
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStrExt;
        let value: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
        let result = unsafe {
            windows_sys::Win32::UI::Shell::ShellExecuteW(
                std::ptr::null_mut(),
                std::ptr::null(),
                value.as_ptr(),
                std::ptr::null(),
                std::ptr::null(),
                1,
            )
        };
        if result as isize <= 32 {
            return Err(DbError::new(
                "ATTACHMENT_INVALID",
                "没有可用的附件查看程序。",
            ));
        }
        Ok(())
    }
    #[cfg(not(windows))]
    {
        let _ = path;
        Err(DbError::new(
            "UNSUPPORTED_PLATFORM",
            "此平台的附件查看 adapter 尚未实现。",
        ))
    }
}
pub fn open_data_directory(root: &Path) -> Result<()> {
    let path = root.canonicalize()?;
    if !path.is_dir() {
        return Err(DbError::invalid());
    }
    open_verified_attachment(&path)
}
pub fn open_local_preview(url: &str) -> Result<()> {
    let parsed = tauri::Url::parse(url).map_err(|_| DbError::invalid())?;
    if parsed.scheme() != "http"
        || parsed.host_str() != Some("127.0.0.1")
        || parsed.port().is_none()
        || parsed.path() != "/"
        || parsed.query().is_some()
        || parsed.fragment().is_some()
        || !parsed.username().is_empty()
        || parsed.password().is_some()
    {
        return Err(DbError::invalid());
    }
    #[cfg(windows)]
    {
        open_verified_attachment(Path::new(url))
    }
    #[cfg(not(windows))]
    {
        let _ = url;
        Err(DbError::new(
            "UNSUPPORTED_PLATFORM",
            "此平台的本地网站浏览器 adapter 尚未实现。",
        ))
    }
}
pub fn register_deep_link() -> Result<()> {
    #[cfg(windows)]
    {
        // HKCU only. Explicit user action; no elevation or filesystem capability is granted.
        let exe = std::env::current_exe()?;
        let command = format!("\"{}\" \"%1\"", exe.to_string_lossy());
        for (key, name, value) in [
            (
                "HKCU\\Software\\Classes\\lumen",
                None,
                "URL:Lumen Protocol".to_string(),
            ),
            (
                "HKCU\\Software\\Classes\\lumen",
                Some("URL Protocol"),
                String::new(),
            ),
            (
                "HKCU\\Software\\Classes\\lumen\\shell\\open\\command",
                None,
                command,
            ),
        ] {
            let mut child = std::process::Command::new("reg.exe");
            child.args(["add", key]);
            if let Some(n) = name {
                child.args(["/v", n]);
            } else {
                child.arg("/ve");
            }
            use std::os::windows::process::CommandExt;
            child.creation_flags(0x08000000);
            let output = child.args(["/t", "REG_SZ", "/d", &value, "/f"]).output()?;
            if !output.status.success() {
                return Err(DbError::new(
                    "PROTOCOL_REGISTER_FAILED",
                    "无法注册当前用户的 Lumen 协议。",
                ));
            }
        }
        Ok(())
    }
    #[cfg(not(windows))]
    {
        Err(DbError::new(
            "UNSUPPORTED_PLATFORM",
            "此平台的协议注册 adapter 尚未实现。",
        ))
    }
}
