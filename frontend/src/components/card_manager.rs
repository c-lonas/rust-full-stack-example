use yew::prelude::*;
use crate::components::card_components::add_income_form::AddIncomeForm;
use crate::components::card_components::manage_demo_profiles_card::ManageDemoProfilesCard;
use crate::active_card::{CardType};

#[derive(Properties, Clone, PartialEq, Default)]
pub struct Props {
    pub active_card: Option<CardType>,
    pub on_close: Callback<MouseEvent>,
}

#[function_component(CardManager)]
pub fn card_manager(props: &Props) -> Html {
    match &props.active_card {
        None => html! { <div>{""}</div> }, // Return an empty div if there is no active card (otherwise none of the cards will render)
      
        Some(card_type) => match card_type {
            CardType::AddIncomeForm => {
                html! {
                    <div class="card-container">
                        <div class="card-overlay" onclick={props.on_close.clone()}></div>
                        <div class="form-wrapper">
                            <AddIncomeForm on_close={props.on_close.clone()} />
                        </div>
                    </div>
                }
            }
            CardType::ManageDemoProfiles => {
                html! {
                    <div class="card-container">
                        <div class="card-overlay" onclick={props.on_close.clone()}></div>
                        <div class="form-wrapper">
                            <ManageDemoProfilesCard on_close={props.on_close.clone()} />
                        </div>
                    </div>
                }
            }
        },
    }
}
