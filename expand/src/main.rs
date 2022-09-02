use std::rc::Rc;
use honey::{Form, CustomProps};
use yew::*;

#[derive(PartialEq, Default, Clone)]
pub struct QuizState {
    pub expanded: bool,
}

#[function_component(ImageInput)]
pub fn image_input(props: &CustomProps<String, QuizState>) -> Html {
    html! {
        <>
        <cobul::Button click={props.change.reform(|_| QuizState {expanded: true})} />
        <p> {props.state.expanded} </p>
        </>
    }
}

#[derive(Form, PartialEq, Clone, Debug)]
#[form(state = "QuizState")]
pub struct Quiz {
    #[form(input)]
    pub title: String,

    #[form(hidden)]
    pub public: bool,

    #[form(custom = "image_input")]
    pub image: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let quiz = Quiz {
        title: "title".to_string(),
        public: true,
        image: "".to_owned(),
    };

    let state = use_state(|| Rc::new(quiz));
    let input = ywt::callback!(state; move |quiz| state.set(quiz));

    let debug = format!("{:?}", &*state);

    html! {
        <cobul::Container>
        <QuizForm value={(*state).clone()} {input} />
        <p> {debug} </p>
        </cobul::Container>
    }
}

fn main() {
    Renderer::<App>::new().render();
}

// fn main() {}