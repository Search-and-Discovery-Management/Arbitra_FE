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
    ToggleDeleteRecord,
    RequestDeleteIndex,
    GetDeleteIndexName,
    InputDeleteIndex(String),

    RequestIndexData,
    GetIndexData(Option<Vec<IndexData>>),

    ResponseError(String),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct IndexData{
    pub index: String
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct WindowDeleteRecordProps {
    // #[prop_or(String::from("this is value"))]
    #[prop_or(false)]
    pub display_delete_record: bool,
    pub on_toggle_deleterecord:Callback<Msg>,

    #[prop_or_default]
    pub app_id: String,
}


pub struct DeleteRecord {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    props: WindowDeleteRecordProps,
    callback_toggle_deleterecord: Callback<Msg>,
    fetch_task: Option<FetchTask>,

    index_data: Option<Vec<IndexData>>,

    index_name: String,
    request_success: bool,
    app_id: String,

    loading: bool,
}

impl Component for DeleteRecord {
    type Message = Msg;
    type Properties = WindowDeleteRecordProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            callback_toggle_deleterecord: props.on_toggle_deleterecord.clone(),
            app_id: props.app_id.clone(),
            props,
            fetch_task: None,

            index_data: Some(vec![]),

            index_name: String::from(""),
            request_success: false,

            loading: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleDeleteRecord => {
                self.callback_toggle_deleterecord.emit(Msg::ToggleDeleteRecord);
                true
            }

            Msg::RequestIndexData => {
                self.loading = true;
                //FETCHING...
                let request = Request::get(format!("https://test-dps-api.dev-domain.site/api/index/{}", &self.app_id))
                    // .header("access_token", get_access_token{}.unwrap_or_default())
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<Vec<IndexData>, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        // let status_number = meta.status.as_u16();
        
                        match data { 
                            Ok(dataok) => {
                                // ConsoleService::info(&format!("data response {:?}", &dataok));
                                Msg:: GetIndexData(Some(dataok))
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

            Msg::GetIndexData(data) => {
                // ConsoleService::info(&format!("data is {:?}", data));
                self.loading = false;
                self.index_data = data;
                true
            }

            Msg::InputDeleteIndex(data) => {
                // ConsoleService::info(&format!("Input Data for deletion: {:?}", data));
                // let test = data.to_owned();
                self.index_name = data;
                true
            }

            Msg::RequestDeleteIndex => {
                self.loading = true;
                let url = format!("https://test-dps-api.dev-domain.site/api/index/{}/{}", &self.app_id, &self.index_name);

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
                            Msg::GetDeleteIndexName                            
                        } else {
                            match data { 
                                Ok(dataok) => {
                                    // ConsoleService::info(&format!("data response {:?}", &dataok));
                                    Msg:: GetDeleteIndexName
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

            Msg::GetDeleteIndexName => {
                self.loading = false;
                self.request_success = true;
                self.link.send_message(Msg::RequestIndexData);
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
			self.link.send_message(Msg::RequestIndexData);
            // ConsoleService::info(&format!("data MODAL app_id {:?}", self.props.app_id));
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
                        //Diganti Lagi ke DELETE RECORD #
                        <h1>{"DELETE INDEX"}{""}</h1>
                        
                        <button 
                            type="button" 
                            class="window-index-closebutton"
                            onclick=self.link.callback(|_| Msg::ToggleDeleteRecord)>
                                <img src="images/close.png" alt="close window" style="width: 32px"/> 
                        </button>
                    </div> 

                    <div style="margin-bottom: 15px">
                        <p style="font-weight: bold;">{ "Here are a list of your indexes:" }</p>
                        { self.view_index_data() }
                    </div>

                    <div style="margin-bottom: 20px">
                        { "Please type the index name you want to delete for confirmation." }
                        <form class="createindex-text-input" id="submit-createindex">

                        <input 
                            type="text" 
                            class="form-control" 
                            id="create-index-text" 
                            aria-describedby="emailHelp"
                            placeholder="Index name to DELETE here..."
                            style="margin-top: 5px"
                            oninput = self.link.callback(|data: InputData| Msg::InputDeleteIndex(data.value))
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
                                form="submit-deleterecord"
                                class="window-confirm-button"
                                onclick=self.link.callback(|_| Msg::RequestDeleteIndex)
        
                                // onclick=self.link.batch_callback(|_| vec![
                                //     Msg::RequestDeleteIndex,
                                //     Msg::ToggleDeleteRecord,
                                // ])
                                >
                                    { "DELETE INDEX" }
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

impl DeleteRecord {
    fn view_index_data(&self) -> Vec<Html> {
        self.index_data.iter().map(|card|{
            card.iter().map(|card_parse|{
                let index_name = card_parse.index.clone().split('.').next_back().unwrap().to_string();
                html!{
                    <li>
                        // { serde_json::to_string_pretty(&card_parse.index).unwrap().trim_start().replace("\"", "")}
                        { card_parse.index.clone().split('.').next_back().unwrap() }
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
                        <h1>{"DELETE INDEX SUCCESSFUL"}</h1>
                    </div> 

                    <button 
                        type="submit"
                        form="submit-deletecard"
                        class="window-confirm-button"
                        onclick=self.link.callback(|_| Msg::ToggleDeleteRecord)
                    >
                        { "OKAY" }
                    </button>  
                </div>
            </div>
        }
    }
}
