use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwest::Client;
use log::info;

use shared::models::income::Income;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub selected_user_id: Option<u32>,
}

#[function_component(Dashboard)]
pub fn dashboard(props: &Props) -> Html {
    let total_income = use_state(|| 0_u32);
    let selected_user_id = props.selected_user_id;
    let is_expanded = use_state(|| false);


    let toggle_expanded = {
    let is_expanded = is_expanded.clone();
        Callback::from(move |_| {
            is_expanded.set(!*is_expanded);
            // info!("Toggle Income Details clicked, is_expanded: {}", *is_expanded);
        })
    };


    let income_sources = use_state(|| Vec::<Income>::new());

    let total_income_clone = total_income.clone();
    let income_sources_clone = income_sources.clone();

    use_effect_with_deps(
        move |_| {
            let selected_user_id = selected_user_id.clone();
            // let total_income = total_income.clone();

            let effect = async move {
                if let Some(user_id) = selected_user_id {
                    let client = Client::new();
                    let url = format!("http://localhost:8000/api/income/user/{}", user_id);
                    match client.get(&url).send().await {
                        Ok(resp) => {
                            if resp.status().is_success() {
                                if let Ok(incomes) = resp.json::<Vec<Income>>().await {
                                    let total: u32 = incomes.iter().map(|income| income.amount).sum();
                                    total_income_clone.set(total);
                                    income_sources_clone.set(incomes);
                                }
                            } else {
                                log::warn!("Failed to fetch income data");
                                total_income_clone.set(0);
                            }
                        }
                        Err(_) => {
                            log::warn!("Failed to fetch income data");
                            total_income_clone.set(0);
                        }
                    }
                }
            };

            spawn_local(effect);
            || ()
        },
        selected_user_id,
    );

    let display_total_income = total_income.clone();

    html! {
        <section class="dashboard">
            <div class="dashboard-card">
                <h3>{ "Total Income" }</h3>
                <p>{ format!("User's total income: {}", *display_total_income) }</p>
                <button onclick={toggle_expanded.clone()}>{ "Toggle Income Details" }</button>
                {
                    if *is_expanded {
                        let display_income_sources = income_sources.clone();
                        html! {
                            <ul>
                                { for display_income_sources.iter().map(|income| {
                                    let onmouseover = Callback::from({
                                        let income_description = income.description.clone();
                                        move |_| {
                                            info!("{:?}", income_description.as_deref().unwrap_or(""));
                                        }
                                    });
                                    html! {
                                        <li onmouseover={onmouseover}>
                                            <span>{ &income.name }</span>
                                            <span>{ format!("${}", income.amount) }</span>
                                        </li>
                                    }
                                })}
                            </ul>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
            <div class="dashboard-card">
                <h3>{ "Total Expenses" }</h3>
                <p>{ "Placeholder for total expenses value" }</p>
            </div>
            <div class="dashboard-card">
                <h3>{ "Savings" }</h3>
                <p>{ "Placeholder for savings value" }</p>
            </div>
            <div class="dashboard-card">
                <h3>{ "Budget" }</h3>
                <p>{ "Placeholder for budget value" }</p>
            </div>
        </section>
    }
}