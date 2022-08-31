use std::rc::Rc;

#[derive(yew::Properties, PartialEq)]
pub struct CustomProps<T, S = ()> {
    pub value: Rc<T>,
    pub input: yew::Callback<Rc<T>>,
    pub error: Option<String>,
    pub state: Option<S>
}