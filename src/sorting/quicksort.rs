use async_recursion::async_recursion;
#[cfg(not(test))]
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
            animation_delay().await;
        }
    }
    i += 1;
    numbers.update(|n| n.swap(i as usize, end as usize));
    animation_delay().await;
    i
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

    async fn assert_quick_sort(input: Vec<f64>, expected: Vec<f64>) {
        let numbers = RwSignal::new(input);
        let end = numbers.with_untracked(|v| v.len() as i32 - 1);
        quick_sort(numbers, 0, end).await;
        assert_eq!(numbers.get_untracked(), expected);
    }

    #[wasm_bindgen_test]
    async fn sorts_empty_input() {
        assert_quick_sort(vec![], vec![]).await;
    }

    #[wasm_bindgen_test]
    async fn sorts_single_element() {
        assert_quick_sort(vec![42.0], vec![42.0]).await;
    }

    #[wasm_bindgen_test]
    async fn sorts_unsorted_input() {
        assert_quick_sort(vec![3.0, 1.0, 4.0, 2.0], vec![1.0, 2.0, 3.0, 4.0]).await;
    }

    #[wasm_bindgen_test]
    async fn keeps_sorted_input_sorted() {
        assert_quick_sort(vec![1.0, 2.0, 3.0, 4.0], vec![1.0, 2.0, 3.0, 4.0]).await;
    }

    #[wasm_bindgen_test]
    async fn sorts_reverse_order_and_duplicates() {
        assert_quick_sort(vec![5.0, 4.0, 4.0, 2.0, 1.0], vec![1.0, 2.0, 4.0, 4.0, 5.0]).await;
    }
}
