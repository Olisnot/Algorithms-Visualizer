#[cfg(not(test))]
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;

pub async fn bubble_sort(numbers: RwSignal<Vec<f64>>) {
    let len = numbers.with_untracked(|v| v.len());
    for i in 0..len {
        for j in 0..len - i - 1 {
            if numbers.get_untracked()[j] > numbers.get_untracked()[j + 1] {
                numbers.update(|n| n.swap(j, j + 1));
                animation_delay().await;
            }
        }
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

    async fn assert_bubble_sort(input: Vec<f64>, expected: Vec<f64>) {
        let numbers = RwSignal::new(input);
        bubble_sort(numbers).await;
        assert_eq!(numbers.get_untracked(), expected);
    }

    #[wasm_bindgen_test]
    async fn sorts_empty_input() {
        assert_bubble_sort(vec![], vec![]).await;
    }

    #[wasm_bindgen_test]
    async fn sorts_single_element() {
        assert_bubble_sort(vec![42.0], vec![42.0]).await;
    }

    #[wasm_bindgen_test]
    async fn sorts_unsorted_input() {
        assert_bubble_sort(vec![3.0, 1.0, 4.0, 2.0], vec![1.0, 2.0, 3.0, 4.0]).await;
    }

    #[wasm_bindgen_test]
    async fn keeps_sorted_input_sorted() {
        assert_bubble_sort(vec![1.0, 2.0, 3.0, 4.0], vec![1.0, 2.0, 3.0, 4.0]).await;
    }

    #[wasm_bindgen_test]
    async fn sorts_reverse_order_and_duplicates() {
        assert_bubble_sort(vec![5.0, 4.0, 4.0, 2.0, 1.0], vec![1.0, 2.0, 4.0, 4.0, 5.0]).await;
    }
}
