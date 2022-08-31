use std::rc::Rc;
use honey_derive::Form;
use yew::*;

// #[derive(Form)]
// #[form(state = "QuizState")]
// pub struct ConsumingType {
//     pub name: String,
//     #[form(field = "checkbox")]
//     pub verified: bool,
// }

// pub fn image_field(value: String, input: Callback<String>, error: Option<String>, state: State) -> Html {
//     state.expanded.set(true);
//     html! {<Input blabla>}
// }

pub struct QuizState {
    pub expanded: UseStateHandle<bool>
}

#[function_component(ImageInput)]
pub fn image_input(props: &CustomProps) -> Html {
    html! {<p> {"unpit"} </p>}
}

#[derive(Form, PartialEq, Clone, Debug)]
pub struct Quiz {
    #[form(input)]
    pub title: String,

    #[form(checkbox)]
    pub public: bool,

    #[form(custom = "image_input")]
    pub image: Option<String>,
}

#[function_component(App)]
pub fn app() -> Html {
    let quiz = Quiz {
        title: "title".to_string(),
        public: true,
        image: None
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