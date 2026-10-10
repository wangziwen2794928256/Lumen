use crate::kernel::platform::media::Sample;
use uuid::Uuid;

// Conservative observed playback, never wall-clock duration or a seek distance.
#[derive(Default)]
pub struct Tracker {
    previous: Option<Sample>,
    event: String,
    elapsed_ms: u64,
    saved_ms: u64,
    started_at: String,
}
pub struct Recording {
    pub sample: Sample,
    pub event: String,
    pub seconds: i64,
    pub started_at: String,
}
impl Tracker {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
    pub fn observe(
        &mut self,
        sample: Option<Sample>,
        delta_ms: u64,
        threshold: i64,
    ) -> Option<Recording> {
        let Some(next) = sample.filter(|s| s.valid()) else {
            self.reset();
            return None;
        };
        let same = self
            .previous
            .as_ref()
            .is_some_and(|p| p.identity() == next.identity());
        let rewind = self
            .previous
            .as_ref()
            .is_some_and(|p| next.position_ms + 5000 < p.position_ms);
        if !same || rewind {
            self.event = Uuid::new_v4().to_string();
            self.elapsed_ms = 0;
            self.saved_ms = 0;
            self.started_at = crate::database::now();
        } else if let Some(prev) = &self.previous {
            let advanced = next.position_ms - prev.position_ms;
            if next.playing && prev.playing && delta_ms <= 4000 && advanced > 0 && advanced <= 5000
            {
                self.elapsed_ms = (self.elapsed_ms + delta_ms.min(advanced as u64)).min(86_400_000);
            }
        }
        self.previous = Some(next.clone());
        if self.elapsed_ms >= threshold as u64 * 1000
            && (self.saved_ms == 0 || self.elapsed_ms >= self.saved_ms + 5000)
        {
            return Some(Recording {
                sample: next,
                event: self.event.clone(),
                seconds: (self.elapsed_ms / 1000) as i64,
                started_at: self.started_at.clone(),
            });
        }
        None
    }
    pub fn acknowledge(&mut self) {
        self.saved_ms = self.elapsed_ms;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn sample(position: i64, playing: bool) -> Sample {
        Sample {
            title: "Fixture song".into(),
            artist: "A".into(),
            album: "B".into(),
            app: "fixture.player".into(),
            position_ms: position,
            playing,
        }
    }
    #[test]
    fn threshold_pause_repeat_seek_and_missing_metadata() {
        let mut t = Tracker::default();
        assert!(t.observe(Some(sample(0, true)), 1000, 5).is_none());
        for i in 1..5 {
            assert!(t.observe(Some(sample(i * 1000, true)), 1000, 5).is_none());
        }
        let r = t.observe(Some(sample(5000, true)), 1000, 5).unwrap();
        assert_eq!(r.seconds, 5);
        let event = r.event;
        t.acknowledge();
        assert!(t.observe(Some(sample(5000, false)), 1000, 5).is_none());
        assert!(t.observe(Some(sample(5000, false)), 50_000, 5).is_none());
        assert!(t.observe(Some(sample(5000, true)), 1000, 5).is_none());
        assert!(t.observe(Some(sample(60_000, true)), 1000, 5).is_none());
        for i in 1..5 {
            assert!(t
                .observe(Some(sample(60_000 + i * 1000, true)), 1000, 5)
                .is_none());
        }
        let r = t.observe(Some(sample(65_000, true)), 1000, 5).unwrap();
        assert_eq!(r.event, event);
        assert_eq!(r.seconds, 10);
        t.acknowledge();
        t.observe(Some(sample(0, true)), 1000, 5);
        for i in 1..5 {
            t.observe(Some(sample(i * 1000, true)), 1000, 5);
        }
        assert_ne!(
            t.observe(Some(sample(5000, true)), 1000, 5).unwrap().event,
            event
        );
        assert!(t.observe(None, 1000, 5).is_none());
        let mut ad = sample(0, true);
        ad.title = "Advertisement".into();
        assert!(t.observe(Some(ad), 1000, 5).is_none());
    }
}
