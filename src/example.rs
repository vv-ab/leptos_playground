use leptos::*;
use nalgebra::{Matrix2, Point2, Vector2};
// use leptos::ev::Event;

#[component]
pub fn Example(cx: Scope) -> impl IntoView {

    let width = 800.0;
    let height = 500.0;

    let translated_x = 100.0;
    let translated_y = 100.0;
    let translated_width = width / 2.0;
    let translated_height = height / 2.0;

    let origin = Vector2::new(0.0, 500.0);

    let circle_a = Point2::new(550.0, 320.0);
    let translation: Vector2<f32> = Vector2::new(translated_x, translated_y + translated_height);
    let reflect = Matrix2::new(
        1.0, 0.0,
        0.0, -1.0
    );
    let scale = Matrix2::new(
        translated_width / width, 0.0,
        0.0, translated_height / height
    );
    // (2x2) (2x1)
    let circle_b = (scale * reflect) * circle_a + translation;


    let radius = 5;

    let coords = Vector2::new(150.0, 150.0);
    //let circle_c = Point2::from(coords);

    view! {cx,
        <svg width="800" height="500">
            <rect width=width height=height fill="none" stroke="green"></rect>
            <rect width=translated_width height=translated_height x=translated_x y=translated_y fill="none" stroke="red"></rect>
            <circle cx=circle_a.x cy=circle_a.y r=radius fill="green"></circle>
            <circle cx=circle_b.x cy=circle_b.y r=radius fill="red"></circle>
            //<circle cx=circle_c.x cy=circle_c.y r=radius></circle>
        </svg>
    }
}

