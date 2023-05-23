use leptos::*;
use leptos::ev::Event;
use crate::vec2d::Vec2D;

const UNIT_VECS: [(Vec2D, Vec2D); 4] = [
    (Vec2D { x: 1.0, y: 0.0 }, Vec2D { x: -1.0, y: -1.0 }),
    (Vec2D { x: 0.0, y: -1.0 }, Vec2D { x: -1.0, y: 1.0 }),
    (Vec2D { x: -1.0, y: 0.0 }, Vec2D { x: 1.0, y: 1.0 }),
    (Vec2D { x: 0.0, y: 1.0 }, Vec2D { x: 1.0, y: -1.0 }),
];

#[component]
pub fn App(cx: Scope) -> impl IntoView {

    let size_of_square = create_rw_signal(cx, 500);
    let number_of_lines = create_rw_signal(cx, 100);

    let lines = create_rw_signal(cx, Vec::with_capacity(1000));

    let center = move || {
        let size = size_of_square.get();
        size / 2
    };

    let lower_border = move || {
        let size = size_of_square.get();
        (size as f32 * 0.05) as i32
    };
    let higher_border = move || {
        let size = size_of_square.get();
        (size as f32 * 0.95) as i32
    };

    let border_len = move || (size_of_square.get() as f32 / 2.0 - lower_border() as f32) as f32;

    let compute_lines = move || {
        let mut vec_of_lines = lines.get();
        vec_of_lines.clear();

        for (va, vb) in UNIT_VECS {
            let n = number_of_lines.get();

            let va = va * (size_of_square.get() as f32 / 2.0 - lower_border() as f32);
            let va = va + Vec2D { x: center() as f32, y: center() as f32 };
            let vb = vb * border_len();

            for index in 1..=n {
                let point_of_line_on_border = va + vb * (index as f32 / (n as f32 + 1.0));
                vec_of_lines.push(point_of_line_on_border);
            }
        }

        lines.set(vec_of_lines);
    };

    compute_lines();

    view! {cx,
        <div>
            <p>"Size of the square:"</p>
            <input type="range" min="1" max="1000" prop:value=size_of_square.get() on:input=move |event: Event| {
                let value = event_target_value(&event);
                size_of_square.set(value.parse::<i32>().unwrap());
                compute_lines();
            }></input>
            <p>{move || size_of_square.get()}</p>
        </div>
        <div>
            <p>"Number of lines through the center:"</p>
            <input type="range" min="1" max="200" prop:value=number_of_lines.get() on:input=move |event: Event| {
                let value = event_target_value(&event);
                number_of_lines.set(value.parse::<i32>().unwrap());
                compute_lines();
            }></input>
            <p>{move || number_of_lines.get()}</p>
        </div>
        <svg width="1000" height="1000">
            <rect width="1000" height="1000" fill="lightblue"></rect>
            {
                move || {
                    lines.get().iter()
                        .map(|line| {
                            view! {cx,
                                <line x1=center y1=center x2={line.x} y2={line.y} stroke="black"></line>
                            }
                        })
                        .collect::<Vec<_>>()
                }
            }
            <line id="top" x1=center y1=center x2=center y2=lower_border stroke="black"></line>
            <line id="right" x1=center y1=center x2=higher_border y2=center stroke="black"></line>
            <line id="bottom" x1=center y1=center x2=center y2=higher_border stroke="black"></line>
            <line id="left" x1=center y1=center x2=lower_border y2=center stroke="black"></line>
            <line id="tr" x1=center y1=lower_border x2=higher_border y2=center stroke="black"></line>
            <line id="br" x1=higher_border y1=center x2=center y2=higher_border stroke="black"></line>
            <line id="bl" x1=center y1=higher_border x2=lower_border y2=center stroke="black"></line>
            <line id="tl" x1=lower_border y1=center x2=center y2=lower_border stroke="black"></line>
        </svg>
    }
}
