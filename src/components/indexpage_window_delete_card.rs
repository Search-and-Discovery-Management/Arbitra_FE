use yew::{prelude::*, services::ConsoleService};

pub enum Msg {
    ToggleDeleteCard,
}


#[derive(Properties, Clone, Debug, PartialEq)]
pub struct WindowDeleteCardProps {
    // #[prop_or(String::from("this is value"))]
    #[prop_or(false)]
    pub display_delete_card: bool,
    #[prop_or_default]
    pub delete_index: String,
    pub on_toggle_deletecard:Callback<Msg>,
}


pub struct DeleteCard {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    props: WindowDeleteCardProps,
    callback_toggle_deletecard: Callback<Msg>,
}

impl Component for DeleteCard {
    type Message = Msg;
    type Properties = WindowDeleteCardProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            callback_toggle_deletecard: props.on_toggle_deletecard.clone(),
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleDeleteCard => {
                self.callback_toggle_deletecard.emit(Msg::ToggleDeleteCard);
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
                        <h1>{"DELETE RECORD "}{self.props.delete_index.clone()}</h1>
                        
                        <button 
                            type="button" 
                            class="window-index-closebutton"
                            onclick=self.link.callback(|_| Msg::ToggleDeleteCard)>
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
                        form="submit-deletecard"
                        class="window-confirm-button"
                        //WIRING EVENT DELETE CARD / RECORD DISINI!!
                    >
                        { "DELETE RECORD" }
                    </button>
                    
                </div>

            </div>
        }
    }
}