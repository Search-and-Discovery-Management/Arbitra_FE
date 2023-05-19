use serde::Serialize;
use serde_json::json;
use wasm_bindgen::JsValue;
use yew::{prelude::*, services::ConsoleService};
use yew::services::fetch::Request;
use yew::services::fetch::{Response, FetchService, FetchTask};
use yew::format::{Json, self};
use yew::events::ChangeData;
use yew::web_sys::File;
use web_sys::FormData;

pub enum Msg {
    ToggleUploadRecord,
    RequestCreateRecordsData,
    ValidateUploadFile,
    Ignore,
    ErrorIgnore,

    SelectFile(Option<File>),
    ClearFile,
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct WindowUploadRecordProps {
    // #[prop_or(String::from("this is value"))]
    #[prop_or(false)]
    pub display_upload_record: bool,
    pub on_toggle_uploadrecord:Callback<Msg>,

    #[prop_or_default]
    pub app_id: String,
    #[prop_or_default]
    pub card_index: String, //index_name

}

pub struct UploadRecord {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    props: WindowUploadRecordProps,
    callback_toggle_uploadrecord: Callback<Msg>,
    value: String,
    file_is_valid: bool,
    fetch_task: Option<FetchTask>,

    //UNTUK MUNCULIN MODAL CONFIRMATION
    request_success: bool,
    request_fail: bool,

    //UNTUK DETECT DIA MASIH LOADING ATAU NGK DI MODAL
    loading: bool,

    file: Option<File>,
}

impl Component for UploadRecord {
    type Message = Msg;
    type Properties = WindowUploadRecordProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            callback_toggle_uploadrecord: props.on_toggle_uploadrecord.clone(),
            props,
            value: "".to_string(),
            file_is_valid: false,
            fetch_task: None,

            request_success: false,
            request_fail: false,

            loading: false,

            file: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleUploadRecord => {
                self.callback_toggle_uploadrecord.emit(Msg::ToggleUploadRecord);
                true
            }

            Msg::ValidateUploadFile => {
                self.file_is_valid = true;
                true
            }

            Msg::SelectFile(file) => {
                self.file = file;
                true
            }

            Msg::ClearFile => {
                self.file = None;
                true
            }

            Msg::RequestCreateRecordsData => {
                //LOADING STATUS KE STATE
                self.loading = true;
                ConsoleService::info(&format!("DEBUG loading status : {:?}", &self.loading));

                let form_data = FormData::new().unwrap();
                form_data.append_with_blob("file", self.file.as_ref().unwrap()).unwrap();

                let mut opts = web_sys::RequestInit::new();
                opts.method("POST");
                opts.body(Some(&JsValue::from(form_data)));

                let request = web_sys::Request::new_with_str_and_init(&format!("https://test-dps-api.dev-domain.site/api/upload/document/{}/{}", &self.props.app_id, &self.props.card_index).to_string() , &opts).unwrap();
                let window = web_sys::window().unwrap();
                let _ = window.fetch_with_request(&request);

                // let url = format!("https://test-dps-api.dev-domain.site/api/upload/document/{}/{}", &self.props.app_id, &self.props.card_index);
                // let request = Request::post(url)
                //     .header("Content-Type", "multipart/form-data")
                //     .body(Ok(form_data.to_string().into()))
                //     .expect("Could not build request.");
                // let callback = 
                //     self.link.callback(|response: Response<Json<Result<String, anyhow::Error>>>| {
                //         let (meta, Json(data)) = response.into_parts();
                //         match data { 
                //             Ok(dataok) => {
                //                 // ConsoleService::info(&format!("data response {:?}", &dataok));
                //                 Msg::Ignore
                                
                //             }
                //             Err(error) => {
                //                 Msg::ErrorIgnore
                //             }
                //         }
                //     });
                //     // self.callback_toggle_insertrecord.emit(Msg::ToggleInsertRecord);
                //     let task = FetchService::fetch(request, callback).expect("failed to start request");
                
                    
                //     self.fetch_task = Some(task);
                //     ConsoleService::info(&format!("Getting Data.."));
                    
                true
            }

            Msg::Ignore => {
                self.loading = false;
                self.request_success = true;
                ConsoleService::info(&format!("DEBUG loading status : {:?}", &self.loading));
                // ConsoleService::info(&format!("DEBUG request success : {:?}", &self.request_success));
                true
            }

            Msg::ErrorIgnore => {
                self.loading = false;
                self.request_success = true;
                ConsoleService::info(&format!("DEBUG loading status : {:?}", &self.loading));
                // ConsoleService::info(&format!("DEBUG request fail : {:?}", &self.request_fail));
                true
            }
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            // ConsoleService::info(&format!("data MODAL card_index / Index name {:?}", self.props.card_index));
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
                        <h1>{"UPLOAD RECORDS FILE TO INDEX "}{self.props.card_index.clone().replace("\"", "")}</h1>
                        
