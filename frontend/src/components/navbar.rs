use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub on_add_income_click: Callback<MouseEvent>,
}


#[function_component(Navbar)]
pub fn navbar(props: &Props) -> Html {
    html! {
        <nav class="navbar">
            <ul>
                <li>
                    <button onclick={props.on_add_income_click.clone()}>{ "Add Income" }</button>
                </li>
                <li><button>{ "Add Expense" }</button></li>
                <li><button>{ "Create Widget" }</button></li>
                <li><button>{ "Visualize Data" }</button></li>
            </ul>
        </nav>
    }
}
