use yew::{
    format::{ Json, Nothing },
    prelude::*,
    services::{
        fetch::{FetchService, FetchTask, Request, Response},
        ConsoleService,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value, from_value, Map};
use crate::{types::var};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CreateApp{
    pub app_name: String
}

pub enum Msg {
    ToggleCreateApp,
    RequestCreateApplication,
    GetCreateApp(String),
    ResponseError(String),
    InputCreateApp(String),
    Ignore
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
    fetch_task: Option<FetchTask>,
    app_name: String,
    request_success: bool,
    loading: bool,

}

impl Component for AppCreate {
    type Message = Msg;
    type Properties = WindowCreateAppProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            callback_toggle_createapp: props.on_toggle_createapp.clone(),
            props,
            fetch_task: None,
            app_name: String::from(""),
            request_success: false,
            loading: false,
            
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

            Msg::InputCreateApp(data) => {
                ConsoleService::info(&format!("Input Data: {:?}", data));
                self.app_name = data;
                true
            }

            Msg::RequestCreateApplication => {
                self.loading = true;

                let create = CreateApp {
                    app_name: self.app_name.clone(),
                };

                let request = Request::post("https://test-dps-api.dev-domain.site/api/app")
                    .header("Content-Type", "application/json")
                    .body(Json(&create))
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<String, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        // let status_number = meta.status.as_u16();
                        
                        match data { 
                            Ok(dataok) => {
                                ConsoleService::info(&format!("data response {:?}", &dataok));
                                Msg:: GetCreateApp(dataok)
                            }
                            Err(error) => {
                                Msg::ResponseError(error.to_string())
                            }
                        }
                    });
        
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                
                self.fetch_task = Some(task);
                
                true
            }

            Msg::GetCreateApp(data) => {
                self.request_success = true;
                self.loading = false;
                self.app_name = data;
                true
            }

            Msg::ResponseError(text) => {
                //TEMP DEBUG
                self.request_success = true;
                self.loading = false;
                // self.app_name = data;
                true
            }

            Msg::Ignore => {
                false
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
                            placeholder="Application name here..."
                            oninput = self.link.callback(|data: InputData| Msg::InputCreateApp(data.value))/>
            
                    // <div class="window-confirm-button">
                    // </div>

                    {
                        if self.loading {
                            html!{
                                html!{
                                    <button 
                                    type="submit"
                                    form="submit-insertrecord"
                                    class="window-confirm-button"
                                    >
                                        <span class="loader">
                                            <span class="loader-inner">
                                            </span>
                                        </span>
                                    </button>
                                }
                            }
                        } else {
                            html! {
                                <button 
                                type="submit"
                                class="window-confirm-button"
                                form="submit-createapp"
                                onclick = self.link.callback(|_| Msg::RequestCreateApplication)
                                >
                                    { "CREATE APPLICATION" }
                                </button>                                
                            }
                        }
                    }
                    
                    </form>

                    
                </div>
                {
                    if self.request_success {
                        html!{
                            {self.modal_success()}
                        }
                            
                    } else {
                        html!{}
                    }
                }

            </div>
        }
    }  
}

impl AppCreate {
    fn modal_success(&self) -> Html {
        html! {
            <div class="window-overlay">
                <div class="window-index" id="create-index"> 

                    <div class="top-row-index-window-insert">
                        <h1>{"CREATE APPLICATION SUCCESSFUL"}</h1>
                    </div> 

                    <button 
                        type="submit"
                        form="submit-deletecard"
                        class="window-confirm-button"
                        onclick=self.link.callback(|_| Msg::ToggleCreateApp)
                    >
                        { "OKAY" }
                    </button>  
                </div>
            </div>
        }
    }
}
