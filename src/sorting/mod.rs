mod chart;
mod quicksort;
use leptos::prelude::*;

use quicksort::QuicksortView;

#[component]
pub fn SortingView() -> impl IntoView {
    view! { <QuicksortView/> }
}
