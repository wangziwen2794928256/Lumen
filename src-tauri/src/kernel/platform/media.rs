use crate::database::{DbError, Result};
#[derive(Clone, Debug)]
pub struct Sample {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub app: String,
    pub playing: bool,
    pub position_ms: i64,
}
impl Sample {
    pub fn identity(&self) -> String {
        serde_json::to_string(&(&self.app, &self.title, &self.artist, &self.album))
            .unwrap_or_default()
    }
    pub fn valid(&self) -> bool {
        let title = self.title.trim().to_lowercase();
        !title.is_empty()
            && !["advertisement", "advert", "ad", "广告", "廣告"].contains(&title.as_str())
            && self.title.chars().count() <= 300
            && self.artist.chars().count() <= 300
            && self.album.chars().count() <= 300
            && !self.app.is_empty()
            && self.app.len() <= 300
            && self.position_ms >= 0
    }
}
pub trait MediaAdapter {
    fn poll(&mut self, allowed_app: &str) -> Result<Option<Sample>>;
}
#[cfg(any(windows, test))]
fn project_position(
    base: i64,
    updated_ms: i64,
    now_ms: i64,
    playing: bool,
    rate: f64,
    end: i64,
) -> i64 {
    let advance = if playing && rate.is_finite() && (0.0..=4.0).contains(&rate) && updated_ms > 0 {
        ((now_ms - updated_ms).clamp(0, 86_400_000) as f64 * rate) as i64
    } else {
        0
    };
    let value = base.saturating_add(advance);
    if end > 0 {
        value.min(end)
    } else {
        value
    }
}
#[cfg(test)]
mod timeline_tests {
    #[test]
    fn timeline_projection_uses_snapshot_time_pause_rate_and_end_bound() {
        assert_eq!(
            super::project_position(1000, 10_000, 12_000, true, 1.0, 60_000),
            3000
        );
        assert_eq!(
            super::project_position(1000, 10_000, 12_000, false, 1.0, 60_000),
            1000
        );
        assert_eq!(
            super::project_position(1000, 10_000, 12_000, true, 2.0, 60_000),
            5000
        );
        assert_eq!(
            super::project_position(59_000, 10_000, 12_000, true, 1.0, 60_000),
            60_000
        );
        assert_eq!(
            super::project_position(1000, 0, 12_000, true, 1.0, 60_000),
            1000
        );
    }
}
pub fn connect() -> Result<Box<dyn MediaAdapter>> {
    #[cfg(windows)]
    {
        windows::connect()
    }
    #[cfg(not(windows))]
    {
        Err(DbError::new(
            "MEDIA_UNSUPPORTED",
            "此平台暂无媒体监听，请使用手动记录。",
        ))
    }
}
#[cfg(windows)]
mod windows {
    use super::*;
    use ::windows::{
        Media::Control::{
            GlobalSystemMediaTransportControlsSessionManager as Manager,
            GlobalSystemMediaTransportControlsSessionPlaybackStatus as Status,
        },
        Win32::System::WinRT::{RoInitialize, RoUninitialize, RO_INIT_MULTITHREADED},
    };
    struct Apartment;
    impl Drop for Apartment {
        fn drop(&mut self) {
            unsafe {
                RoUninitialize();
            }
        }
    }
    struct WindowsMedia {
        manager: Manager,
        runtime: tokio::runtime::Runtime,
        _apartment: Apartment,
    }
    fn failure() -> DbError {
        DbError::new(
            "MEDIA_UNAVAILABLE",
            "媒体会话不可用或权限不足，请使用手动记录。",
        )
    }
    pub fn connect() -> Result<Box<dyn MediaAdapter>> {
        unsafe {
            RoInitialize(RO_INIT_MULTITHREADED).map_err(|_| failure())?;
        }
        let apartment = Apartment;
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|_| failure())?;
        let operation = Manager::RequestAsync().map_err(|_| failure())?;
        let manager = runtime
            .block_on(async {
                tokio::time::timeout(std::time::Duration::from_secs(2), async { operation.await })
                    .await
            })
            .map_err(|_| failure())?
            .map_err(|_| failure())?;
        Ok(Box::new(WindowsMedia {
            manager,
            runtime,
            _apartment: apartment,
        }))
    }
    impl MediaAdapter for WindowsMedia {
        fn poll(&mut self, allowed_app: &str) -> Result<Option<Sample>> {
            let session = match self.manager.GetCurrentSession() {
                Ok(s) => s,
                Err(_) => return Ok(None),
            };
            let app = session
                .SourceAppUserModelId()
                .map_err(|_| failure())?
                .to_string();
            // Check the consented source before reading private song metadata.
            if app != allowed_app {
                return Ok(Some(Sample {
                    app,
                    title: String::new(),
                    artist: String::new(),
                    album: String::new(),
                    playing: false,
                    position_ms: 0,
                }));
            }
            let operation = session
                .TryGetMediaPropertiesAsync()
                .map_err(|_| failure())?;
            let props = self
                .runtime
                .block_on(async {
                    tokio::time::timeout(std::time::Duration::from_secs(2), async {
                        operation.await
                    })
                    .await
                })
                .map_err(|_| failure())?
                .map_err(|_| failure())?;
            let timeline = session.GetTimelineProperties().map_err(|_| failure())?;
            let info = session.GetPlaybackInfo().map_err(|_| failure())?;
            let playing = info.PlaybackStatus().map_err(|_| failure())? == Status::Playing;
            let rate = info.PlaybackRate().and_then(|r| r.Value()).unwrap_or(1.0);
            let updated = timeline
                .LastUpdatedTime()
                .map_err(|_| failure())?
                .UniversalTime
                / 10_000
                - 11_644_473_600_000;
            let position = project_position(
                timeline.Position().map_err(|_| failure())?.Duration / 10_000,
                updated,
                chrono::Utc::now().timestamp_millis(),
                playing,
                rate,
                timeline.EndTime().map_err(|_| failure())?.Duration / 10_000,
            );
            Ok(Some(Sample {
                title: props.Title().map_err(|_| failure())?.to_string(),
                artist: props.Artist().map_err(|_| failure())?.to_string(),
                album: props.AlbumTitle().map_err(|_| failure())?.to_string(),
                app,
                playing,
                position_ms: position,
            }))
        }
    }
}
