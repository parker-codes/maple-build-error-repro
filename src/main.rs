use maple_core::prelude::*;

fn main() {
    let root = template! {
        p {
            "Hello World!"
        }
    };

    render(|| root);
}