                        <button 
                            type="button" 
                            class="window-index-closebutton"
                            onclick=self.link.callback(|_| Msg::ToggleUploadRecord)>
                                <img src="images/close.png" alt="close window" style="width: 32px"/> 
                        </button>
                    </div> 

                    <div class="container-middle-width">
                        <label class="upload-label" for="upload-input">{"Upload your JSON file here:"}</label>

                        <div class="upload-file-container"> 
                            <input 
                                type="file" 
                                id="upload-dropzone" 
                                name="upload-input" 
                                accept=".json"
                                onchange=self.link.callback(|event: ChangeData| {
                                    if let ChangeData::Files(files) = event {
                                        if let Some(file) = files.get(0) {
                                            Msg::SelectFile(Some(file))
                                        } else {
                                            Msg::Ignore
                                        }
                                    } else {
                                        Msg::Ignore
                                    }
                                })
                                />
                        </div>

                        <h6>{"Uploading a file will add records to this index; it will not erase previous records."}</h6>
                    </div>
                    

                    <h6>{"Upload file with the JSON Format, containing a single object or an array of
                     objects"}
                     </h6>

// FORM INPUT TEXT UNTUK EXAMPLE JSON 
                    <div>
                        <form class="record-text-input" id="example-no-submit">
                            <textarea 
                                readonly=true
                                type="text" 
                                class="insert-record" 
                                style="font-size:12px;font-weight: bold; line-height: 1.4;"
                                
                                >{"[{
    \"key1\": \"value1\",
    \"key2\": \"value2\",
    \"key3\": \"value3\",
    \"key4\": \"value4\",
    \"key5\": \"value5\"
}]
                        "}  </textarea>
                        </form>   
                    </div> 
                   
// FORM INPUT EXAMPLE END

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
                    { "UPLOAD RECORDS FILE" }
                    </button>

                // {
                //     if self.file_is_valid {
                //         if self.loading {
                //             html!{
                //                 <button 
                //                 type="submit"
                //                 form="submit-insertrecord"
                //                 class="window-confirm-button"
                //                 // onclick = self.link.callback(|_| Msg::RequestCreateRecordsData)
                //                 >
                //                     <span class="loader">
                //                         <span class="loader-inner">
                //                         </span>
                //                     </span>
                //                 </button>
                //             }
                //         } else {
                //             html!{
                //                 <button 
                //                     type="submit"
                //                     form="submit-insertrecord"
                //                     class="window-confirm-button"
                //                     onclick = self.link.callback(|_| Msg::RequestCreateRecordsData)
    
                //                     // onclick=self.link.batch_callback(|_| vec![
                //                     //     Msg::ToggleInsertRecord,
                //                     //     Msg::RequestCreateRecordsData,
                //                     // ])
                //                 >
                //                     { "UPLOAD RECORDS FILE" }
                //                 </button>
                //             }
                //         }
                        
                //     } else {
                //         html! {
                //             <button disabled=true class="window-confirm-button">
                //                 {"PLEASE UPLOAD A FILE"}
                //             </button> 
                //         }
                //     }

                // }
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

                {
                    if self.request_fail {
                        html!{
                            {self.modal_fail()}
                        }
                    } else {
                        html!{}
                    }
                }

            </div>
        }
    }
}
impl UploadRecord {
    fn modal_success(&self) -> Html {
        html! {
            <div class="window-overlay">
                <div class="window-index" id="create-index"> 

                    <div class="top-row-index-window-insert">
                        <h1>{"UPLOAD RECORD SUCCESSFUL"}</h1>
                    </div> 

                    <button 
                        type="submit"
                        form="submit-deletecard"
                        class="window-confirm-button"
                        onclick=self.link.callback(|_| Msg::ToggleUploadRecord)
                    >
                        { "OKAY" }
                    </button>  
                </div>
            </div>
        }
    }

    fn modal_fail(&self) -> Html {
        html! {
            <div class="window-overlay">
                <div class="window-index" id="create-index"> 

                    <button disabled=true class="window-delete-warning-main" >
                        {"UPLOAD RECORD FAILED!"}
                    </button> 

                    <button 
                        type="submit"
                        form="submit-deletecard"
                        class="window-confirm-button"
                        onclick=self.link.callback(|_| Msg::ToggleUploadRecord)
                    >
                        { "OKAY" }
                    </button>  
                </div>
            </div>
        }
    }


}