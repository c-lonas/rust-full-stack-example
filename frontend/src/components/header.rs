use yew::prelude::*;
use dotenvy::dotenv;
// use std::env;

// use log::info;
// use web_logger;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub on_manage_demo_profiles_click: Callback<MouseEvent>,
}

#[function_component(Header)]
pub fn header(props: &Props) -> Html {

    dotenv().ok();

  
    html! {
        <header class="main-header">
            <h1>{ "Personal Finance Tracker" }</h1>
            <nav class="main-nav">
                <ul>
                    <li><a href="#">{ "Dashboard" }</a></li>
                    <li><a href="#">{ "Overview" }</a></li>
                    <li><a href="#">{ "Details" }</a></li>
                </ul>
            </nav>
            <nav class="setting-nav">
                <ul>
                    <li>
                        <button onclick={props.on_manage_demo_profiles_click.clone()}>{ "Manage Demo Profiles" }</button>
                    </li>
                    <li><a href="#">{ "Settings" }</a></li>
                </ul>
            </nav>
        </header>
    }
}
