#[cfg(not(test))]
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;

pub async fn interpolation_search(
    numbers: RwSignal<Vec<i64>>,
    query: i64,
    highlighted: WriteSignal<Option<usize>>,
) {
    let nums = numbers.get_untracked();
    if nums.is_empty() {
        highlighted.set(None);
        return;
    }

    let mut high = nums.len() - 1;
    let mut low = 0;

    while query >= nums[low] && query <= nums[high] && low <= high {
        if nums[low] == nums[high] {
            highlighted.set(if nums[low] == query { Some(low) } else { None });
            return;
        }

        let probe = low
            + (high - low) * (query as usize - nums[low] as usize)
                / (nums[high] as usize - nums[low] as usize);

        highlighted.set(Some(probe));
        animation_delay().await;

        if nums[probe] == query {
            return;
        } else if nums[probe] < query {
            low = probe + 1;
        } else {
            high = probe - 1;
        }
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

    async fn run_interpolation_search(input: Vec<i64>, query: i64) -> Option<usize> {
        let numbers = RwSignal::new(input);
        let (highlighted, set_highlighted) = signal(None::<usize>);
        interpolation_search(numbers, query, set_highlighted).await;
        highlighted.get_untracked()
    }

    #[wasm_bindgen_test]
    async fn finds_existing_value_in_evenly_distributed_input() {
        assert_eq!(
            run_interpolation_search(vec![0, 2, 4, 6, 8], 6).await,
            Some(3)
        );
    }

    #[wasm_bindgen_test]
    async fn returns_none_when_value_is_missing() {
        assert_eq!(run_interpolation_search(vec![0, 2, 4, 6, 8], 5).await, None);
    }

    #[wasm_bindgen_test]
    async fn returns_none_when_query_is_out_of_range() {
        assert_eq!(
            run_interpolation_search(vec![0, 2, 4, 6, 8], 10).await,
            None
        );
    }

    #[wasm_bindgen_test]
    async fn handles_single_value_input() {
        assert_eq!(run_interpolation_search(vec![9], 9).await, Some(0));
    }

    #[wasm_bindgen_test]
    async fn returns_none_for_empty_input() {
        assert_eq!(run_interpolation_search(vec![], 9).await, None);
    }
}
