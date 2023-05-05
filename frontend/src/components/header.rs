use yew::prelude::*;
use dotenvy::dotenv;
// use std::env;

// use log::info;
// use web_logger;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub on_manage_demo_profiles_click: Callback<MouseEvent>,
    pub user_name: Option<String>,
}

#[function_component(Header)]
pub fn header(props: &Props) -> Html {

    let unknown_user = String::from("Unknown");
    let user_name = props.user_name.as_ref().unwrap_or(&unknown_user);

    dotenv().ok();

  
    html! {
        <header class="main-header">
            <h1>{ format!("Current Demo User: {}", user_name) }</h1>
            <nav class="main-nav">
                <ul>
                    <li>
                        <button onclick={props.on_manage_demo_profiles_click.clone()}>{ "Manage Demo Profiles" }</button>
                    </li>
                </ul>
            </nav>
            <nav class="setting-nav">
                <ul>
                    
                    <li><a href="#">{ "Settings" }</a></li>
                </ul>
            </nav>
        </header>
    }
}
