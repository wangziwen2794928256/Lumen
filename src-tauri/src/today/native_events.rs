use super::*;

#[derive(Clone, Copy)]
pub(crate) enum SystemEvent {
    Lock,
    Unlock,
    Suspend,
    Resume,
}
impl TodayService {
    pub(crate) fn signal(&self, event: SystemEvent) {
        match event {
            SystemEvent::Lock | SystemEvent::Suspend => {
                let bit = if matches!(event, SystemEvent::Lock) {
                    1
                } else {
                    2
                };
                self.system_mask.fetch_or(bit, Ordering::SeqCst);
                self.system_reason.store(bit, Ordering::SeqCst);
                self.system_epoch.fetch_add(1, Ordering::SeqCst);
            }
            SystemEvent::Unlock => {
                self.system_mask.fetch_and(!1, Ordering::SeqCst);
            }
            SystemEvent::Resume => {
                self.system_mask.fetch_and(!2, Ordering::SeqCst);
            }
        }
    }
}

pub use crate::kernel::platform::install_system_events as install;
