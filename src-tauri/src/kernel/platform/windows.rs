#[cfg(windows)]
mod windows_events {
    use crate::database::{DbError, Result};
    use crate::today::{native_events::SystemEvent, TodayService};
    use std::sync::{atomic::Ordering, Arc};
    use windows_sys::Win32::{
        Foundation::{HWND, LPARAM, LRESULT, WPARAM},
        System::{RemoteDesktop::*, Threading::GetCurrentThreadId},
        UI::{
            Shell::{DefSubclassProc, RemoveWindowSubclass, SetWindowSubclass},
            WindowsAndMessaging::*,
        },
    };
    const ID: usize = 0x4C554D45;
    unsafe extern "system" fn callback(
        hwnd: HWND,
        msg: u32,
        w: WPARAM,
        l: LPARAM,
        id: usize,
        data: usize,
    ) -> LRESULT {
        // The boxed Arc lives on the owning UI thread until WM_NCDESTROY.
        let service = &*(data as *const Arc<TodayService>);
        let event = match (msg, w as u32) {
            (
                WM_WTSSESSION_CHANGE,
                WTS_SESSION_LOCK | WTS_CONSOLE_DISCONNECT | WTS_REMOTE_DISCONNECT,
            ) => Some(SystemEvent::Lock),
            (
                WM_WTSSESSION_CHANGE,
                WTS_SESSION_UNLOCK | WTS_CONSOLE_CONNECT | WTS_REMOTE_CONNECT,
            ) => Some(SystemEvent::Unlock),
            (WM_POWERBROADCAST, PBT_APMSUSPEND) => Some(SystemEvent::Suspend),
            (
                WM_POWERBROADCAST,
                PBT_APMRESUMEAUTOMATIC | PBT_APMRESUMESUSPEND | PBT_APMRESUMECRITICAL,
            ) => Some(SystemEvent::Resume),
            _ => None,
        };
        if let Some(event) = event {
            service.signal(event); // Latch synchronously before returning, even if sleep follows immediately.
            let service = service.clone();
            tauri::async_runtime::spawn(async move {
                let _ = service.snapshot().await;
            });
        }
        if msg == WM_NCDESTROY {
            WTSUnRegisterSessionNotification(hwnd);
            RemoveWindowSubclass(hwnd, Some(callback), id);
            let result = DefSubclassProc(hwnd, msg, w, l);
            drop(Box::from_raw(data as *mut Arc<TodayService>));
            return result;
        }
        DefSubclassProc(hwnd, msg, w, l)
    }
    pub fn install(window: &tauri::WebviewWindow, service: Arc<TodayService>) -> Result<()> {
        let hwnd = window
            .hwnd()
            .map_err(|_| {
                DbError::new(
                    "NATIVE_HOOK_UNAVAILABLE",
                    "系统事件监听未能启动，专注功能暂不可用。",
                )
            })?
            .0 as HWND;
        install_hwnd(hwnd, service)
    }
    fn install_hwnd(hwnd: HWND, service: Arc<TodayService>) -> Result<()> {
        unsafe {
            if GetWindowThreadProcessId(hwnd, std::ptr::null_mut()) != GetCurrentThreadId() {
                return Err(DbError::new(
                    "NATIVE_HOOK_UNAVAILABLE",
                    "系统事件监听未能启动，专注功能暂不可用。",
                ));
            }
            let data = Box::into_raw(Box::new(service.clone()));
            if SetWindowSubclass(hwnd, Some(callback), ID, data as usize) == 0 {
                drop(Box::from_raw(data));
                return Err(DbError::new(
                    "NATIVE_HOOK_UNAVAILABLE",
                    "系统事件监听未能启动，专注功能暂不可用。",
                ));
            }
            if WTSRegisterSessionNotification(hwnd, NOTIFY_FOR_THIS_SESSION) == 0 {
                RemoveWindowSubclass(hwnd, Some(callback), ID);
                drop(Box::from_raw(data));
                return Err(DbError::new(
                    "NATIVE_HOOK_UNAVAILABLE",
                    "系统事件监听未能启动，专注功能暂不可用。",
                ));
            }
        }
        service.native_hooks_ready.store(true, Ordering::SeqCst);
        Ok(())
    }
    #[cfg(test)]
    #[tokio::test]
    async fn real_subclass_dispatch_and_destroy_release_registration() {
        let _folder = tempfile::tempdir().unwrap();
        let db = crate::database::Database::open(_folder.path())
            .await
            .unwrap();
        let service = TodayService::open(db).await.unwrap();
        let service = Arc::new(service);
        let class: Vec<u16> = "STATIC\0".encode_utf16().collect();
        let hwnd = unsafe {
            CreateWindowExW(
                0,
                class.as_ptr(),
                class.as_ptr(),
                0,
                0,
                0,
                1,
                1,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null(),
            )
        };
        assert!(!hwnd.is_null());
        install_hwnd(hwnd, service.clone()).unwrap();
        assert!(service.native_hooks_ready.load(Ordering::SeqCst));
        unsafe {
            SendMessageW(hwnd, WM_WTSSESSION_CHANGE, WTS_SESSION_LOCK as usize, 0);
        }
        assert_eq!(service.system_mask.load(Ordering::SeqCst), 1);
        unsafe {
            SendMessageW(hwnd, WM_WTSSESSION_CHANGE, WTS_SESSION_UNLOCK as usize, 0);
            SendMessageW(hwnd, WM_POWERBROADCAST, PBT_APMSUSPEND as usize, 0);
        }
        assert_eq!(service.system_mask.load(Ordering::SeqCst), 2);
        unsafe {
            SendMessageW(hwnd, WM_POWERBROADCAST, PBT_APMRESUMEAUTOMATIC as usize, 0);
        }
        assert_eq!(service.system_mask.load(Ordering::SeqCst), 0);
        assert_eq!(service.system_epoch.load(Ordering::SeqCst), 2);
        assert_ne!(unsafe { DestroyWindow(hwnd) }, 0);
        for _ in 0..100 {
            if Arc::strong_count(&service) == 1 {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        }
        assert_eq!(Arc::strong_count(&service), 1);
        service.db.pool.close().await;
    }
}

pub use windows_events::install;
