use std::{thread, time::Instant};

use rand::seq::SliceRandom;

fn generate_numbers(n: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<usize> = (1..=n).collect();
    numbers.shuffle(&mut rng);
    numbers
}

fn selection_sort(v: &mut Vec<usize>) {
    let n = v.len();
    for i in 0..(n - 1) {
        let mut min_index = i;
        for j in i..n {
            if v[j] < v[min_index] {
                min_index = j;
            }
        }
        let temp = v[i];
        v[i] = v[min_index];
        v[min_index] = temp;
    }
}

fn insertion_sort(v: &mut Vec<usize>) {
    let n = v.len();
    for i in 0..(n) {
        for j in 0..i {
            if v[i] < v[j] {
                let temp = v[j];
                v[j] = v[i];
                v[i] = temp;
            }
        }
    }
}

fn bubble_sort(v: &mut Vec<usize>) {
    let n = v.len();
    for _ in 0..n {
        for j in 0..(n - 1) {
            if v[j + 1] < v[j] {
                let temp = v[j];
                v[j] = v[j + 1];
                v[j + 1] = temp;
            }
        }
    }
}

fn run_sort(numbers: &Vec<usize>, sort_name: &str, sort: Box<dyn Fn(&mut Vec<usize>) -> ()>) {
    let n = numbers.len();
    let mut xs = numbers.clone();
    let start = Instant::now();
    sort(&mut xs);
    let dt = Instant::now() - start;
    println!(
        "{sort_name:9} sort with {n} elements: {dt:.2?}, {:.2?} / element",
        dt / n as u32
    );
}

fn main() {
    const N: usize = 10_000;
    let numbers = generate_numbers(N);

    thread::scope(|s| {
        s.spawn(|| run_sort(&numbers, "Insertion", Box::new(insertion_sort)));

        s.spawn(|| run_sort(&numbers, "Selection", Box::new(selection_sort)));

        s.spawn(|| run_sort(&numbers, "Bubble", Box::new(bubble_sort)));
    });
}

#[cfg(test)]
mod tests {
    use crate::{bubble_sort, generate_numbers, insertion_sort, selection_sort};

    fn is_sorted(v: Vec<usize>) -> bool {
        let n = v.len();
        for i in 1..n {
            if v[i - 1] > v[i] {
                return false;
            }
        }
        true
    }

    #[test]
    fn check_sorted() {
        let sorted_numbers = vec![1, 2, 3, 4, 5];
        assert!(is_sorted(sorted_numbers));
    }

    #[test]
    fn check_unsorted() {
        let unsorted_numbers = vec![5, 2, 1, 4, 3];
        assert!(!is_sorted(unsorted_numbers));
    }

    #[test]
    fn selection_sort_works() {
        let mut numbers = generate_numbers(10);
        selection_sort(&mut numbers);
        assert!(is_sorted(numbers));
    }

    #[test]
    fn insertion_sort_works() {
        let mut numbers = generate_numbers(10);
        insertion_sort(&mut numbers);
        assert!(is_sorted(numbers));
    }

    #[test]
    fn bubble_sort_works() {
        let mut numbers = generate_numbers(10);
        bubble_sort(&mut numbers);
        assert!(is_sorted(numbers));
    }
}
