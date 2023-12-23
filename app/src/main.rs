use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
enum Route {
    #[at("/about")]
    About,
    #[at("/")]
    Home,
}

#[function_component(Home)]
fn home() -> Html {
    html! { <h1>{ "Welcome to Sangam!" }</h1> }
}

#[function_component(About)]
fn about() -> Html {
    html! { <h1>{ "About Sangam" }</h1> }
}

#[function_component(Main)]
fn app() -> Html {
    let render = Callback::from(|switch: Route| -> Html {
        match switch {
            Route::Home => html! { <Home /> },
            Route::About => html! { <About /> },
        }
    });

    html! {
        <BrowserRouter>
            <Switch<Route> render={render} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
