pub use honey_derive::*;

#[derive(yew::Properties, PartialEq)]
pub struct CustomProps<T: PartialEq, S: PartialEq> {
    pub value: T,
    pub input: yew::Callback<T>,
    pub error: Option<String>,
    pub state: S
}