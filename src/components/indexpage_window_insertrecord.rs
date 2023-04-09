use serde::Serialize;
use yew::{prelude::*, services::ConsoleService};
use yew::services::fetch::Request;
use yew::services::fetch::{Response, FetchService, FetchTask};
use yew::format::{Json, self};

pub enum Msg {
    ToggleInsertRecord,
    ValidateInputJson(String),
    RequestCreateRecordsData,
    Ignore,
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct WindowInsertRecordProps {
    // #[prop_or(String::from("this is value"))]
    #[prop_or(false)]
    pub display_insert_record: bool,
    pub on_toggle_insertrecord:Callback<Msg>,

    #[prop_or_default]
    pub app_id: String,
    #[prop_or_default]
    pub card_index: String, //index_name

}

pub struct InsertRecord {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    props: WindowInsertRecordProps,
    callback_toggle_insertrecord: Callback<Msg>,
    value: String,
    json_is_valid: bool,
    fetch_task: Option<FetchTask>,
    request_success: bool,

    app_id: String,
    app_name: String
}

impl Component for InsertRecord {
    type Message = Msg;
    type Properties = WindowInsertRecordProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            callback_toggle_insertrecord: props.on_toggle_insertrecord.clone(),
            app_id: props.app_id.clone(),
            app_name: props.card_index.clone(),
            props,
            value: "".to_string(),
            json_is_valid: false,
            fetch_task: None,
            request_success: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleInsertRecord => {
                self.callback_toggle_insertrecord.emit(Msg::ToggleInsertRecord);
                true
            }

            Msg::ValidateInputJson (data) => {
                self.value = data;
                self.json_is_valid = match serde_json::from_str::<serde_json::Value>(&self.value) {
                    Ok(_) => true,
                    Err(_) => false,
                };
                // ConsoleService::info(&format!("DEBUG : value:{:?}", &self.value));
                // ConsoleService::info(&format!("DEBUG : json_is_valid:{:?}", self.json_is_valid));
                true
            }

            Msg::RequestCreateRecordsData => {
                let mut records = serde_json::json!({});
                match serde_json::from_str::<serde_json::Value>(&self.value) {
                    Ok(create) => records = create,
                    Err(Error) => ConsoleService::info(&format!("Data Input = {}", &Error)),
                };
                // ConsoleService::info(&format!("Data Input = {}", &records));

                let url = format!("https://test-dps-api.dev-domain.site/api/document/{}/{}", &self.app_id, &self.app_name);
                let request = Request::post(url)
                    .header("Content-Type", "application/json")
                    .body(Json(&records))
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<String, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        match data { 
                            Ok(dataok) => {
                                // ConsoleService::info(&format!("data response {:?}", &dataok));
                                Msg::Ignore
                            }
                            Err(error) => {
                                Msg::Ignore
                            }
                        }
                    });
                    // self.callback_toggle_insertrecord.emit(Msg::ToggleInsertRecord);
                    let task = FetchService::fetch(request, callback).expect("failed to start request");
                
                    
                    self.fetch_task = Some(task);
                    ConsoleService::info(&format!("REQUEST JALAN"));
                    self.request_success = true;
                true
            }

            Msg::Ignore => {
                false
            }
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            ConsoleService::info(&format!("data MODAL card_index / Index name {:?}", self.props.card_index));
            ConsoleService::info(&format!("data MODAL app_id {:?}", self.props.app_id));
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
                        <h1>{"INSERT NEW RECORD"}</h1>
                        
                        <button 
                            type="button" 
                            class="window-index-closebutton"
                            onclick=self.link.callback(|_| Msg::ToggleInsertRecord)>
                                <img src="images/close.png" alt="close window" style="width: 32px"/> 
                        </button>
                    </div> 

                    <h6>{"Add record with the JSON Format, containing a single object or an array of
                     objects, for example : "}
                     </h6>

// FORM INPUT TEXT UNTUK EXAMPLE JSON 
                    <div>
                        <form class="record-text-input" id="example-no-submit">
                            <textarea 
                                readonly=true
                                type="text" 
                                class="insert-record" 
                                style="font-size:12px;font-weight: bold; line-height: 1.4;"
                                
                                >{"
{
    \"product\": \"Ipon\",
    \"price\": \"9999\",
    \"Quantity\": \"9999\",
    \"Product Etalase\": \"Electronic\"
}            
"}                     </textarea>
                        </form>   
                    </div> 
// FORM INPUT EXAMPLE END

                    <h6>{"Add your records here"}
                    </h6>


// FORM INPUT TEXT UNTUK EXAMPLE JSON 
                    <div class="window-submit-form">
                        <form class="record-text-input" id="submit-insertrecord">
                            <textarea 
                                type="text" 
                                class="insert-record" 
                                style="font-size:12px;font-weight: bold; line-height: 1.4;"

                                oninput = self.link.callback(|data: InputData| Msg::ValidateInputJson(data.value))
                                >
                            {""}  
                            </textarea>
                //BUTTON SUBMIT (HARUS DI FORM YANG SAMA)
                        </form>   
                    </div> 
// FORM INPUT EXAMPLE END
                {
                    if self.json_is_valid {
                        html!{
                            <button 
                                type="submit"
                                form="submit-insertrecord"
                                class="window-confirm-button"
                                onclick = self.link.callback(|_| Msg::RequestCreateRecordsData)

                                // onclick=self.link.batch_callback(|_| vec![
                                //     Msg::ToggleInsertRecord,
                                //     Msg::RequestCreateRecordsData,
                                // ])
                            >
                                { "INSERT NEW RECORD" }
                            </button>
                        }
                    } else {
                        html! {
                            <button disabled=true class="window-confirm-button">
                                {"FORM INPUT MUST BE IN JSON FORMAT!"}
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
impl InsertRecord {
    fn modal_success(&self) -> Html {
        html! {
            <div class="window-overlay">
                <div class="window-index" id="create-index"> 

                    <div class="top-row-index-window-insert">
                        <h1>{"INSERT RECORD SUCCESSFUL"}</h1>
                    </div> 

                    <button 
                        type="submit"
                        form="submit-deletecard"
                        class="window-confirm-button"
                        onclick=self.link.callback(|_| Msg::ToggleInsertRecord)
                    >
                        { "OKAY" }
                    </button>  
                </div>
            </div>
        }
    }
}