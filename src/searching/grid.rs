use leptos::prelude::*;

#[component]
pub fn NumberGrid(
    data: ReadSignal<Vec<i64>>,
    highlighted: ReadSignal<Option<usize>>,
) -> impl IntoView {
    view! {
        <div
            style="display: grid; grid-template-columns: repeat(10, 48px); gap: 6px;"
        >
            {move || {
                let highlighted_index = highlighted.get();
                data.get()
                    .into_iter()
                    .enumerate()
                    .map(|(index, value)| {
                        let is_highlighted = highlighted_index == Some(index);
                        let style = if is_highlighted {
                            "padding: 6px; border: 1px solid var(--base0D); border-radius: 6px; text-align: center; background: var(--base02); color: var(--base06);"
                        } else {
                            "padding: 6px; border: 1px solid var(--base02); border-radius: 6px; text-align: center; background: var(--base01);"
                        };
                        view! {
                            <div style=style>{value.to_string()}</div>
                        }
                    })
                    .collect_view()
            }}
        </div>
    }
}
