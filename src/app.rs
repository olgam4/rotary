use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/start-axum.css"/>
        <Title text="Rotary"/>
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
    let (email, set_email) = create_signal::<String>(cx, "".to_string());
    let (password, set_password) = create_signal::<String>(cx, "".to_string());

    let submit = move |_| {
        let email = email.get();
        let password = password.get();

        log!("email: {}, password: {}", email, password);

        set_email("".to_string());
        set_password("".to_string());
    };
    view! { cx,
        <div class="m-10 space-y-2">
            <h1 class="text-3xl">"Welcome to Rotary!"</h1>
            <div class="flex items-center space-x-4">
                <input
                    class="border border-gray-300 rounded p-2"
                    prop:value={move || email.get()}
                    placeholder="john@doe.ca"
                    type="email"
                    on:input=move |ev| {
                        let input = event_target_value(&ev);
                        set_email(input);
                    }
                />
                <input
                    class="border border-gray-300 rounded p-2"
                    prop:value={move || password.get()}
                    placeholder="Password"
                    type="password"
                    on:input=move |ev| {
                        let input = event_target_value(&ev);
                        set_password(input);
                    }
                />
            </div>
            <button on:click=submit class="bg-blue-500 text-white rounded p-2 hover:scale-105">
                "Submit"
            </button>
        </div>
    }
}
