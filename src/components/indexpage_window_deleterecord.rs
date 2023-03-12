use yew::{prelude::*, services::ConsoleService};

pub enum Msg {
    ToggleDeleteRecord,
}


#[derive(Properties, Clone, Debug, PartialEq)]
pub struct WindowDeleteRecordProps {
    // #[prop_or(String::from("this is value"))]
    #[prop_or(false)]
    pub display_delete_record: bool,
    #[prop_or_default]
    pub delete_index: usize,
    pub on_toggle_deleterecord:Callback<Msg>,
}


pub struct DeleteRecord {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    props: WindowDeleteRecordProps,
    callback_toggle_deleterecord: Callback<Msg>,
}

impl Component for DeleteRecord {
    type Message = Msg;
    type Properties = WindowDeleteRecordProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            callback_toggle_deleterecord: props.on_toggle_deleterecord.clone(),
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleDeleteRecord => {
                self.callback_toggle_deleterecord.emit(Msg::ToggleDeleteRecord);
                ConsoleService::info(&format!("DEBUG : self.delete_index MODAL COMP:{:?}", self.props.delete_index.clone()));
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.delete_index != props.delete_index {
            self.props.delete_index = props.delete_index;
            true 
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="window-overlay">
                <div class="window-index" id="create-index"> 

                    <div class="top-row-index-window-insert">
                        <h1>{"DELETE RECORD #"}{self.props.delete_index.clone() + 1}</h1>
                        
                        <button 
                            type="button" 
                            class="window-index-closebutton"
                            onclick=self.link.callback(|_| Msg::ToggleDeleteRecord)>
                                <img src="images/close.png" alt="close window" style="width: 32px"/> 
                        </button>
                    </div> 

                    <h6>{"Are you sure?"}
                     </h6>

                    <button disabled=true class="window-delete-warning">
                        {"THIS OPERATION CANNOT BE REVERSED OR UNDONE!"}
                    </button> 

                    <button 
                        type="submit"
                        form="submit-deleterecord"
                        class="window-confirm-button"
                    >
                        { "DELETE RECORD" }
                    </button>
                    
                </div>

            </div>
        }
    }
}