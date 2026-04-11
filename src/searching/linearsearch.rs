#[cfg(not(test))]
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;

pub async fn linear_search(
    numbers: RwSignal<Vec<i64>>,
    query: i64,
    highlighted: WriteSignal<Option<usize>>,
) {
    for (index, num) in numbers.get_untracked().iter().enumerate() {
        highlighted.set(Some(index));
        if num == &query {
            return;
        }
        animation_delay().await;
    }

    highlighted.set(None);
}

#[cfg(not(test))]
async fn animation_delay() {
    TimeoutFuture::new(200).await;
}

#[cfg(test)]
async fn animation_delay() {}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    async fn run_linear_search(input: Vec<i64>, query: i64) -> Option<usize> {
        let numbers = RwSignal::new(input);
        let (highlighted, set_highlighted) = signal(None::<usize>);
        linear_search(numbers, query, set_highlighted).await;
        highlighted.get_untracked()
    }

    #[wasm_bindgen_test]
    async fn finds_first_element() {
        assert_eq!(run_linear_search(vec![4, 2, 7], 4).await, Some(0));
    }

    #[wasm_bindgen_test]
    async fn finds_middle_element() {
        assert_eq!(run_linear_search(vec![4, 2, 7, 9], 7).await, Some(2));
    }

    #[wasm_bindgen_test]
    async fn returns_none_when_value_is_missing() {
        assert_eq!(run_linear_search(vec![4, 2, 7, 9], 5).await, None);
    }

    #[wasm_bindgen_test]
    async fn returns_none_for_empty_input() {
        assert_eq!(run_linear_search(vec![], 5).await, None);
    }
}
