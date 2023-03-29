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

pub enum Msg {
    ToggleCreateIndex,

    RequestCreateIndex,
    GetCreateIndex(String),
    InputCreateIndex(String),

    Ignore,

    ResponseError(String),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CreateIndex{
    pub index: String
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct WindowCreateIndexProps {
    // #[prop_or(String::from("this is value"))]
    #[prop_or(false)]
    pub display_create_index: bool,
    pub on_toggle_createindex:Callback<Msg>,
}
pub struct IndexCreate {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    props: WindowCreateIndexProps,
    callback_toggle_createindex: Callback<Msg>,
    index: String,
    fetch_task: Option<FetchTask>,
}

impl Component for IndexCreate {
    type Message = Msg;
    type Properties = WindowCreateIndexProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link: link.clone(),
            callback_toggle_createindex: props.on_toggle_createindex.clone(),
            props,
            index: String::from(""),
            fetch_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg{
            Msg::ToggleCreateIndex => {
                self.callback_toggle_createindex.emit(Msg::ToggleCreateIndex);
                true
            }

            Msg::InputCreateIndex(data) => {
                // ConsoleService::info(&format!("Input Data: {:?}", data));
                // let test = data.to_owned();
                self.index = data;
                true
            }

            Msg::RequestCreateIndex => {
                //POST FETCHING...

                let create = CreateIndex {
                    index: self.index.clone(),
                };

                let request = Request::post("https://search-discovery-api.dev-domain.site/api/index")
                    .header("Content-Type", "application/json")
                    .body(Json(&create))
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<String, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        // let status_number = meta.status.as_u16();
                        
                        match data { 
                            Ok(dataok) => {
                                // ConsoleService::info(&format!("data response {:?}", &dataok));
                                Msg:: GetCreateIndex(dataok)
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

            Msg::GetCreateIndex(data) => {
                // ConsoleService::info(&format!("Index name is {:?}", data));
                self.index = data;
                true
            }

            Msg::ResponseError(text) => {
                // ConsoleService::info(&format!("error is {:?}", text));
                true
            }

            Msg::Ignore => {
                false
            }
        }
    }

    // fn rendered(&mut self, first_render: bool) {
    //     if first_render {
	// 		self.link.send_message(Msg::RequestIndexData);
    //     }
    // }

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

                    <div class="top-row-index-window">
                        <h1>{"CREATE INDEX"}</h1>
                        
                        <button 
                            type="button" 
                            class="window-index-closebutton"
                            onclick=self.link.callback(|_| Msg::ToggleCreateIndex)>
                                <img src="images/close.png" alt="close window" style="width: 32px"/> 
                        </button>
                    </div> 

                    <h5>{"INSERT INDEX NAME"}</h5>

                    
                    <form class="createindex-text-input" id="submit-createindex">

                        <input 
                            type="text" 
                            class="form-control" 
                            id="create-index-text" 
                            aria-describedby="emailHelp"
                            placeholder="Index name here..."
                            oninput = self.link.callback(|data: InputData| Msg::InputCreateIndex(data.value))
                            />
                        // <div class="window-confirm-button">
                    // </div>
                    </form>  
                    
                    <button 
                        type="submit"
                        form="submit-createindex"
                        class="window-confirm-button"
                        onclick=self.link.callback(|_| Msg::RequestCreateIndex)
                    >
                            { "CREATE INDEX" }
                    </button>

                    
                    
                </div>

            </div>
        }
    }
}