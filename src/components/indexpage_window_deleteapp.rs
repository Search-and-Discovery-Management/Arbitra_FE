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
use crate::types::var;

pub enum Msg {
    ToggleDeleteApp,
    RequestDeleteApp,
    GetDeleteAppName,
    InputDeleteApp(String),

    RequestAppData,
    GetAppData(Option<Vec<AppData>>),

    ResponseError(String),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AppData{
    pub app: String
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct WindowDeleteAppProps {
    // #[prop_or(String::from("this is value"))]
    #[prop_or(false)]
    pub display_delete_app: bool,
    pub on_toggle_deleteapp:Callback<Msg>,
}


pub struct DeleteApp {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    props: WindowDeleteAppProps,
    callback_toggle_deleteapp: Callback<Msg>,
    fetch_task: Option<FetchTask>,

    app_data: Option<Vec<AppData>>,

    app_name: String,
    request_success: bool,
}

impl Component for DeleteApp {
    type Message = Msg;
    type Properties = WindowDeleteAppProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            callback_toggle_deleteapp: props.on_toggle_deleteapp.clone(),
            props,
            fetch_task: None,

            app_data: Some(vec![]),

            app_name: String::from(""),
            request_success: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleDeleteApp => {
                self.callback_toggle_deleteapp.emit(Msg::ToggleDeleteApp);
                true
            }

            Msg::RequestAppData => {
                //FETCHING...
                let request = Request::get("https://search-discovery-api.dev-domain.site/api/index")
                    // .header("access_token", get_access_token{}.unwrap_or_default())
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<Vec<AppData>, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        // let status_number = meta.status.as_u16();
        
                        match data { 
                            Ok(dataok) => {
                                // ConsoleService::info(&format!("data response {:?}", &dataok));
                                Msg::GetAppData(Some(dataok))
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

            Msg::GetAppData(data) => {
                // ConsoleService::info(&format!("data is {:?}", data));
                self.app_data = data;
                true
            }

            Msg::InputDeleteApp(data) => {
                // ConsoleService::info(&format!("Input Data for deletion: {:?}", data));
                // let test = data.to_owned();
                self.app_name = data;
                true
            }

            Msg::RequestDeleteApp => {
                //POST FETCHING...

                let url = format!("https://search-discovery-api.dev-domain.site/api/index/{}", &self.app_name);

                let request = Request::delete(url)
                    // .header("Content-Type", "application/json")
                    // .header(Json(&villain))
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<(), anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        // let status_number = meta.status.as_u16();

                        if meta.status.is_success() {
                            Msg::GetDeleteAppName                            
                        } else {
                            match data { 
                                Ok(dataok) => {
                                    // ConsoleService::info(&format!("data response {:?}", &dataok));
                                    Msg::GetDeleteAppName
                                }
                                Err(error) => {
                                    Msg::ResponseError(error.to_string())
                                }
                            }   
                        }
                    });
        
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                
                self.fetch_task = Some(task);
                self.request_success = true;
                true
            }

            Msg::GetDeleteAppName => {
                self.link.send_message(Msg::RequestAppData);
                true
            }

            Msg::ResponseError(text) => {
                ConsoleService::info(&format!("error is {:?}", text));
                true
            }
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
			self.link.send_message(Msg::RequestAppData)
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
                <div class="window-index" id="create-index"> 

                    <div class="top-row-index-window-insert">
                        <h1>{"DELETE APPLICATION"}{""}</h1>
                        
                        <button 
                            type="button" 
                            class="window-index-closebutton"
                            onclick=self.link.callback(|_| Msg::ToggleDeleteApp)>
                                <img src="images/close.png" alt="close window" style="width: 32px"/> 
                        </button>
                    </div> 

                    <div style="margin-bottom: 15px">
                        <p style="font-weight: bold;">{ "Here are a list of your Applications:" }</p>
                        { self.view_app_data() }
                    </div>

                    <div style="margin-bottom: 20px">
                        { "Please type the app name you want to delete for confirmation." }
                        <form class="deleteapp-text-input" id="submit-deleteapp">

                        <input 
                            type="text" 
                            class="form-control" 
                            id="create-app-text" 
                            aria-describedby="emailHelp"
                            placeholder="App name to DELETE here..."
                            style="margin-top: 5px"
                            oninput = self.link.callback(|data: InputData| Msg::InputDeleteApp(data.value))
                            />
                        // <div class="window-confirm-button">
                        // </div>
                        </form>  
                    </div>
                    
                    <h6>{"Are you sure?"}
                     </h6>

                    <button disabled=true class="window-delete-warning">
                        {"THIS OPERATION CANNOT BE REVERSED OR UNDONE!"}
                    </button> 

                    <button disabled=true class="window-delete-warning">
                        {"ALL INDICES AND RECORD DATA INSIDE THE APPLICATION WILL BE DELETED!"}
                    </button> 

                    <button 
                        type="submit"
                        form="submit-deleteapp"
                        class="window-confirm-button"
                        onclick=self.link.callback(|_| Msg::RequestDeleteApp)

                        // onclick=self.link.batch_callback(|_| vec![
                        //     Msg::RequestDeleteIndex,
                        //     Msg::ToggleDeleteRecord,
                        // ])
                    >
                        { "DELETE APPLICATION" }
                    </button>
                    
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

impl DeleteApp {
    fn view_app_data(&self) -> Vec<Html> {
        self.app_data.iter().map(|card|{
                card.iter().map(|card_parse|{
                    let app_name = card_parse.app.clone();
                    html!{
                        <li>
                            // { serde_json::to_string_pretty(&card_parse.index).unwrap().trim_start().replace("\"", "")}
                            { card_parse.app.clone() }
                        </li>
                    }
                }).collect()
                
            }).collect()
    }
    fn modal_success(&self) -> Html {
        html! {
            <div class="window-overlay">
                <div class="window-index" id="create-index"> 

                    <div class="top-row-index-window-insert">
                        <h1>{"DELETE APPLICATION SUCCESSFUL"}</h1>
                    </div> 

                    <button 
                        type="submit"
                        form="submit-deletecard"
                        class="window-confirm-button"
                        onclick=self.link.callback(|_| Msg::ToggleDeleteApp)
                    >
                        { "OKAY" }
                    </button>  
                </div>
            </div>
        }
    }
}
