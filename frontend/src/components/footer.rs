use yew::prelude::*;


#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="footer">
            <p>{ "Personal Finance Tracker © 2023" }</p>
        </footer>
    }
}
