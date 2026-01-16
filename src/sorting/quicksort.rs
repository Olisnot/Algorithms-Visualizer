use crate::chart::BarChart;
use async_recursion::async_recursion;
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos::task::spawn_local;
use rand::Rng;

#[component]
pub fn QuicksortView() -> impl IntoView {
    let rng = rand::rng();
    let nums: Vec<f64> = rng.random_iter().take(100).collect();
    let end = nums.len() as i32 - 1;
    let numbers = RwSignal::new(nums);

    let domain = numbers.with_untracked(|v| {
        let min = v.iter().copied().fold(f64::INFINITY, f64::min);
        let max = v.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        (min, max)
    });

    view! {
        <p> Quick Sort </p>
            <BarChart data=numbers.read_only() domain=domain />
            <button class="btn" on:click=move |_| {
                spawn_local(async move {
                    sort(numbers, 0, end).await;
                    log::info!("{}", print_array(numbers.get_untracked()));
                });
            }>
            "Sort"
            </button>
    }
}

#[async_recursion(?Send)]
async fn sort(numbers: RwSignal<Vec<f64>>, start: i32, end: i32) {
    if end <= start {
        return;
    }
    let pivot = partition(numbers, start, end).await;
    sort(numbers, start, pivot - 1).await;
    sort(numbers, pivot + 1, end).await;
}

#[async_recursion(?Send)]
async fn partition(numbers: RwSignal<Vec<f64>>, start: i32, end: i32) -> i32 {
    let pivot = numbers.with_untracked(|v| v[end as usize]);
    let mut i = start - 1;

    for j in start..end {
        if numbers.with_untracked(|v| v[j as usize]) < pivot {
            i += 1;
            numbers.update(|n| n.swap(i as usize, j as usize));
            TimeoutFuture::new(16).await;
        }
    }
    i += 1;
    numbers.update(|n| n.swap(i as usize, end as usize));
    TimeoutFuture::new(16).await;
    i
}

fn print_array(numbers: Vec<f64>) -> String {
    let mut message = "".to_string();
    for i in numbers.iter() {
        message += &i.to_string();
        message += " ";
    }
    message.to_string()
}
