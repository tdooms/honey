use yew::Callback;
pub use honey_derive::*;

#[derive(yew::Properties, PartialEq)]
pub struct CustomProps<T: PartialEq, S: PartialEq = ()> {
    pub value: T,
    pub input: Callback<T>,

    pub state: S,
    pub change: Callback<S>,

    pub error: Option<String>,

    pub submit: Callback<()>,
    pub cancel: Callback<()>,
}