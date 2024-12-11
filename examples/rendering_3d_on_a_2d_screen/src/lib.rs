pub mod colors;
pub mod data;
pub mod triangles;

use std::cmp::Ordering;

use colors::COLORS;
use data::{
    degrees_to_radians, point_of_intersection_of_plane_and_normal_line, Plane, PointOrVector, Ray,
    CAMERA_DIRECTION, CAMERA_POS, PLANE_CONTAINING_X_AXIS_AND_CAMERA,
};
use embedded_graphics_web_simulator::{
    display::WebSimulatorDisplay, output_settings::OutputSettingsBuilder,
};
use gloo_timers::future::TimeoutFuture;
use js_sys::wasm_bindgen::JsValue;
use triangles::get_triangles;
use wasm_bindgen::prelude::*;

use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::Rgb888,
    prelude::{Point, RgbColor, Size, WebColors},
    primitives::{Circle, PrimitiveStyle, Rectangle, StyledDrawable, Triangle},
};
use wasm_bindgen_futures::spawn_local;
// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let document = web_sys::window()
        .expect("could not get window")
        .document()
        .expect("could not get document");
    let body = document.body().expect("could not get document body");

    // for simplicity reasons, this example uses `cargo-run-wasm`, which doesn't allow
    // custom html - so it's augmented here inline. In a real project, you'd likely use `trunk` instead.
    body.set_inner_html(
        r#"
    <header>
    3D First person (Very basic) Drawing
  </header>

  <div id="custom-container"></div>
  <footer>
    <a href="https://github.com/ChocolateLoverRaj/embedded-graphics-web-simulator">Link to GitHub repository</a>
    <a href="https://github.com/embedded-graphics/simulator" target="_blank">Embedded Graphics</a>
  </footer>
    "#,
    );

    spawn_local(async {
        let output_settings = OutputSettingsBuilder::new().scale(1).build();
        let screen_size = 800;
        let mut display =
            WebSimulatorDisplay::new((screen_size, screen_size), &output_settings, None);

        let viewing_plane_x = {
            let fov = degrees_to_radians(90.0);
            let opposite = 1.0;
            let angle = fov / 2.0;
            let adjacent = opposite / angle.tan();
            CAMERA_POS.x - adjacent
        };
        let viewing_plane = Plane {
            a: viewing_plane_x,
            b: 0.0,
            c: 0.0,
            d: 0.0,
        };
        let mut color_iter = COLORS.iter().cycle();
        loop {
            display.clear(Rgb888::BLACK).unwrap();
            // Draw a crosshair
            let crosshair_thickness = 5;
            let crosshair_size = 50;
            let center = Point::new_equal((screen_size / 2) as i32);
            let crosshair_style = PrimitiveStyle::with_fill(Rgb888::CSS_GRAY);
            // Vertical crosshair
            Rectangle::with_center(center, Size::new(crosshair_thickness, crosshair_size))
                .draw_styled(&crosshair_style, &mut display)
                .unwrap();
            // Horizontal crosshair
            Rectangle::with_center(center, Size::new(crosshair_size, crosshair_thickness))
                .draw_styled(&crosshair_style, &mut display)
                .unwrap();
            let triangles_to_draw = {
                let sorted_triangles = {
                    let mut sorted_triangles = vec![];
                    enum TriangleOrdering {
                        InFront,
                        Behind,
                        NoOverlap,
                    }
                    let get_ordering =
                        |points0: &[PointOrVector; 3], points1: &[PointOrVector; 3]| {
                            let mut i = 0;
                            loop {
                                match Ray::from_start_to_end(CAMERA_POS, points0[i])
                                    .multiplier_to_plane_intersection(Plane::from_3_points(
                                        *points1,
                                    )) {
                                    Some(multiplier) => match multiplier.partial_cmp(&1.0) {
                                        Some(Ordering::Greater) => break TriangleOrdering::InFront,
                                        Some(Ordering::Less) => break TriangleOrdering::Behind,
                                        _ => {}
                                    },
                                    None => {}
                                }
                                i += 1;
                                if i == points0.len() {
                                    break TriangleOrdering::NoOverlap;
                                }
                            }
                        };
                    for triangle in get_triangles() {
                        sorted_triangles.insert(
                            {
                                if sorted_triangles.len() == 0 {
                                    0
                                } else {
                                    let mut i = 0;
                                    loop {
                                        match sorted_triangles.get(i) {
                                            Some(triangle2) => {
                                                match get_ordering(&triangle, triangle2) {
                                                    TriangleOrdering::Behind => break,
                                                    _ => {}
                                                }
                                            }
                                            None => {}
                                        }
                                        i += 1;
                                        if i == sorted_triangles.len() {
                                            break;
                                        }
                                    }
                                    i
                                }
                            },
                            triangle,
                        );
                    }
                    sorted_triangles
                };
                sorted_triangles
                    .into_iter()
                    .map(|points| {
                        points.map(|point| {
                            let vector_from_camera_to_point = point - CAMERA_POS;
                            let point_on_viewing_plane = {
                                let x_distance = viewing_plane_x - CAMERA_POS.x;
                                let scale = x_distance / vector_from_camera_to_point.x;
                                let vector_to_point_on_viewing_plane =
                                    vector_from_camera_to_point * scale;
                                CAMERA_POS + vector_to_point_on_viewing_plane
                            };
                            let point_on_screen = Point::new(
                                (screen_size as f64 / 2.0
                                    + point_on_viewing_plane.y * screen_size as f64 / 2.0)
                                    as i32,
                                (screen_size as f64 / 2.0
                                    - point_on_viewing_plane.z * screen_size as f64 / 2.0)
                                    as i32,
                            );
                            point_on_screen
                        })
                    })
                    .collect::<Vec<_>>()
            };
            for triangle in triangles_to_draw {
                let triangle_color = color_iter.next().unwrap();
                for point_on_screen in triangle {
                    Circle::with_center(point_on_screen, 5)
                        .draw_styled(&PrimitiveStyle::with_fill(*triangle_color), &mut display)
                        .unwrap();
                    display.flush().unwrap();
                    TimeoutFuture::new(300).await;
                }
                Triangle::new(triangle[0], triangle[1], triangle[2])
                    .draw_styled(&PrimitiveStyle::with_fill(*triangle_color), &mut display)
                    .unwrap();
                display.flush().unwrap();
                TimeoutFuture::new(600).await;
            }
            TimeoutFuture::new(10000).await;
        }
    });

    Ok(())
}
