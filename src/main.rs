use sycamore::prelude::*;

fn main() {
    render(|| template! { App() });
}

#[component(App<G>)]
fn app() -> TemplateResult<G> {
    template! {
        Counter()

        TextBinding()
    }
}

#[component(Counter<G>)]
fn counter() -> TemplateResult<G> {
    let counter = Signal::new(0);

    let decrement = cloned!((counter) => move |_| counter.set(*counter.get() - 1));
    let increment = cloned!((counter) => move |_| counter.set(*counter.get() + 1));

    template! {
        div(class="counter") {
            button(class="decrement", on:click=decrement) { "-" }
            (counter.get())
            button(class="increment", on:click=increment) { "+" }
        }
    }
}

#[component(TextBinding<G>)]
fn text_binding() -> TemplateResult<G> {
    let value = Signal::new(String::new());

    let uppercase = create_memo(cloned!((value) => move || {
        return value.get().to_uppercase();
    }));

    template! {
        div(class="text-binding") {
            input(type="text", bind:value=value)

            div(class="uppercase-text") {
                (uppercase.get())
            }
        }
    }
}
