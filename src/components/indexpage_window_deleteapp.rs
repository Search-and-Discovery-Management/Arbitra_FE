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
    pub _id: String,
    pub _source: Value
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

    app_id: String,

    app_name: String,
    request_success: bool,

    loading: bool,
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

            app_id: String::from(""),
            app_name: String::from(""),
            request_success: false,

            loading: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleDeleteApp => {
                self.callback_toggle_deleteapp.emit(Msg::ToggleDeleteApp);
                true
            }

            Msg::RequestAppData => {
                self.loading = true;
                let request = Request::get("https://test-dps-api.dev-domain.site/api/apps")
                    // .header("access_token", get_access_token{}.unwrap_or_default())
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<Vec<AppData>, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
        
                        match data { 
                            Ok(dataok) => {
                                // ConsoleService::info(&format!("data response {:?}", &dataok));
                                Msg:: GetAppData(Some(dataok))
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
                self.loading = false;
                self.app_data = data;
                true
            }

            Msg::InputDeleteApp(data) => {
                // ConsoleService::info(&format!("Input Data for deletion: {:?}", data));
                // let test = data.to_owned();
                self.app_id = data;
                true
            }

            Msg::RequestDeleteApp => {
                //POST FETCHING...
                self.loading = true;

                let url = format!("https://test-dps-api.dev-domain.site/api/app/{}", &self.app_id);

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
                
                true
            }

            Msg::GetDeleteAppName => {
                self.loading = false;
                self.request_success = true;
                
                self.link.send_message(Msg::RequestAppData);
                true
            }

            Msg::ResponseError(text) => {
                self.loading = false;
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
                        { "Please type the App ID you want to delete for confirmation." }
                        <form class="deleteapp-text-input" id="submit-deleteapp">

                        <input 
                            type="text" 
                            class="form-control" 
                            id="create-app-text" 
                            aria-describedby="emailHelp"
                            placeholder="App ID to DELETE here..."
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

                    {
                        if self.loading {
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
                        } else {
                            html!{
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
                            }
                        }
                    }
                        
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
                    let app_id = card_parse._id.clone();
                    let app_name = card_parse._source.clone();
                    html!(
                        <div>
                            <li>
                                { app_name.get("name").unwrap().as_str().unwrap() }
                            </li>
                            <ul>
                                <b>{ "App ID: " }</b>{ app_id }
                            </ul>
                        </div>
                    )
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
