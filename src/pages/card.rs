use yew::{
    format::{ Json, Nothing },
    prelude::*,
    services::{
        fetch::{FetchService, FetchTask, Request, Response},
        ConsoleService,
    },
};

use serde_json::{Value};

pub enum Msg {
    RequestData,
    GetCardData(Option<Vec<Value>>),
    ResponseError(String),
}

pub struct CardTemp {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    card_data: Option<Vec<Value>>,
    // error: Option<String>,
}

impl Component for CardTemp {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            fetch_task: None,
            link,
            card_data: Some(vec![]),
            // error: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RequestData => {
                //FETCHING...
                let request = Request::get("http://localhost:3000/index_card_data")
                    // .header("access_token", get_access_token{}.unwrap_or_default())
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<Vec<Value>, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
        
                        match data { 
                            Ok(dataok) => {
                                // ConsoleService::info(&format!("data response {:?}", &dataok));
                                Msg:: GetCardData(Some(dataok))
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

            Msg::GetCardData(data) => {
                // ConsoleService::info(&format!("data is {:?}", data));
                self.card_data = data;
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
			self.link.send_message(Msg::RequestData);
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
            <div>
                { self.view_data() }

                {
                    if self.card_data.clone().unwrap().is_empty() {
                        html!{
                            <div class="alert alert-danger m-4" role="alert">
                                { "No Record in this Index" }
                                
                            </div>
                        }
                    } else {
                        html! {
                            //NOTHING YET
                        }
                    }
                }
            </div>
            //BODY END
        }
    }
}


impl CardTemp {

    fn view_data(&self) -> Vec<Html> {
        self.card_data.iter().map(|card|{
                card.iter().map(|card_parse|{
                    html!{
                        <div class="index-card">
                            { serde_json::to_string(card_parse).unwrap() }
                        </div>
                    }
                }).collect()
                
            }).collect()
        }
}
