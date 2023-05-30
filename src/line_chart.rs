use leptos::*;
use leptos::svg::Text;
use nalgebra::{Matrix2, point, Point2, Vector2};

#[component]
pub fn LineChart(cx: Scope) -> impl IntoView {

    let data = {
        let mut data = vec![
            (-10.0, 0.0),
            (-1.66, 2.44),
            (5.0, 1.0),
            (6.4, 5.0),
            (-8.0, 3.8),
            (1.9, 5.3)
        ];
        data.sort_by(|(ax, _), (bx, _)| ax.partial_cmp(bx).unwrap());
        data
    };

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
        (scale * point) + translation
    }).collect::<Vec<Point2<f32>>>();

    let circles = points.iter().zip(data).map(|(point, (x, y))| {
        view! {cx,
            <circle cx=point.x cy=point.y r="3" fill="red"></circle>
            // <text x=point.x y=point.y>{format!("{:.2},{:.2} ~ {:.2},{:.2}", point.x, point.y, x, y)}</text>
        }
    }).collect::<Vec<_>>();

    let lines = points.iter().take(points.len() - 1).zip(points.iter().skip(1)).map(|(start_point, end_point)| {
        view! {cx,
            <line x1=start_point.x y1=start_point.y x2=end_point.x y2=end_point.y stroke="black"></line>
        }
    }).collect::<Vec<_>>();


    let label_x =
        (((min_x * 10.0) as i32)..((max_x * 10.0) as i32))
            .step_by(1 * 10)
            .map(|value| {
                let text_y = height - 0.5 * padding;
                let point = Point2::new(value as f32 / 10.0, 0.0);
                let position = (scale * point) + translation;
                view! {cx,
                    <line x1=position.x y1=(text_y - 20.0) x2=position.x y2=(text_y - 30.0) stroke="black"></line>
                    <text x=position.x y=text_y class="small">{value / 10}</text>
                }
            })
            .collect::<Vec<_>>();

    let label_y =
        (((min_y * 10.0) as i32)..((max_y * 10.0) as i32))
            .step_by(1 * 10)
            .map(|value| {
                let text_x = 0.5 * padding;
                let point = Point2::new(0.0, value as f32 / 10.0);
                let position = (scale * point) + translation;
                view! {cx,
                    <line y1=position.y x1=(padding - 5.0) y2=position.y x2=(padding + 5.0) stroke="black"></line>
                    <text y=position.y x=text_x class="small">{value / 10}</text>
                }
            })
            .collect::<Vec<_>>();

    view! {cx,
        <svg width=width height=height>
            <rect width=width height=height fill="none" stroke="black"></rect>
            <path id="YAxis" fill="none" d="M500,0 L500,-500 z"></path>
            <text stroke="black">
                <textPath href="#YAxis">"Wasserstand"</textPath>
            </text>
            {lines}
            {circles}
            {label_x}
            {label_y}
            <rect width=move || width - 2.0 * padding height= move || height - 2.0 * padding x=padding y=padding fill="none" stroke="black"></rect>
        </svg>
    }
}

