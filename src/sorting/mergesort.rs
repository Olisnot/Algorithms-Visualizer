use async_recursion::async_recursion;
#[cfg(not(test))]
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
        animation_delay().await;
    }

    while l < left.len() {
        let value = left[l];
        numbers.update(|v| v[i] = value);
        l += 1;
        i += 1;
        animation_delay().await;
    }

    while r < right.len() {
        let value = right[r];
        numbers.update(|v| v[i] = value);
        r += 1;
        i += 1;
        animation_delay().await;
    }
}

#[cfg(not(test))]
async fn animation_delay() {
    TimeoutFuture::new(16).await;
}

#[cfg(test)]
async fn animation_delay() {}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    async fn assert_merge_sort(input: Vec<f64>, expected: Vec<f64>) {
        let numbers = RwSignal::new(input);
        merge_sort(numbers).await;
        assert_eq!(numbers.get_untracked(), expected);
    }

    #[wasm_bindgen_test]
    async fn sorts_empty_input() {
        assert_merge_sort(vec![], vec![]).await;
    }

    #[wasm_bindgen_test]
    async fn sorts_single_element() {
        assert_merge_sort(vec![42.0], vec![42.0]).await;
    }

    #[wasm_bindgen_test]
    async fn sorts_unsorted_input() {
        assert_merge_sort(vec![3.0, 1.0, 4.0, 2.0], vec![1.0, 2.0, 3.0, 4.0]).await;
    }

    #[wasm_bindgen_test]
    async fn keeps_sorted_input_sorted() {
        assert_merge_sort(vec![1.0, 2.0, 3.0, 4.0], vec![1.0, 2.0, 3.0, 4.0]).await;
    }

    #[wasm_bindgen_test]
    async fn sorts_reverse_order_and_duplicates() {
        assert_merge_sort(vec![5.0, 4.0, 4.0, 2.0, 1.0], vec![1.0, 2.0, 4.0, 4.0, 5.0]).await;
    }
}
