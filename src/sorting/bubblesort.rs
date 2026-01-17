use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;

pub async fn bubble_sort(numbers: RwSignal<Vec<f64>>) {
    let len = numbers.with_untracked(|v| v.len());
    for i in 0..len {
        for j in 0..len - i - 1 {
            if numbers.get_untracked()[j] > numbers.get_untracked()[j + 1] {
                numbers.update(|n| n.swap(j, j + 1));
                TimeoutFuture::new(16).await;
            }
        }
    }
}
