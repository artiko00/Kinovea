use crate::ncc::Plane;

/// Summed-area tables of the values and of their squares, one plane per channel.
pub(crate) struct Integral {
    stride: usize,
    channels: usize,
    sum: Vec<f64>,
    square: Vec<f64>,
}

impl Integral {
    pub(crate) fn new<T: Copy + Into<f64>>(image: Plane<T>) -> Self {
        let (stride, ch) = (image.width + 1, image.channels);
        let len = (image.height + 1) * stride * ch;
        let mut table = Integral {
            stride,
            channels: ch,
            sum: vec![0.0; len],
            square: vec![0.0; len],
        };
        for y in 0..image.height {
            for x in 0..image.width {
                for c in 0..ch {
                    let v: f64 = image.data[(y * image.width + x) * ch + c].into();
                    let at = table.slot(x + 1, y + 1, c);
                    let (above, left, corner) = (
                        table.slot(x + 1, y, c),
                        table.slot(x, y + 1, c),
                        table.slot(x, y, c),
                    );
                    table.sum[at] = v + table.sum[left] + table.sum[above] - table.sum[corner];
                    table.square[at] =
                        v * v + table.square[left] + table.square[above] - table.square[corner];
                }
            }
        }
        table
    }

    fn slot(&self, x: usize, y: usize, c: usize) -> usize {
        (y * self.stride + x) * self.channels + c
    }

    /// `(sum, sum of squares)` of channel `c` over the `size` window at `origin`.
    pub(crate) fn window(
        &self,
        (x, y): (usize, usize),
        (w, h): (usize, usize),
        c: usize,
    ) -> (f64, f64) {
        let rect = |table: &[f64]| {
            table[self.slot(x + w, y + h, c)]
                - table[self.slot(x, y + h, c)]
                - table[self.slot(x + w, y, c)]
                + table[self.slot(x, y, c)]
        };
        (rect(&self.sum), rect(&self.square))
    }
}
