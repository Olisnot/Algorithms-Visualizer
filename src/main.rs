mod searching;
mod sorting;
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use searching::SearchingView;
use sorting::SortingView;

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    struct Tab {
        id: i32,
        label: &'static str,
        content: Box<dyn Fn() -> AnyView + Send + Sync>,
    }

    impl Tab {
        pub fn new<F>(id: i32, label: &'static str, f: F) -> Self
        where
            F: Fn() -> AnyView + Send + Sync + 'static,
        {
            Self {
                id,
                label,
                content: Box::new(f),
            }
        }
    }

    let tabs = [
        Tab::new(0, "Sorting", || view! { <SortingView/> }.into_any()),
        Tab::new(1, "Searching", || view! { <SearchingView/> }.into_any()),
    ];

    let (active_tab, set_active_tab) = signal(tabs[0].id);

    view! {
        <body class:bg=move || true />
        <div class="tabs">
            {tabs
                .iter()
                .map(|tab| {
                    let id = tab.id;
                    let label = tab.label;
                    view! {
                        <button
                            class="tab-button"
                            class:active=move || active_tab.get() == id
                            on:click=move |_| set_active_tab.set(id)
                        >
                            {label}
                        </button>
                    }
                })
                .collect_view()}
        </div>
        <div class="tab-content">
            {move || {
                tabs
                    .iter()
                    .find(|tab| tab.id == active_tab.get())
                    .map(|tab| (tab.content)())
                    .unwrap()
            }}
        </div>
    }
}
