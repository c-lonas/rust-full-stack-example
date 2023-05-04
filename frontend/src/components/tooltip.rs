use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub content: String,
    pub x: i32,
    pub y: i32,
}

#[function_component(Tooltip)]
pub fn tooltip(props: &Props) -> Html {
    html! {
        <div class="tooltip" style={format!("left:{}px;top:{}px;", props.x, props.y)}>
            { &props.content }
        </div>
    }
}
