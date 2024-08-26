use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App /> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
            // the class: syntax reactively updates a single class
            // here, we'll set the `red` class when `count` is odd
            class:red=move || count.get() % 2 == 1
        >

            "Click me: "
            // on stable, this is move || count.get();
            {move || count.get()}
        </button>
        <progress
            max="50"
            // signals are functions, so `value=count` and `value=move || count.get()`
            // are interchangeable.
            value=move || count.get()
        />
        <p>
            <strong>"Reactive: "</strong>
            // you can insert Rust expressions as values in the DOM
            // by wrapping them in curly braces
            // if you pass in a function, it will reactively update
            {move || count.get()}
        </p>
        <p>
            <strong>"Reactive shorthand: won't work"</strong>
        // signals are functions, so we can remove the wrapping closure
        // I'm not using nightly so signals are NOT functions, fuck off
        // {count.get}
        </p>
        <p>
            <strong>"Not reactive: "</strong>
            // NOTE: if you write {count()}, this will *not* be reactive
            // it simply gets the value of count once
            {count.get()}
        </p>
    }
        /*
    let (x, set_x) = create_signal(0);
    view! {
        <button
            on:click=move |_| {
                set_x.update(|n| *n += 10);
            }
            // set the `style` attribute
            style="position: absolute"
            // and toggle individual CSS properties with `style:`
            style:left=move || format!("{}px", x.get() + 100)
            style:background-color=move || format!("rgb({}, {}, 100)", x.get(), 100)
            style:max-width="400px"
            // Set a CSS variable for stylesheet use
            style=("--columns", x)
        >
            "Click to Move "
            {move || x.get()}
        </button>
    }
*/
}