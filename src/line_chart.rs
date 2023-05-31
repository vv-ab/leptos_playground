use futures::SinkExt;
use leptos::*;
use leptos::ev::MouseEvent;
use leptos::svg::Text;
use nalgebra::{Matrix2, point, Point2, Vector2};

#[component]
pub fn LineChart(cx: Scope) -> impl IntoView {

    let crosshair_position = create_rw_signal(cx, Point2::new(0.0, 0.0));
    let crosshair_enabled = create_rw_signal(cx, false);

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

    let circle_data = {
        let data = points.iter().zip(data).enumerate().map(|(index, (point, (x, y)))| {
            ChartCircleData {
                x_value: x,
                y_value: y,
                position: *point,
                selected: false,
            }
        }).collect::<Vec<ChartCircleData>>();
        create_rw_signal(cx, data)
    };

    let circles = move || {
        circle_data.with(|circles| {
            circles.iter().cloned().enumerate().map(|(index, data)| {
                view! {cx,
                    <ChartCircle data=data on_select=move |is_selected| {
                        circle_data.update(|circles| {
                            circles.iter_mut().enumerate().for_each(|(i, circle)| {
                                if i == index {
                                    circle.selected = is_selected;
                                }
                                else {
                                    circle.selected = false;
                                }
                            });
                        });
                    }></ChartCircle>
                }
            }).collect::<Vec<_>>()
        })
    };

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

    let crosshair = move || {
        crosshair_position.with(|position| {
            if crosshair_enabled.get_untracked() {
                let x = if position.x > (width - padding) {
                    width - padding
                } else if position.x < padding {
                    padding
                } else {
                    position.x
                };
                let y = if position.y > (height - padding) {
                    height - padding
                } else if position.y < padding {
                    padding
                } else {
                    position.y
                };
                let non_scaled_value =  scale.try_inverse().unwrap() * (Point2::new(x, y)  - translation) ;
                Some(view! {cx,
                    <line x1=padding y1=y x2=(width - padding) y2=y stroke="grey"></line>
                    <line x1=x y1=padding x2=x y2=(height - padding) stroke="grey"></line>
                    <text x=padding y={padding - 5.0} style="font-size: 13px" fill="grey">{format!("{:.2}/{:.2}", non_scaled_value.x, non_scaled_value.y)}</text>
                })
            }
            else {
                None
            }
        })
    };

    view! {cx,
        <svg width=width height=height on:mousemove=move |event: MouseEvent| {
            crosshair_enabled.set(event.ctrl_key());
            crosshair_position.set(Point2::new(event.offset_x() as f32, event.offset_y() as f32));
        }>
            <rect width=width height=height fill="none" stroke="black"></rect>
            <path id="YAxis" fill="none" d="M500,0 L500,-500 z"></path>
            <text stroke="black">
                <textPath href="#YAxis">"Wasserstand"</textPath>
            </text>
            {lines}
            {circles}
            {label_x}
            {label_y}
            {crosshair}
            <rect width=move || width - 2.0 * padding height= move || height - 2.0 * padding x=padding y=padding fill="none" stroke="black"></rect>
        </svg>
    }
}

#[derive(Clone)]
struct ChartCircleData {
    x_value: f32,
    y_value: f32,
    position: Point2<f32>,
    selected: bool,
}

#[component]
fn chart_circle<A>(cx: Scope, data: ChartCircleData, on_select: A) -> impl IntoView
where A: Fn(bool) -> () + 'static + Copy {

    let radius = if data.selected {
        10.0
    }
    else {
        5.0
    };

    let info = move || {

        if data.selected {
            let rect_position = data.position + Vector2::new(1.5 * radius, 0.0);
            let text_position = rect_position + Vector2::new(10.0, 10.0);

            Some(view! {cx,
                <rect width="120" height="3.7em" x=rect_position.x y=rect_position.y rx=3 fill="#260D41" stroke="#260D41"></rect>
                <text dominant-baseline="hanging" x=text_position.x y=text_position.y fill="white">
                    <tspan x=text_position.x dy="0em">{format!("Date: {:.3}", data.x_value)}</tspan>
                    <tspan x=text_position.x dy="1.7em">{format!("Data: {:.3}", data.y_value)}</tspan>
                </text>
            })
        }
        else {
            None
        }
    };

    view! {cx,
        <circle cx=data.position.x cy=data.position.y r=radius fill="red"
            on:mouseenter=move |_| { on_select(true) }
            on:mouseleave=move |_| { on_select(false) }>
        </circle>
        {info}
    }
}

