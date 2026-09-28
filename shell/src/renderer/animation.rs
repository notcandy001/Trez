use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub enum Easing {
    Linear,
    EaseInOutQuad,
    EaseOutQuad,
    EaseOutExpo,
}

impl Easing {
    pub fn apply(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Easing::Linear => t,
            Easing::EaseInOutQuad => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            Easing::EaseOutQuad => t * (2.0 - t),
            Easing::EaseOutExpo => {
                if t == 1.0 {
                    1.0
                } else {
                    1.0 - 2.0f32.powf(-10.0 * t)
                }
            }
        }
    }
}

pub struct Animator {
    start_time: Option<Instant>,
    duration: Duration,
    easing: Easing,
    start_val: f32,
    end_val: f32,
}

impl Animator {
    pub fn new(duration: Duration, easing: Easing, start_val: f32, end_val: f32) -> Self {
        Self {
            start_time: None,
            duration,
            easing,
            start_val,
            end_val,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn value(&self) -> f32 {
        if let Some(start) = self.start_time {
            let elapsed = start.elapsed();
            if elapsed >= self.duration {
                self.end_val
            } else {
                let t = elapsed.as_secs_f32() / self.duration.as_secs_f32();
                let eased = self.easing.apply(t);
                self.start_val + (self.end_val - self.start_val) * eased
            }
        } else {
            self.start_val
        }
    }

    pub fn is_finished(&self) -> bool {
        if let Some(start) = self.start_time {
            start.elapsed() >= self.duration
        } else {
            false
        }
    }
}
