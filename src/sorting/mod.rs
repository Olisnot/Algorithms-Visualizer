mod bubblesort;
mod chart;
mod mergesort;
mod quicksort;
use bubblesort::bubble_sort;
use chart::BarChart;
use leptos::prelude::*;
use leptos::task::spawn_local;
use mergesort::merge_sort;
use quicksort::quick_sort;
use rand::Rng;

#[component]
pub fn SortingView() -> impl IntoView {
    let nums: Vec<f64> = rand::rng().random_iter().take(100).collect();
    let numbers = RwSignal::new(nums);

    let (choice, set_choice) = signal("quicksort".to_string());

    let domain = numbers.with_untracked(|v| {
        let min = v.iter().copied().fold(f64::INFINITY, f64::min);
        let max = v.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        (min, max)
    });

    view! {
        <select
            class="select"
            on:change=move |ev| {
                set_choice.set(event_target_value(&ev));
                numbers.set(rand::rng().random_iter().take(100).collect());
            }>
            <option value="quicksort">"Quick Sort"</option>
            <option value="bubblesort">"Bubble Sort"</option>
            <option value="bubblesort">"Merge Sort"</option>
        </select>

        <p/>
            <button class="btn" on:click=move |_| {
                spawn_local(async move {
                    match choice.get_untracked().as_str() {
                        "quicksort" => {
                            let end = numbers.with_untracked(|v| v.len() as i32 - 1);
                            quick_sort(numbers, 0, end).await
                        },
                        "bubblesort" => {
                            bubble_sort(numbers).await
                        },
                        "mergesort" => {
                            merge_sort(numbers).await
                        },
                        _ => {
                            set_choice.set("quicksort".to_string());
                        },
                    };
                    log::info!("{}", print_array(numbers.get_untracked()));
                });
            }>
            "Sort"
            </button>
        <p/>
            <BarChart data=numbers.read_only() domain=domain />
    }
}

fn print_array(numbers: Vec<f64>) -> String {
    let mut message = "".to_string();
    for i in numbers.iter() {
        message += &i.to_string();
        message += " ";
    }
    message.to_string()
}
