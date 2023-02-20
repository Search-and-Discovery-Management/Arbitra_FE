use yew::prelude::*;

pub enum Msg {
    ToggleCreateIndex,
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
}

impl Component for IndexCreate {
    type Message = Msg;
    type Properties = WindowCreateIndexProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            callback_toggle_createindex: props.on_toggle_createindex.clone(),
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg{
            Msg::ToggleCreateIndex => {
                self.callback_toggle_createindex.emit(Msg::ToggleCreateIndex);
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
                            placeholder="Index name here..."/>
                        // <div class="window-confirm-button">
                    // </div>
                    </form>  
                    
                    <button 
                        type="submit"
                        form="submit-createindex"
                        class="window-confirm-button"
                    >
                            { "CREATE INDEX" }
                    </button>

                    
                    
                </div>

            </div>
        }
    }
}