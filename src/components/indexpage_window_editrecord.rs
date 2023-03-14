use yew::{prelude::*, services::ConsoleService};
use serde_json::{from_str, Value, from_value, to_string_pretty};
pub enum Msg {
    ToggleEditRecord,
    ValidateInputJson(String)
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct WindowEditRecordProps {
    // #[prop_or(String::from("this is value"))]
    #[prop_or(false)]
    pub display_edit_record: bool,

    #[prop_or_default]
    pub edit_data: String,
    #[prop_or_default]
    pub edit_index: usize,

    pub on_toggle_editrecord:Callback<Msg>,
}


pub struct EditRecord {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    props: WindowEditRecordProps,
    callback_toggle_editecord: Callback<Msg>,
    //Validasi Input / Edit Json
    value: String,
    json_is_valid: bool,

    textarea_string: String,

}

impl Component for EditRecord {
    type Message = Msg;
    type Properties = WindowEditRecordProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {

        let textarea_value = props.edit_data.clone();
        let textarea_parse:Value = serde_json::from_str(&textarea_value).unwrap();
        let textarea_pretty = serde_json::to_string_pretty(&textarea_parse).unwrap();

        Self {
            link,
            callback_toggle_editecord: props.on_toggle_editrecord.clone(),
            props,
            value: "".to_string(),
            json_is_valid: false,

            textarea_string: textarea_pretty,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleEditRecord => {
                self.callback_toggle_editecord.emit(Msg::ToggleEditRecord);
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

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.edit_data != props.edit_data {
            self.props.edit_data = props.edit_data;
            // self.role_permissions = props.role_permissions;
            true
        }else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="window-overlay">
                <div class="window-index" id="create-index"> 

                    <div class="top-row-index-window-insert">
                        <h1>{"EDIT RECORD #"}{self.props.edit_index.clone()+1}</h1>
                        
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
                                style="font-size:12px;font-weight: bold; line-height: 1.4;"

                                oninput = self.link.callback(|data: InputData| Msg::ValidateInputJson(data.value))
                                >
                            {self.textarea_string.clone()}  
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