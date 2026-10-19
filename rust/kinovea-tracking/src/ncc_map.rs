/// Correlation map of size `(W - w + 1) x (H - h + 1)`, row-major, as `cv::matchTemplate` returns it.
#[derive(Clone, Debug, PartialEq)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub values: Vec<f32>,
}

/// Result of `cv::minMaxLoc`: locations are `(x, y)` of the first extreme in row-major order.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MinMax {
    pub min: f32,
    pub max: f32,
    pub min_loc: (usize, usize),
    pub max_loc: (usize, usize),
}

impl Map {
    pub fn at(&self, x: usize, y: usize) -> f32 {
        self.values[y * self.width + x]
    }

    /// `cv::minMaxLoc`: NaN is never selected, ties keep the first location in row-major order.
    pub fn min_max_loc(&self) -> MinMax {
        let mut found = MinMax {
            min: f32::INFINITY,
            max: f32::NEG_INFINITY,
            min_loc: (0, 0),
            max_loc: (0, 0),
        };
        for (i, &v) in self.values.iter().enumerate() {
            let loc = (i % self.width, i / self.width);
            if v < found.min {
                found.min = v;
                found.min_loc = loc;
            }
            if v > found.max {
                found.max = v;
                found.max_loc = loc;
            }
        }
        found
    }
}
