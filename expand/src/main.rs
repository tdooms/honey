use std::rc::Rc;
use honey::{Form, CustomProps};
use yew::*;

pub struct QuizState {
    pub expanded: UseStateHandle<bool>
}

#[function_component(ImageInput)]
pub fn image_input(props: &CustomProps<String>) -> Html {
    html! {<p> {"unpit"} </p>}
}

#[derive(Form, PartialEq, Clone, Debug)]
#[form(state = "QuizState")]
pub struct Quiz {
    #[form(input)]
    pub title: String,

    #[form(checkbox)]
    pub public: bool,

    #[form(custom = "image_input")]
    pub image: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let quiz = Quiz {
        title: "title".to_string(),
        public: true,
        image: "".to_owned()
    };

    let state = use_state(|| Rc::new(quiz));
    let change = ywt::callback!(state; move |quiz| state.set(quiz));

    let debug = format!("{:?}", &*state);

    html! {
        <cobul::Container>
        <QuizForm value={(*state).clone()} {change} />
        <p> {debug} </p>
        </cobul::Container>
    }
}
fn main() {
    Renderer::<App>::new().render();
}

// fn main() {}