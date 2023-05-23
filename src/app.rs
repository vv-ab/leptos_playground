use leptos::*;
use leptos_router::*;
use crate::geometrical_figure::GeometricalFigure;
use crate::line_chart::LineChart;
use crate::example::Example;

#[component]
pub fn App(cx: Scope) -> impl IntoView {

    view! {cx,
        <div>
            <a href="/a">"go to geometrical figure"</a>
        </div>
        <div>
            <a href="/b">"go to line chart"</a>
        </div>
        <div>
            <a href="/c">"go to example"</a>
        </div>
        <Router>
            <Routes>
                <Route path="/a" view=move |cx| view! {cx, <GeometricalFigure></GeometricalFigure> }></Route>
                <Route path="/b" view=move |cx| view! {cx, <LineChart></LineChart> }></Route>
                <Route path="/c" view=move |cx| view! {cx, <Example></Example> }></Route>
            </Routes>
        </Router>
    }
}

