use your_crate::your_attr;

#[your_attr(path = "hello", timeout_ms = 15)]
fn do_stuff() {
    println!("Hello");
}

fn main() {
    println!("Hello, world!");
}
