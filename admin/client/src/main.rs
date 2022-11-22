use yew::{function_component, html};

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{"Hello Yew!"}</h1>
    }
}

fn main() {
    yew::start_app::<App>();
}
