use hello_derive::Hello;
use macros::Hello;

fn main() {
    User::hello();
}

#[derive(Hello)]
struct User{}
