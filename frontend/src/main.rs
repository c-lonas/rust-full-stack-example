#[macro_use]
extern crate log;
extern crate web_logger;

mod components {
    pub mod footer;
    pub mod header;
    pub mod main_content;
    pub mod navbar;
    pub mod dashboard;
    pub mod card_manager;
    pub mod card_components;
}

mod api_error;
mod active_card;

use yew::prelude::*;
use yew::create_portal;
use gloo::utils::document;
use wasm_bindgen::JsCast;
use web_sys::Element;
use dotenvy::dotenv;
use dotenvy_macro::dotenv;
// use std::env;
// use yew_oauth2::prelude::*;
use yew_oauth2::oauth2::*;

use components::footer::Footer;
use components::header::Header;
use components::navbar::Navbar;
use components::dashboard::Dashboard;
use components::card_manager::CardManager;
use active_card::{ ActiveCard, CardType };


#[function_component(App)]
fn app() -> Html {
    dotenv().ok();

    // Auth config
    let client_id = dotenv!("GOOGLE_CLIENT_ID");
    let config = Config {
        client_id: client_id.into(),
        auth_url: "https://accounts.google.com/o/oauth2/auth".into(),
        token_url: "https://oauth2.googleapis.com/token".into(),
    };


    // Card manager config
    let active_card = use_state(|| ActiveCard::new());

    let show_add_income_form = {
        let active_card = active_card.clone();
        Callback::from(move |_| {
            info!("Show Add Income Form");
            active_card.set(ActiveCard { card: Some(CardType::AddIncomeForm) });
        })
    };

    let close_active_card = {
        let active_card = active_card.clone();
        Callback::from(move |_| {
            info!("Close Active Card");
            active_card.set(ActiveCard { card: None });
        })
    };

    html! {
        <OAuth2 config={config}>
            <main>
                <Header />
                <Navbar on_add_income_click={ show_add_income_form.clone() }/>
                {
                    create_portal(
                        html! { <CardManager active_card={(*active_card).card.clone()} on_close={close_active_card.clone()} /> },
                        document().body().unwrap().dyn_into::<Element>().unwrap(),
                    )
                }
                <Dashboard />
                <Footer />
            </main>
        </OAuth2>
    }
}

fn main() {
    web_logger::init();
    yew::Renderer::<App>::new().render();
}