use std::{
    thread,
    time::{Duration, Instant},
};

use sorts::{bubble_sort, insertion_sort, quick_sort, selection_sort};

use rand::seq::SliceRandom;

pub fn generate_numbers(n: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<usize> = (1..=n).collect();
    numbers.shuffle(&mut rng);
    numbers
}

fn run_sort(numbers: &Vec<usize>, sort: &Box<dyn Fn(&mut Vec<usize>) -> ()>) -> Duration {
    let mut xs = numbers.clone();
    let start = Instant::now();
    sort(&mut xs);
    Instant::now() - start
}

fn bench_sort(
    numbers: &Vec<usize>,
    sort_name: &str,
    sort: Box<dyn Fn(&mut Vec<usize>) -> ()>,
    samples: usize,
) {
    let mut avg_t = Duration::ZERO;
    let start = Instant::now();
    for _ in 0..samples {
        avg_t += run_sort(numbers, &sort);
    }
    let total_duration = Instant::now() - start;
    let avg_t = avg_t / samples.try_into().unwrap();
    println!(
        "{sort_name} sort ran in an average of {avg_t:.3?} ({:.3?} / element), {total_duration:.3?} total",
        avg_t / numbers.len().try_into().unwrap()
    );
}

fn main() {
    const N: usize = 10_000;
    const SAMPLES: usize = 500;
    let numbers = generate_numbers(N);

    println!("Running benchmarks with {SAMPLES} samples and {N} elements...");
    thread::scope(|s| {
        s.spawn(|| bench_sort(&numbers, "Insertion", Box::new(insertion_sort), SAMPLES));

        s.spawn(|| bench_sort(&numbers, "Selection", Box::new(selection_sort), SAMPLES));

        s.spawn(|| bench_sort(&numbers, "Bubble", Box::new(bubble_sort), SAMPLES));

        s.spawn(|| bench_sort(&numbers, "Quick", Box::new(quick_sort), SAMPLES));
    });
}

#[cfg(test)]
mod tests {
    use sorts::{partition_at, quick_sort};

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

    #[test]
    fn partition_works() {
        let numbers = vec![1, 4, 3, 2, 5];
        let (left, right) = partition_at(&numbers, 3);
        assert_eq!(left, vec![1, 2]);
        assert_eq!(right, vec![4, 5]);
    }

    #[test]
    fn quick_sort_works() {
        let mut numbers = vec![1, 4, 3, 2, 5];
        quick_sort(&mut numbers);
        assert!(is_sorted(numbers));
    }
}
