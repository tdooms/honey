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

// pub struct QuizState {
//     pub expanded: UseStateHandle<bool>
// }

#[derive(Form)]
pub struct Quiz {
    #[form(field = "input")]
    pub title: String,

    #[form(field = "checkbox")]
    pub public: String,

    // #[field(custom("image_field"))]
    // pub image: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let quiz = Quiz {
        title: "title".to_string(),
        public: "public".to_string(),
    };

    html! {
        <cobul::Container>
        { quiz.view() }
        </cobul::Container>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
