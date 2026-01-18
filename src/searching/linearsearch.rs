use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;

pub async fn linear_search(
    numbers: RwSignal<Vec<i64>>,
    query: i64,
    highlighted: WriteSignal<Option<usize>>,
) {
    let mut index = 0;
    highlighted.set(Some(index));

    for num in numbers.get_untracked().iter() {
        if num == &query || index >= 100 {
            return;
        } else {
            index += 1;
            highlighted.set(Some(index));
            TimeoutFuture::new(200).await;
        }
    }
}
