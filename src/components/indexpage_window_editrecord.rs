use yew::{prelude::*, services::ConsoleService};

pub enum Msg {
    ToggleEditRecord,
    ValidateInputJson(String)
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct WindowEditRecordProps {
    // #[prop_or(String::from("this is value"))]
    #[prop_or(false)]
    pub display_edit_record: bool,

    pub on_toggle_editrecord:Callback<Msg>,
    // #[prop_or_default]
    // pub edit_data: String,
}


pub struct EditRecord {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    props: WindowEditRecordProps,
    callback_toggle_editrecord: Callback<Msg>,
    value: String,
    json_is_valid: bool,
}

impl Component for EditRecord {
    type Message = Msg;
    type Properties = WindowEditRecordProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            callback_toggle_editrecord: props.on_toggle_editrecord.clone(),
            props,
            value: "".to_string(),
            json_is_valid: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleEditRecord => {
                self.callback_toggle_editrecord.emit(Msg::ToggleEditRecord);
                // ConsoleService::info(&format!("DEBUG : self.edit_data COMPONENT:{:?}", self.props.edit_data.clone()));
                true
            }
            Msg::ValidateInputJson (data) => {
                self.value = data;
                self.json_is_valid = match serde_json::from_str::<serde_json::Value>(&self.value) {
                    Ok(_) => true,
                    Err(_) => false,
                };
                ConsoleService::info(&format!("DEBUG : value:{:?}", self.value));
                ConsoleService::info(&format!("DEBUG : json_is_valid:{:?}", self.json_is_valid));
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
                <div class="window-index" id="create-index"> 

                    <div class="top-row-index-window-insert">
                        <h1>{"EDIT RECORD #"}{""}</h1>
                        
                        <button 
                            type="button" 
                            class="window-index-closebutton"
                            onclick=self.link.callback(|_| Msg::ToggleEditRecord)>
                                <img src="images/close.png" alt="close window" style="width: 32px"/> 
                        </button>
                    </div> 

                    <h6>{"Edit this record's JSON values"}
                     </h6>

                    <div class="window-submit-form">
                        <form class="record-text-input" id="submit-editrecord">
                            <textarea 
                                type="text" 
                                class="insert-record" 
                                
                                oninput = self.link.callback(|data: InputData| Msg::ValidateInputJson(data.value))
                                >
                            {"self.props.edit_data.clone()"}  
                            </textarea>
                        </form>   
                    </div> 

                    {
                        if self.json_is_valid {
                            html!{
                                <button 
                                type="submit"
                                form="submit-editrecord"
                                class="window-confirm-button"
                                >
                                { "EDIT RECORD" }
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

            </div>
        }
    }
}