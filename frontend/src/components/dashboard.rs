use std::rc::Rc;
use std::cell::RefCell;
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwest::Client;

use shared::models::income::Income;
use crate::components::tooltip::Tooltip;

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
        })
    };

    let income_sources = use_state(|| Rc::new(RefCell::new(Vec::<Income>::new())));

    let total_income_clone = total_income.clone();
    let income_sources_clone = income_sources.clone();

    // Tooltip config
    let tooltip_content = use_state(|| "".to_string());
    let tooltip_x = use_state(|| 0_i32);
    let tooltip_y = use_state(|| 0_i32);
    let tooltip_visible = use_state(|| false);

    let onmouseover = {
        let tooltip_content = tooltip_content.clone();
        let tooltip_visible = tooltip_visible.clone();
        let tooltip_x = tooltip_x.clone();
        let tooltip_y = tooltip_y.clone();
        let income_sources_cb = income_sources.clone();
        Callback::from(move |(index, x, y): (usize, i32, i32)| { // Update the parameter type
            let income_sources = income_sources_cb.borrow();
            let income = &income_sources[index];
            tooltip_content.set(format!("income desciption: {}", income.description.clone().unwrap_or("".to_string())));
            tooltip_x.set(x);
            tooltip_y.set(y);
            tooltip_visible.set(true);
        })
    };

    let onmouseout = {
        let tooltip_visible = tooltip_visible.clone();
        Callback::from(move |_: usize| {
            tooltip_visible.set(false);
        })
    };

    use_effect_with_deps(
        move |_| {
            let selected_user_id = selected_user_id.clone();

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
                                    income_sources_clone.set(Rc::new(RefCell::new(incomes)));
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
            {
                if *tooltip_visible {
                    html! { <Tooltip content={tooltip_content.to_string()} x={*tooltip_x} y={*tooltip_y} /> }
                } else {
                    html! {}
                }
            }
            <div class="dashboard-item">
                <div class="dashboard-card">
                    <h3>{ "Total Income" }</h3>
                    <p>{ format!("User's total income: {}", *display_total_income) }</p>
                    <button onclick={toggle_expanded.clone()}>{ "Toggle Income Details" }</button>
                    {
                        if *is_expanded {
                            let income_sources = income_sources.clone();
                            html! {
                                <div class="income-sources">
                                    { render_income_sources(&income_sources, &onmouseover, &onmouseout) }
                                </div>
                            }

                        } else {
                            html! {}
                        }
                    }

                </div>
            </div>
            <div class="dashboard-item">
                <div class="dashboard-card">
                    <h3>{ "Total Expenses" }</h3>
                    <p>{ "Placeholder for total expenses value" }</p>
                </div>
            </div>
            <div class="dashboard-item">
                <div class="dashboard-card">
                    <h3>{ "Savings" }</h3>
                    <p>{ "Placeholder for savings value" }</p>
                </div>
            </div>
        </section>
    }
}


fn render_income_sources(
    income_sources: &Rc<RefCell<Vec<Income>>>,
    onmouseover: &Callback<(usize, i32, i32)>,
    onmouseout: &Callback<usize>,
) -> Html {
    let income_sources = income_sources.borrow();
    html! {
        for income_sources.iter().enumerate().map(|(index, income)| {
            let onmouseover_with_index = onmouseover.clone().reform(move |e: MouseEvent| (index, e.client_x(), e.client_y()));
            let onmouseout_with_index = onmouseout.clone().reform(move |_| index);
            html! {
                <div class="income-source" onmouseover={onmouseover_with_index} onmouseout={onmouseout_with_index}>
                    <p>{ &income.name }</p>
                    <p>{ income.amount }</p>
                </div>
            }
        })
    }
}