mod binarysearch;
mod grid;
mod interpolationsearch;
mod linearsearch;
use binarysearch::binary_search;
use grid::NumberGrid;
use interpolationsearch::interpolation_search;
use leptos::prelude::*;
use leptos::task::spawn_local;
use linearsearch::linear_search;
use rand::seq::SliceRandom;

#[component]
pub fn SearchingView() -> impl IntoView {
    let nums: Vec<i64> = (0..100).collect();
    let numbers = RwSignal::new(nums);
    numbers.update(|v| v.shuffle(&mut rand::rng()));

    let (choice, set_choice) = signal("linear".to_string());
    let (is_searching, set_is_searching) = signal(false);
    let (is_sorted, set_is_sorted) = signal(false);
    let (query, set_query) = signal(String::new());
    let (highlighted, set_highlighted) = signal(None::<usize>);

    view! {
        <select
            class="select"
            disabled=move || is_searching.get()
            on:change=move |ev| {
                set_choice.set(event_target_value(&ev));
                numbers.set((0..100).collect());
                set_is_searching.set(false);
                set_highlighted.set(None);
                if choice.get_untracked() == "linear" {
                    numbers.update(|v| v.shuffle(&mut rand::rng()));
                    set_is_sorted.set(false);
                }
                else if choice.get_untracked() == "interpolation" {
                    numbers.update(|v| {
                        for n in v.iter_mut() {
                            *n = *n * *n /2;
                        }
                    });
                }
            }
        >
            <option value="linear">"Linear Search"</option>
            <option value="binary">"Binary Search"</option>
            <option value="interpolation">"Interpolation Search"</option>
        </select>

        <p/>
            <input
                class="input"
                type="number"
                inputmode="numeric"
                min="0"
                max="100"
                step="1"
                disabled=move || is_searching.get()
                placeholder="Search value"
                style="margin-right: 8px;"
                on:input=move |ev| set_query.set(event_target_value(&ev))
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
                                linear_search(numbers, query.get_untracked().parse().unwrap(), set_highlighted).await
                            }
                            "binary" => {
                                binary_search(numbers, query.get_untracked().parse().unwrap(), set_highlighted).await
                            }
                            "interpolation" => {
                                interpolation_search(numbers, query.get_untracked().parse().unwrap(), set_highlighted).await
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
                    numbers.set((0..100).collect());
                    if choice.get_untracked().as_str() == "linear" && !is_sorted.get_untracked() {
                        numbers.update(|v| v.shuffle(&mut rand::rng()));
                    }
                    set_highlighted.set(None);
                }
            >
            "Reset"
            </button>
            <button class="btn" disabled=move || {is_searching.get() || choice.get_untracked().as_str() != "linear"} on:click=move |_| {
                numbers.set((0..100).collect());
                if is_sorted.get_untracked() {
                    numbers.update(|v| v.shuffle(&mut rand::rng()));
                    set_is_sorted.set(false);
                    set_highlighted.set(None);
                }
                else {
                    set_is_sorted.set(true);
                    set_highlighted.set(None);
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
            <NumberGrid data=numbers.read_only() highlighted=highlighted />
    }
}
