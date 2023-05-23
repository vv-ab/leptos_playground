use leptos::*;
use nalgebra::{Matrix2, Point2, Vector2};
// use leptos::ev::Event;

#[component]
pub fn LineChart(cx: Scope) -> impl IntoView {

    let data = vec![
        (0.0, 0.0),
        (1.66, 2.44),
        (5.0, 1.0),
        (6.4, 5.0),
        (10.0, 3.8),
        (16.3, 5.3)
    ];
    let max_x = data.iter().map(|(x, _)| *x).reduce(f32::max).unwrap();
    let max_y = data.iter().map(|(_, y)| *y).reduce(f32::max).unwrap();
    let min_x = data.iter().map(|(x, _)| *x).reduce(f32::min).unwrap();
    let min_y = data.iter().map(|(_, y)| *y).reduce(f32::min).unwrap();

    let width = 500.0;
    let height = 500.0;
    let padding = 50.0;

    let translation: Vector2<f32> = Vector2::new(
        padding + -1.0 * min_x * (width - 2.0 * padding) / (max_x - min_x),
        height - padding + min_y * (height + 0.5 * padding) / (max_y - min_y)
    );
    let scale: Matrix2<f32> = Matrix2::new(
        (width - 2.0 * padding) / (max_x - min_x), 0.0,
        0.0, -1.0 * ((height - 2.0 * padding) / (max_y - min_y))
    );

    let origin = scale * Point2::new(0.0, 0.0) + translation;

    let points = data.iter().map(|(x, y)| {
       let point = Point2::new(*x, *y);
        (scale * point) + translation;
    }).collect::<Vec<Point2<f32>>>();

    let circles = points.iter().zip(data).map(|(point, (x, y))| {
        view! {cx,
            <circle cx=point.x cy=point.y r="5" fill="red"></circle>
            <text x=point.x y=point.y>{format!("{:.2},{:.2} ~ {:.2},{:.2}", point.x, point.y, x, y)}</text>
        }
    }).collect::<Vec<_>>();

    view! {cx,
        <svg width=width height=height>
            <rect width=width height=height fill="none" stroke="black"></rect>
            <path id="YAxis" fill="none" d="M500,0 L500,-500 z"></path>
            <text stroke="black">
                <textPath href="#YAxis">"Wasserstand"</textPath>
            </text>
            {circles}
            <text x="0" y="20" class="small">"Datum 0"</text>
            <text x="100" y="20" class="small">"Datum 1"</text>
            <text x="200" y="20" class="small">"Datum 2"</text>
            <rect width=move || width - 2.0 * padding height= move || height - 2.0 * padding x=padding y=padding fill="none" stroke="black"></rect>
            <text x="300" y="20" class="small">"Datum 3"</text>
        </svg>
    }
}

