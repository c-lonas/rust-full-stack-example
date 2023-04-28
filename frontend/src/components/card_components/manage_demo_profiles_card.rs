use yew::{prelude::*, Context};
use yew::events::InputEvent;
use shared::user::User;
use wasm_bindgen::{ JsCast };
use wasm_bindgen_futures::spawn_local;
use reqwest::Client;
use log::info;
use crate::api_error::ApiError;
use shared::user::UserCreate;
use web_sys::{Event};


pub struct ManageDemoProfilesCard {
    users: Vec<User>,
    selected_user: Option<User>,
    new_user_name: String,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub on_close: Callback<MouseEvent>,
    pub on_selected_user_id_update: Callback<Option<u32>>,
    pub selected_user_id: Option<u32>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            on_close: Callback::noop(),
            on_selected_user_id_update: Callback::noop(),
            selected_user_id: None,
        }
    }
}

pub enum Msg {
    FetchUsers,
    UsersFetched(Result<Vec<User>, ApiError>),
    UserSelected(Option<User>),
    UpdateSelectedUserId(Option<u32>),
    UpdateNewUserName(String),
    CreateUser,
    UserCreated(Result<User, ApiError>),
    NoOp,
}

async fn fetch_users() -> Result<Vec<User>, ApiError> {
    let client = Client::new();
    let response = client.get("http://localhost:8000/api/users")
        .send()
        .await?;

    if response.status().is_success() {
        let users = response.json::<Vec<User>>().await?;
        Ok(users)
    } else {
        Err(ApiError::HttpStatus(response.status()))
    }
}

async fn create_user(username: String) -> Result<User, ApiError> {
    let client = Client::new();
    let user_create = UserCreate { username}; 
    let response = client.post("http://localhost:8000/api/users")
        .json(&user_create) 
        .send()
        .await?;

    if response.status().is_success() {
        let created_user = response.json::<User>().await?;
        Ok(created_user)
    } else {
        Err(ApiError::HttpStatus(response.status()))
    }
}


impl Component for ManageDemoProfilesCard {
    type Message = Msg;
    type Properties = Props;


    fn create(ctx: &Context<Self>) -> Self {
        let component = Self {
            users: Vec::new(),
            selected_user: None,
            new_user_name: String::new(),
        };


        ctx.link().send_message(Msg::FetchUsers);


        ctx.link().send_message(Msg::NoOp);
        component
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchUsers => {
                // info!("Fetching users...");
                ctx.link().send_future(async {
                    Msg::UsersFetched(fetch_users().await)
                });
                ctx.link().send_message(Msg::NoOp);
                false
            }
            Msg::UsersFetched(Ok(users)) => {
                // info!("Users fetched: {:?}", users);
                self.users = users;
                true
            }
            Msg::UsersFetched(Err(err)) => {
                info!("Error fetching users: {:?}", err);
                false
            }
            // Msg::UserSelected(user) => {
            //     info!("UserSelected: {:?}", user);
            //     self.selected_user = user.clone();
            //     if let Some(user) = user {
            //         ctx.link().send_message(Msg::UpdateSelectedUserId(Some(user.id)));
            //     } else {
            //         ctx.link().send_message(Msg::UpdateSelectedUserId(None));
            //     }
            //     false
            // }

            Msg::UserSelected(user) => {
                info!("UserSelected: {:?}", user);
                self.selected_user = user.clone();
                if let Some(user) = user {
                    ctx.link().send_message(Msg::UpdateSelectedUserId(Some(user.id)));
                } else {
                    ctx.link().send_message(Msg::UpdateSelectedUserId(None));
                }
                false
            }
            Msg::UpdateSelectedUserId(user_id) => {
                ctx.props().on_selected_user_id_update.emit(user_id);
                false
            }



            Msg::UpdateNewUserName(name) => {
                self.new_user_name = name;
                true
            }
            Msg::CreateUser => {
                if self.new_user_name.is_empty() {
                    log::warn!("Invalid input: User name is empty");
                    return false;
                }
                let username = self.new_user_name.clone();
                ctx.link().send_future(async move {
                    Msg::UserCreated(create_user(username).await)
                });
                ctx.link().send_message(Msg::NoOp);
                false
            }
            Msg::UserCreated(Ok(user)) => {
                self.users.push(user);
                self.new_user_name.clear();
                true
            }
            Msg::UserCreated(Err(err)) => {
                log::error!("Error creating user: {:?}", err);
                false
            }
            Msg::NoOp => false,
        }
    }


    fn changed(&mut self, _ctx: &Context<Self>, _props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // info!("Rendering view with users: {:?}", self.users);
        let users_dropdown = self.users.iter().map(|user| {
            let user_id = user.id;
            let user_name = user.username.clone();
            html! {
                <option value={user_id.to_string()}>{user_name}</option>
            }
        });

        html! {
            <div class= {classes!("card-main", "manage-profiles-card")}>
                <h1>{"Manage Demo Profiles"}</h1>
                <div class = {classes!("manage-profiles-sections")}>
                    <div class = {classes!("select-profile-section")}>
                        <h2>{"Select a user"}</h2>
                        <select
                            value={self.selected_user.as_ref().map(|user| user.id.to_string()).unwrap_or_default()}

                            onchange={ctx.link().callback({
                                let users = self.users.clone();
                                move |event: Event| {
                                    if let Some(target) = event.target() {
                                        if let Ok(select_element) = target.dyn_into::<web_sys::HtmlSelectElement>() {
                                            let value = select_element.value().parse::<u32>().ok();
                                            let selected_user = value.and_then(|id| {
                                                users.iter().find(|user| user.id == id).cloned()
                                            });
                                            info!("Setting selected_user to: {:?}", selected_user);
                                            Msg::UserSelected(selected_user)
                                        } else {
                                            Msg::NoOp
                                        }
                                    } else {
                                        Msg::NoOp
                                    }
                                }
                            })}

                        >
                            <option value="" disabled=true>{"Select a user"}</option>
                            { for users_dropdown }
                        </select>
                    </div>
                    <div class= {classes!("vertical-divider")}></div>
                    <div class= {classes!("create-new-user-section")}>
                        <h2>{"Create new user"}</h2>
                        <input
                            placeholder="Name"
                            value={self.new_user_name.clone()}
                            oninput={ctx.link().callback(|event: InputEvent| {
                                if let Some(target) = event.target() {
                                    if let Ok(input_element) = target.dyn_into::<web_sys::HtmlInputElement>() {
                                        Msg::UpdateNewUserName(input_element.value())
                                    } else {
                                        Msg::NoOp
                                    }
                                } else {
                                    Msg::NoOp
                                }
                            })}
                        />
                        <button
                            onclick={ctx.link().callback(|_| Msg::CreateUser)}
                        >
                            { "Create" }
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}