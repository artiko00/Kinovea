use crate::ncc::Plane;
use crate::ncc_integral::Integral;
use crate::ncc_reference_test::lcg_bytes;

#[test]
fn window_sums_match_direct_summation_per_channel() {
    let (w, h, ch) = (7, 5, 2);
    let data = lcg_bytes(5, w * h * ch);
    let integral = Integral::new(Plane {
        data: &data,
        width: w,
        height: h,
        channels: ch,
    });
    for (ox, oy, ww, wh) in [(0, 0, 7, 5), (2, 1, 3, 2), (6, 4, 1, 1), (0, 3, 4, 2)] {
        for c in 0..ch {
            let (mut sum, mut sq) = (0.0, 0.0);
            for y in oy..oy + wh {
                for x in ox..ox + ww {
                    let v = f64::from(data[(y * w + x) * ch + c]);
                    sum += v;
                    sq += v * v;
                }
            }
            assert_eq!(
                integral.window((ox, oy), (ww, wh), c),
                (sum, sq),
                "window ({ox}, {oy}) channel {c}"
            );
        }
    }
}
