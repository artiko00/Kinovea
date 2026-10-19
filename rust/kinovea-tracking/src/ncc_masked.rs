//! `TM_CCOEFF_NORMED` with a binary mask, as `matchTemplateMask` computes it (OpenCV 4.x). The
//! template is centred on its masked mean, only masked pixels take part in the image energy and
//! the image is not centred: the centring term sums to zero over the mask and drops out.

use crate::ncc::{Map, Plane};

pub(crate) fn match_masked<T: Copy + Into<f64>>(
    image: Plane<T>,
    template: Plane<T>,
    mask: &[u8],
) -> Map {
    assert_eq!(mask.len(), template.width * template.height);
    let (width, height) = (
        image.width - template.width + 1,
        image.height - template.height + 1,
    );
    let included: Vec<bool> = mask.iter().map(|&m| m != 0).collect();
    let n = included.iter().filter(|&&m| m).count() as f64;
    assert!(n > 0.0, "the mask excludes every template pixel");
    let centred = centred_template(template, &included, n);
    let template_norm = centred.iter().map(|a| a * a).sum::<f64>().sqrt();
    // OpenCV: a template that is flat under the mask makes every window 0/0, so the whole map is NaN.
    if template_norm < f64::EPSILON {
        return Map {
            width,
            height,
            values: vec![f32::NAN; width * height],
        };
    }
    let mut values = Vec::with_capacity(width * height);
    for y in 0..height {
        for x in 0..width {
            let (cross, energy) = masked_window(image, template, (&included, &centred, n), (x, y));
            // Rounding can leave a flat window's energy slightly negative: clamp before the root,
            // where OpenCV would produce NaN. Flat windows give a zero denominator as in OpenCV.
            values.push((cross / (energy.max(0.0).sqrt() * template_norm)) as f32);
        }
    }
    Map {
        width,
        height,
        values,
    }
}

/// `a = M (T - mean over M of T)` per template pixel and channel, zero outside the mask.
fn centred_template<T: Copy + Into<f64>>(
    template: Plane<T>,
    included: &[bool],
    n: f64,
) -> Vec<f64> {
    let ch = template.channels;
    let mut means = vec![0.0; ch];
    for (pixel, &inc) in template.data.chunks_exact(ch).zip(included) {
        if inc {
            for (m, &v) in means.iter_mut().zip(pixel) {
                *m += v.into();
            }
        }
    }
    means.iter_mut().for_each(|m| *m /= n);
    template
        .data
        .chunks_exact(ch)
        .zip(included)
        .flat_map(|(pixel, &inc)| {
            pixel.iter().zip(&means).map(move |(&v, &m)| {
                let v: f64 = v.into();
                if inc {
                    v - m
                } else {
                    0.0
                }
            })
        })
        .collect()
}

/// `(sum a I, sum_c sum M I_c^2 - sum_c (sum M I_c)^2 / n)` over the window at `origin`.
fn masked_window<T: Copy + Into<f64>>(
    image: Plane<T>,
    template: Plane<T>,
    (included, centred, n): (&[bool], &[f64], f64),
    (x, y): (usize, usize),
) -> (f64, f64) {
    let ch = template.channels;
    let (mut cross, mut sq) = (0.0, 0.0);
    let mut sums = vec![0.0; ch];
    for (p, (&inc, a)) in included.iter().zip(centred.chunks_exact(ch)).enumerate() {
        let base = ((y + p / template.width) * image.width + x + p % template.width) * ch;
        for (c, &a_c) in a.iter().enumerate() {
            let v: f64 = image.data[base + c].into();
            cross += a_c * v;
            if inc {
                sums[c] += v;
                sq += v * v;
            }
        }
    }
    (cross, sq - sums.iter().map(|s| s * s / n).sum::<f64>())
}
