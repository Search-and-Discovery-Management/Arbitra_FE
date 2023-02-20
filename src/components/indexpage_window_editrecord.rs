use yew::prelude::*;

pub enum Msg {
    ToggleEditRecord,
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct WindowEditRecordProps {
    // #[prop_or(String::from("this is value"))]
    #[prop_or(false)]
    pub display_edit_record: bool,
    pub on_toggle_editrecord:Callback<Msg>,
}


pub struct EditRecord {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    props: WindowEditRecordProps,
    callback_toggle_editecord: Callback<Msg>,
}

impl Component for EditRecord {
    type Message = Msg;
    type Properties = WindowEditRecordProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            callback_toggle_editecord: props.on_toggle_editrecord.clone(),
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleEditRecord => {
                self.callback_toggle_editecord.emit(Msg::ToggleEditRecord);
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
                                >
                            {""}  
                            </textarea>
                        </form>   
                    </div> 

                    <button 
                        type="submit"
                        form="submit-editrecord"
                        class="window-confirm-button"
                    >
                        { "EDIT RECORD" }
                    </button>
                    
                </div>

            </div>
        }
    }
}