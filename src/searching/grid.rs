use leptos::prelude::*;

#[component]
pub fn NumberGrid(data: ReadSignal<Vec<i64>>) -> impl IntoView {
    view! {
        <div
            style="display: grid; grid-template-columns: repeat(10, 48px); gap: 6px;"
        >
            {move || {
                data.get()
                    .into_iter()
                    .enumerate()
                    .map(|(_index, value)| {
                        view! {
                            <div
                                style="padding: 6px; border: 1px solid var(--base02); border-radius: 6px; text-align: center; background: var(--base01);"
                            >
                                {format!("{value:.2}")}
                            </div>
                        }
                    })
                    .collect_view()
            }}
        </div>
    }
}
