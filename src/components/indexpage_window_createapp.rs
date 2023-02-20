use yew::prelude::*;

pub enum Msg {
    ToggleCreateApp,
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct WindowCreateAppProps {
    // #[prop_or(String::from("this is value"))]
    #[prop_or(false)]
    pub display_create_app: bool,
    pub on_toggle_createapp:Callback<Msg>,
}


pub struct AppCreate {

    link: ComponentLink<Self>,
    props: WindowCreateAppProps,
    callback_toggle_createapp: Callback<Msg>,

}

impl Component for AppCreate {
    type Message = Msg;
    type Properties = WindowCreateAppProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            callback_toggle_createapp: props.on_toggle_createapp.clone(),
            props,
            
            // {
            //     display_create_app: props.display_create_app,
            //     on_toggle: props.on_toggle,
            // }

        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg{
            Msg::ToggleCreateApp => {
                self.callback_toggle_createapp.emit(Msg::ToggleCreateApp);
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="window-overlay">
                <div class="window-index" id="create-app"> 

                    <div class="top-row-index-window">
                        <h1>{"CREATE NEW APPLICATION"}</h1>
                        
                        <button 
                            type="button" 
                            class="window-index-closebutton"
                            onclick=self.link.callback(|_| Msg::ToggleCreateApp)>
                                <img src="images/close.png" alt="close window" style="width: 32px"/> 
                        </button>
                    </div> 

                    <h5>{"INSERT APPLICATION NAME"}</h5>

                    <form class="createindex-text-input" id="submit-createapp">
                        <input 
                            type="text" 
                            class="form-control" 
                            id="create-app-text" 
                            aria-describedby="emailHelp"
                            placeholder="Application name here..."/>
            
                    // <div class="window-confirm-button">
                    // </div>

                    <button 
                        type="submit"
                        class="window-confirm-button"
                        form="submit-createapp"
                    >
                    { "CREATE APPLICATION" }
                    </button>
                    </form>

                    
                </div>

            </div>
        }
    }  
}