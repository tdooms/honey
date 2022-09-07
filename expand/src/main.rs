use std::rc::Rc;
use validator::ValidationError;
use honey::{Form, CustomProps};
use yew::*;

#[derive(PartialEq, Default, Clone)]
pub struct QuizState {
    pub expanded: bool,
}

#[function_component(ImageInput)]
pub fn image_input(props: &CustomProps<String, ()>) -> Html {
    // html! {
    //     <>
    //     <cobul::Button click={props.change.reform(|_| QuizState {expanded: true})} > {"expand"} </cobul::Button>
    //     <p> {props.state.expanded} </p>
    //     </>
    // }
    html!{}
}

fn must_be_true(pred: &bool) -> Result<(), ValidationError> {
    pred.then(|| ()).ok_or(ValidationError::new(""))
}

#[derive(Form, PartialEq, Clone, Debug, validator::Validate)]
#[form(submit, cancel)]
pub struct Quiz {
    #[form(input)]
    pub title: String,

    #[validate(custom(function = "must_be_true", message = "Must be true"))]
    #[form(checkbox)]
    pub public: bool,

    #[validate(length(min = 1, message = "Must be older than 10"))]
    #[form(input)]
    pub age: String,

    #[form(custom = "image_input")]
    pub image: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let quiz = Quiz {
        title: "title".to_string(),
        age: 19.to_string(),
        public: true,
        image: "".to_owned(),
    };

    let state = use_state(|| Rc::new(quiz));
    let input = ywt::callback!(state; move |quiz| state.set(quiz));

    let submit = ywt::callback!(|_| log::info!("submit"));

    let debug = format!("{:?}", &*state);

    html! {
        <cobul::Container>
        <QuizForm value={(*state).clone()} {input} {submit}/>
        <p> {debug} </p>
        </cobul::Container>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    Renderer::<App>::new().render();
}

// fn main() {}