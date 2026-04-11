#[cfg(not(test))]
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;

pub async fn binary_search(
    numbers: RwSignal<Vec<i64>>,
    query: i64,
    highlighted: WriteSignal<Option<usize>>,
) {
    let nums = numbers.get_untracked();

    if nums.is_empty() {
        highlighted.set(None);
        return;
    }

    let mut min = 0isize;
    let mut max = nums.len() as isize - 1;

    while min <= max {
        let middle = min + (max - min) / 2;
        let middle = middle as usize;
        let value = nums[middle];
        highlighted.set(Some(middle));
        animation_delay().await;

        if value < query {
            min = middle as isize + 1;
        } else if value > query {
            max = middle as isize - 1;
        } else {
            highlighted.set(Some(middle));
            return;
        };
    }
    highlighted.set(None);
}

#[cfg(not(test))]
async fn animation_delay() {
    TimeoutFuture::new(500).await;
}

#[cfg(test)]
async fn animation_delay() {}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    async fn run_binary_search(input: Vec<i64>, query: i64) -> Option<usize> {
        let numbers = RwSignal::new(input);
        let (highlighted, set_highlighted) = signal(None::<usize>);
        binary_search(numbers, query, set_highlighted).await;
        highlighted.get_untracked()
    }

    #[wasm_bindgen_test]
    async fn finds_first_element() {
        assert_eq!(run_binary_search(vec![1, 3, 5, 7], 1).await, Some(0));
    }

    #[wasm_bindgen_test]
    async fn finds_last_element() {
        assert_eq!(run_binary_search(vec![1, 3, 5, 7], 7).await, Some(3));
    }

    #[wasm_bindgen_test]
    async fn returns_none_when_value_is_missing_below_range() {
        assert_eq!(run_binary_search(vec![1, 3, 5, 7], 0).await, None);
    }

    #[wasm_bindgen_test]
    async fn returns_none_when_value_is_missing_above_range() {
        assert_eq!(run_binary_search(vec![1, 3, 5, 7], 8).await, None);
    }

    #[wasm_bindgen_test]
    async fn returns_none_for_empty_input() {
        assert_eq!(run_binary_search(vec![], 5).await, None);
    }
}
