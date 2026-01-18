use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;

pub async fn binary_search(
    numbers: RwSignal<Vec<i64>>,
    query: i64,
    highlighted: WriteSignal<Option<usize>>,
) {
    let nums = numbers.get_untracked();

    if nums.is_empty() {
        return;
    }

    let mut min = 0;
    let mut max = nums.len() - 1;

    while min <= max {
        let middle = min + (max - min) / 2;
        let value = nums[middle];
        highlighted.set(Some(middle));
        TimeoutFuture::new(500).await;

        if value < query {
            min = middle + 1;
        } else if value > query {
            max = middle - 1;
        } else {
            highlighted.set(Some(middle));
            return;
        };
    }
    highlighted.set(None);
}
