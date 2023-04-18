


use yew::{prelude::*, Context};



pub struct ManageDemoProfilesCard {
   
}


#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub on_close: Callback<MouseEvent>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            on_close: Callback::noop(),
        }
    }
}


impl Component for ManageDemoProfilesCard {

    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, _ctx: &Context<Self>, _props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class= {classes!("manage-demo-profiles-card")}>
                <h1>{"Manage Demo Profiles"}</h1>
                <p>{"This is a demo card"}</p>
            </div>
        }
    }
}