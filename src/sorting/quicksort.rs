use leptos::prelude::*;

#[component]
pub fn QuicksortView() -> impl IntoView {
    let numbers = [5, 2, 7, 10, 33, 1];
    let mut numbers_sorted = numbers;
    let end = numbers_sorted.len() as i32 - 1;
    sort(&mut numbers_sorted, 0, end);
    view! { <p> Quick Sort </p> <p>{print_array(numbers)}</p> <p>{print_array(numbers_sorted)}</p>}
}

fn sort(numbers: &mut [i32], start: i32, end: i32) {
    if end <= start {
        return;
    }
    let pivot = partition(numbers, start, end);
    sort(numbers, start, pivot - 1);
    sort(numbers, pivot + 1, end);
}

fn partition(numbers: &mut [i32], start: i32, end: i32) -> i32 {
    let pivot = numbers[end as usize];
    let mut i = start - 1;

    for j in start..end {
        if numbers[j as usize] < pivot {
            i += 1;
            numbers.swap(i as usize, j as usize);
        }
    }
    i += 1;
    numbers.swap(i as usize, end as usize);
    i
}

fn print_array(numbers: [i32; 6]) -> String {
    let mut message = "".to_string();
    for i in numbers {
        message += &i.to_string();
        message += " ";
    }
    message.to_string()
}
