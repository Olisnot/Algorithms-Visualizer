use async_recursion::async_recursion;
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;

#[async_recursion(?Send)]
pub async fn quick_sort(numbers: RwSignal<Vec<f64>>, start: i32, end: i32) {
    if end <= start {
        return;
    }
    let pivot = partition(numbers, start, end).await;
    quick_sort(numbers, start, pivot - 1).await;
    quick_sort(numbers, pivot + 1, end).await;
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
