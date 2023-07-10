use leptos::*;

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    let (x, set_x) = create_signal(cx, 0);
    let (y, set_y) = create_signal(cx, 0);

    view! { cx,
        <button
            class:red=move || count() % 2 == 1
            on:click=move |_| {
                set_count.update(|n| *n += 1);
                set_x.update(|n| *n += 10);
                set_y.update(|n| *n += 10);
            }
        >
            "Click me: "
            {count}
        </button>

        <div
            style="position: absolute"
            style:left=move || format!("{}px", x() + 100)
            style:top=move || format!("{}px", y() + 100)
            style:background-color=move || format!("rgb({}, {}, 100)", x(), y())
            style=("--columns", x)
        >
            "Moves when coordinates change"
        </div>
        <br/>
        <progress
            max="50"
            value=count
        />
        <br/>
        <progress
            max="50"
            value=move || count() * 2
        />
    }
}