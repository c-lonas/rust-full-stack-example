use yew::prelude::*;
use yew_oauth2::prelude::*;
use yew_oauth2::agent::{Agent, AgentConfiguration, OAuth2Operations, OAuth2Client};
use yew_oauth2::oauth2::{Config};
use std::time::Duration;
use dotenvy::dotenv;
use dotenvy_macro::dotenv;
// use std::env;

// use log::info;
// use web_logger;

#[function_component(Header)]
pub fn header() -> Html {

    dotenv().ok();


    let client_id = dotenv!("GOOGLE_CLIENT_ID");

    let agent = use_state(|| {
        let config = Config {
            client_id: client_id.into(),
            auth_url: "https://accounts.google.com/o/oauth2/auth".into(),
            token_url: "https://oauth2.googleapis.com/token".into(),
        };

        let agent_config: AgentConfiguration<OAuth2Client> = AgentConfiguration {
            config,
            scopes: vec!["email".into(), "profile".into()],
            grace_period: Duration::from_secs(60),
        };

        let agent = Agent::new(|_| {});
        let _ = agent.configure(agent_config);

        agent
    });

    let login = {
        let agent = agent.clone();
        Callback::from(move |_| {
            let _ = agent.start_login();
        })
    };

    let logout = Callback::from(move |_| {
        let _ = agent.logout();
    });

  
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
                    <NotAuthenticated>
                        <li><button onclick={login}>{ "Sign In" }</button></li>
                    </NotAuthenticated>
                    <Authenticated>
                        <li><button onclick={logout}>{ "Sign Out" }</button></li>
                    </Authenticated>
                    <li><a href="#">{ "Settings" }</a></li>
                </ul>
            </nav>
        </header>
    }
}
