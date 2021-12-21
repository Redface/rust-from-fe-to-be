use yew::{function_component, html, Html};

#[function_component(Header)]
pub fn header_component() -> Html {
    html! {
      <div>{"Header"}</div>
   }
}