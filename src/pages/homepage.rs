use yew::prelude::*;

pub enum Msg {}

pub struct HomePage {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for HomePage {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {

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
                <div class="nav-red">
                    <a href="">{ "About Us" }</a>
                    <a href="">{ "Partners" }</a>
                    <a href="">{ "Support" }</a>
                    <a href="/loginpage">{ "Login" }</a>
                </div>

                <div class="nav-grey">
                    <img class="nav-logo" src="images/Arbitra_Horizontal1.png"/>
                    <a href="/loginpage">{ "Platform" }</a>
                    <a href="/loginpage">{ "Pricing" }</a>
                    <a href="/loginpage">{ "Customers" }</a>
                    <a href="/loginpage">{ "Developers" }</a>

                    <input class="search-bar-navgrey" type="text" placeholder="Search.." />
                </div>
            </div>
        }
    }
}