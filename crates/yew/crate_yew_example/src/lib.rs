use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::web_sys;
mod lib2;

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

enum Msg {
    AddOne,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self{
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::AddOne)> { "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }

    fn rendered(&mut self, _first_render: bool) {

    }

    fn destroy(&mut self) {

    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    let window: web_sys::Window = web_sys::window().expect("window not available");
    window.alert_with_message("hello from wasm!").expect("alert failed");

    App::<Model>::new().mount_to_body();
    App::<lib2::Model>::new().mount_to_body();
}
