use yew::{prelude::*, Context};
use yew::events::InputEvent;
use shared::user::User;
use wasm_bindgen::{ JsCast };
// use wasm_bindgen_futures::spawn_local;
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
    pub on_selected_user_name_update: Callback<Option<String>>,
    pub on_selected_user_id_update: Callback<Option<u32>>,
    pub selected_user_id: Option<u32>,
    pub user_name: Option<String>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            on_close: Callback::noop(),
            on_selected_user_name_update: Callback::noop(),
            on_selected_user_id_update: Callback::noop(),
            selected_user_id: None,
            user_name: Some("Unknown".to_owned())
        }
    }
}

pub enum Msg {
    FetchUsers,
    UsersFetched(Result<Vec<User>, ApiError>),
    UserSelected(Option<User>),
    UpdateSelectedUserId(Option<u32>),
    UpdateSelectedUserName(Option<String>),
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

            Msg::UserSelected(user) => {
                info!("UserSelected: {:?}", user);
                self.selected_user = user.clone();
                if let Some(user) = user {
                    ctx.link().send_message(Msg::UpdateSelectedUserId(Some(user.id)));
                    ctx.link().send_message(Msg::UpdateSelectedUserName(Some(user.username.clone()))); 
                } else {
                    ctx.link().send_message(Msg::UpdateSelectedUserId(None));
                    ctx.link().send_message(Msg::UpdateSelectedUserName(None)); 
                }
                false
            }

            Msg::UpdateSelectedUserName(user_name) => {
                ctx.props().on_selected_user_name_update.emit(user_name);
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
                true
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


    fn changed(&mut self, ctx: &Context<Self>, props: &Self::Properties) -> bool {
        if props != &*ctx.props() {
            true
        } else {
            false
        }
    }



    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let users_list = self.users.iter().map(|user| {
            let user_id = user.id;
            let user_name = user.username.clone();
            let user_clone = user.clone();
            let user_selected_callback = link.callback(move |_| {
                Msg::UserSelected(Some(user_clone.clone()))
            });

            html! {
                <div class="user-list-item" onclick={user_selected_callback}>
                    {user_name}
                </div>
            }
        }).collect::<Vec<Html>>();




        let unknown_user = String::from("Unknown");
        let user_name = ctx.props().user_name.as_ref().unwrap_or(&unknown_user);

        html! {
            <div class= {classes!("card-main", "manage-profiles-card")}>
                <h1>{"Manage Demo Profiles"}</h1>
                <div class = {classes!("manage-profiles-sections")}>
                    <div class = {classes!("select-profile-section")}>
                        <div class = {classes!("display-current-user-section")}>
                            <h2>{"Current user"}</h2>
                            <h3>{ format!("{}", user_name) }</h3>


                        </div>
                        <div class = {classes!("card-vertical-spacer")}>
                        </div>
                        <div class = {classes!("select-new-current-user-section")}>
                            <h2>{"Select a user"}</h2>
                            <div class="user-list">
                                { for users_list }
                            </div>

                        </div>
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
                        <button onclick={ctx.link().callback(|_| Msg::CreateUser)}>
                            { "Create" }
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}