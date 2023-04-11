use yew::prelude::*;

#[function_component(MainContent)]
pub fn main_content() -> Html {
    html! {
        <main>
            <h2>{ "Welcome to the Personal Finance Tracker!" }</h2>
            <p>{ "Start by managing your incomes and expenses." }</p>
        </main>
    }
}