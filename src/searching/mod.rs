mod grid;
use grid::NumberGrid;
use leptos::prelude::*;
use leptos::task::spawn_local;
use rand::Rng;

#[component]
pub fn SearchingView() -> impl IntoView {
    let nums: Vec<i64> = (0..100)
        .map(|_| rand::rng().random_range(0..=100))
        .collect();
    let numbers = RwSignal::new(nums);

    let (choice, set_choice) = signal("linear".to_string());
    let (is_searching, set_is_searching) = signal(false);
    let (is_sorted, set_is_sorted) = signal(false);

    view! {
        <select
            class="select"
            disabled=move || is_searching.get()
            on:change=move |ev| {
                set_choice.set(event_target_value(&ev));
                numbers.set(rand::rng().random_iter().take(100).collect());
                set_is_searching.set(false);
            }
        >
            <option value="linear">"Linear Search"</option>
            <option value="binary">"Binary Search"</option>
        </select>

        <p/>
            <input
                class="input"
                disabled=move || is_searching.get()
                placeholder="Search value"
                style="margin-right: 8px;"
            />
            <button
                class="btn"
                disabled=move || is_searching.get()
                on:click=move |_| {
                    if is_searching.get_untracked() {
                        return;
                    }
                    set_is_searching.set(true);
                    spawn_local(async move {
                        match choice.get_untracked().as_str() {
                            "linear" => {
                                log::info!("Linear search placeholder");
                            }
                            "binary" => {
                                log::info!("Binary search placeholder");
                            }
                            _ => {
                                set_choice.set("linear".to_string());
                            }
                        }
                        set_is_searching.set(false);
                    });
                }
            >
            "Search"
            </button>
            <button
                class="btn"
                disabled=move || is_searching.get()
                on:click=move |_| {
                    if is_searching.get_untracked() {
                        return;
                    }
                    numbers.set(rand::rng().random_iter().take(100).collect());
                }
            >
            "Reset"
            </button>
            <button class="btn" disabled=move || is_searching.get() on:click=move |_| {
                if is_sorted.get_untracked() {
                    numbers.set((0..100).map(|_| rand::rng().random_range(0..=100)).collect());
                    set_is_sorted.set(false);
                }
                else {
                    numbers.update(|v| v.sort());
                    set_is_sorted.set(true);
                }
            }>
            {move ||
                if is_sorted.get() {
                    "Unsort"
                }
                else {
                    "Sort"
                }
            }
            </button>
        <p/>
            <NumberGrid data=numbers.read_only() />
    }
}
