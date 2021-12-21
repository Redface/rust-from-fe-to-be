use yew::{function_component, html, Html};

mod components;

use components::{
    header::Header,
    footer::Footer,
    body::Body,
};

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <Header />
            <Body />
            <Footer />
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}