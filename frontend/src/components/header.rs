use yew::prelude::*;
use dotenvy::dotenv;
// use std::env;

// use log::info;
// use web_logger;

#[function_component(Header)]
pub fn header() -> Html {

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
            <nav class="auth-nav">
                <ul>
                    <li><a href="#">{ "Manage Demo Profiles" }</a></li>
                    <li><a href="#">{ "Settings" }</a></li>
                </ul>
            </nav>
        </header>
    }
}
