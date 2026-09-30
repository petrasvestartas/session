#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Viewport {
    pub width: u32,
    pub height: u32,
}

impl Viewport {
    pub fn from_css(width: f64, height: f64, ratio: f64, limit: u32) -> Option<Self> {
        if [width, height, ratio].iter().any(|value| !value.is_finite() || *value <= 0.0) || limit == 0 {
            return None;
        }
        let width = width * ratio;
        let height = height * ratio;
        if !width.is_finite() || !height.is_finite() {
            return None;
        }
        // Reduce both dimensions together when the GPU cannot hold the requested image.
        let scale = (limit as f64 / width.max(height)).min(1.0);
        Some(Self {
            width: (width * scale).round().clamp(1.0, limit as f64) as u32,
            height: (height * scale).round().clamp(1.0, limit as f64) as u32,
        })
    }

    pub fn aspect(self) -> f64 {
        self.width as f64 / self.height as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_density_changes_pixels_without_stretching_the_view() {
        let normal = Viewport::from_css(768.0, 384.0, 1.0, 4096).unwrap();
        let dense = Viewport::from_css(768.0, 384.0, 2.0, 4096).unwrap();
        assert_eq!(dense.width, normal.width * 2);
        assert_eq!(dense.height, normal.height * 2);
        assert_eq!(dense.aspect(), normal.aspect());
        let capped = Viewport::from_css(768.0, 384.0, 4.0, 1024).unwrap();
        assert_eq!(capped, Viewport { width: 1024, height: 512 });
    }

    #[test]
    fn a_hidden_canvas_has_no_renderable_size() {
        assert!(Viewport::from_css(640.0, 0.0, 1.0, 4096).is_none());
        assert!(Viewport::from_css(f64::NAN, 480.0, 1.0, 4096).is_none());
        assert!(Viewport::from_css(640.0, 480.0, 0.0, 4096).is_none());
    }
}
