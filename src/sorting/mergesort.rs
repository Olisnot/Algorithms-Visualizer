use async_recursion::async_recursion;
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;

#[async_recursion(?Send)]
pub async fn merge_sort(numbers: RwSignal<Vec<f64>>) {
    let len = numbers.with_untracked(|v| v.len());
    if len <= 1 {
        return;
    }

    merge(numbers, 0, len).await;
}

#[async_recursion(?Send)]
async fn merge(numbers: RwSignal<Vec<f64>>, start: usize, end: usize) {
    if end - start <= 1 {
        return;
    }

    let mid = start + (end - start) / 2;
    merge(numbers, start, mid).await;
    merge(numbers, mid, end).await;

    let left = numbers.with_untracked(|v| v[start..mid].to_vec());
    let right = numbers.with_untracked(|v| v[mid..end].to_vec());

    let mut l = 0;
    let mut r = 0;
    let mut i = start;

    while l < left.len() && r < right.len() {
        if left[l] <= right[r] {
            let value = left[l];
            numbers.update(|v| v[i] = value);
            l += 1;
        } else {
            let value = right[r];
            numbers.update(|v| v[i] = value);
            r += 1;
        }
        i += 1;
        TimeoutFuture::new(16).await;
    }

    while l < left.len() {
        let value = left[l];
        numbers.update(|v| v[i] = value);
        l += 1;
        i += 1;
        TimeoutFuture::new(16).await;
    }

    while r < right.len() {
        let value = right[r];
        numbers.update(|v| v[i] = value);
        r += 1;
        i += 1;
        TimeoutFuture::new(16).await;
    }
}
