use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/start-axum.css"/>
        <Title text="Welcome to Leptos"/>
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <div class="m-10">
            <h1 class="text-3xl">"Welcome to Leptos!"</h1>
            <div class="flex items-center">
                <button class="m-5 p-2 bg-red-500 rounded-lg hover:scale-105 transition-transform" on:click=on_click>"Click Me !"</button>
                <p class="text-xl">{count}</p>
            </div>
        </div>
    }
}
