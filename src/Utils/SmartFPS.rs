#[derive(Clone)]
pub struct SmartFPS {
    current_frametimes: f64,
    weight: f64,
    numerator: i32,
}

impl SmartFPS {
    pub fn new(old_frame_weight: i32) -> SmartFPS {
        SmartFPS {
            current_frametimes: 0.0,
            weight: old_frame_weight as f64 / (old_frame_weight as f64 - 1.0),
            numerator: old_frame_weight,
        }
    }

    pub fn update(&mut self, time_since_last_frame: f64) {
        self.current_frametimes /= self.weight;
        self.current_frametimes += time_since_last_frame;
    }

    pub fn framerate(&self) -> f64 {
        self.numerator as f64 / self.current_frametimes
    }
}
