use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;

pub async fn interpolation_search(
    numbers: RwSignal<Vec<i64>>,
    query: i64,
    highlighted: WriteSignal<Option<usize>>,
) {
    let nums = numbers.get_untracked();
    let mut high = nums.len() - 1;
    let mut low = 0;

    while query >= nums[low] && query <= nums[high] && low <= high {
        let probe = low
            + (high - low) * (query as usize - nums[low] as usize)
                / (nums[high] as usize - nums[low] as usize);

        highlighted.set(Some(probe));
        TimeoutFuture::new(500).await;

        if nums[probe] == query {
            return;
        } else if nums[probe] < query {
            low = probe + 1;
        } else {
            high = probe - 1;
        }
    }
}
