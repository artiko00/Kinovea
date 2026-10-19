use super::grid::{draw_grid, Grid};
use crate::backend::recording::Call;
use crate::paint::helpers::frame::scene::{Scene, COLOR};
use crate::renderer::Pen;
use crate::viewport::Point;
use kinovea_geometry::{PointF, ProjectiveMapper, Quadrilateral, RectangleF};

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

#[test]
fn edges_only_mode_draws_the_edges_then_the_secondary_ticks() {
    // C#: DrawingPlane.cs:297-330 (DrawGrid, else branch: edges 301, ticks 307-325), 314 (halfLength).
    // Identity mapping of a 100 x 100 plane, 2 x 2 divisions, scale 4. The edges are the four sides.
    // Ticks sit on the inner lines 50: horizontal ones span 100 / 40 = 2.5 on each side of the
    // edges (x in [-2.5, 2.5] and [97.5, 102.5]); the vertical ones likewise on y. Scale 4 on top.
    let scene = Scene::new(Point::new(0, 0), 4.0);
    let quad = Quadrilateral::from_rect(RectangleF::new(0.0, 0.0, 100.0, 100.0));
    let mapper = ProjectiveMapper::new(&quad, &quad).expect("identity mapping");
    let grid = Grid {
        mapper: &mapper,
        width: 100.0,
        height: 100.0,
        cols: 2,
        rows: 2,
    };
    let pen = Pen::new(COLOR, 1.0);
    let calls = scene.record(0, |ctx, r| draw_grid(r, &pen, &grid, false, &ctx));
    let drawn: Vec<(PointF, PointF)> = calls
        .iter()
        .filter_map(|c| match c {
            Call::StrokeLine(a, b, _, _) => Some((*a, *b)),
            _ => None,
        })
        .collect();
    assert_eq!(
        drawn,
        vec![
            (p(0.0, 0.0), p(400.0, 0.0)),
            (p(0.0, 400.0), p(400.0, 400.0)),
            (p(0.0, 0.0), p(0.0, 400.0)),
            (p(400.0, 0.0), p(400.0, 400.0)),
            (p(-10.0, 200.0), p(10.0, 200.0)),
            (p(390.0, 200.0), p(410.0, 200.0)),
            (p(200.0, -10.0), p(200.0, 10.0)),
            (p(200.0, 390.0), p(200.0, 410.0)),
        ]
    );
}
