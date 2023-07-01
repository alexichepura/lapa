use leptos::*;
use leptos_meta::Title;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Dashboard"/>
        <h1>"Dashboard"</h1>
        <div class="Card">"Hello"</div>
    }
}
