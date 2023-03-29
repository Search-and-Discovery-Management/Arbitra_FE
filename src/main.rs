mod pages;
mod components;
mod router;
mod app;
mod store;
mod types;

use app::App;

fn main() {
    yew::start_app::<App>();
}