use yew::prelude::*;
use crate::components::card_components::add_income_form::AddIncomeForm;
use crate::components::card_components::manage_demo_profiles_card::ManageDemoProfilesCard;
use crate::active_card::{CardType};

#[derive(Properties, Clone, PartialEq, Default)]
pub struct Props {
    pub active_card: Option<CardType>,
    pub on_close: Callback<MouseEvent>,
    pub selected_user_id: Option<u32>,
    pub user_name: Option<String>,
    pub on_selected_user_id_update: Callback<Option<u32>>,
    pub on_selected_user_name_update: Callback<Option<String>>
}

#[function_component(CardManager)]
pub fn card_manager(props: &Props) -> Html {
    info!("CardManager selected_user_id_update prop: {:?}", props.selected_user_id);
    info!("CardManager on_selected_user_id_update prop: {:?}", props.on_selected_user_id_update);
    match &props.active_card {
        None => html! { <div></div> }, // Return an empty div if there is no active card (otherwise none of the cards will render)
      
        Some(card_type) => match card_type {
            CardType::AddIncomeForm => {
                html! {
                    <div class="card-container">
                        <div class="card-overlay" onclick={props.on_close.clone()}></div>
                        <div class="form-wrapper">
                            <AddIncomeForm 
                                on_close={ props.on_close.clone() } 
                                selected_user_id={ props.selected_user_id }
                                user_name={ props.user_name.clone() }
                            />
                        </div>
                    </div>
                }
            }
            
            CardType::ManageDemoProfiles => {
                html! {
                    <div class="card-container">
                        <div class="card-overlay" onclick={ props.on_close.clone() }></div>
                        <div class="form-wrapper">
                            <ManageDemoProfilesCard
                                on_close={props.on_close.clone()} 
                                on_selected_user_id_update={ props.on_selected_user_id_update.clone() }
                                on_selected_user_name_update={ props.on_selected_user_name_update.clone() }
                                selected_user_id={ props.selected_user_id }
                                user_name={ props.user_name.clone() }
                                
                            />
                        </div>
                    </div>
                }
            }
        },
    }
}
