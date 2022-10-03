use std::f32::consts::TAU;

pub struct AngleRange {
    pub min: f32,
    pub max: f32,
}
impl AngleRange {
    pub fn from(a: f32, b: f32) -> AngleRange {
        let at = AngleRange::true_angle(a);
        let bt = AngleRange::true_angle(b);
        if at < bt {
            AngleRange { min: at, max: bt }
        } else {
            AngleRange { min: bt, max: at }
        }
    }
    pub fn true_angle(a: f32) -> f32 {
        ((a % TAU) + TAU) % TAU + TAU
    }
    pub fn contains(&self, angle: f32) -> bool {
        self.min < AngleRange::true_angle(angle) && self.max > AngleRange::true_angle(angle)
    }

    pub fn overlap(&self, other: Self) -> Option<AngleRange> {
        let mut min: f32 = 0.0;
        let mut max: f32 = 0.0;
        if self.contains(other.min) {
            min = other.min;
        }
        if other.contains(self.min) {
            min = self.min;
        }
        if self.contains(other.max) {
            max = other.max;
        }
        if other.contains(self.max) {
            max = self.max;
        }
        if min == 0.0 && max == 0.0 {
            None
        } else {
            Some(AngleRange { min, max })
        }
    }
}
