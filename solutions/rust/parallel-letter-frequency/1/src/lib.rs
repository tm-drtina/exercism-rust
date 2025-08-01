use std::collections::HashMap;
use std::thread;

fn frequency_worker(input: &[&str]) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for s in input {
        for char in s.chars().filter(|ch| ch.is_alphabetic()) {
            *counts.entry(char.to_ascii_lowercase()).or_default() += 1;
        }
    }
    counts
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    thread::scope(|s| {
        let handles = input
            .chunks((input.len() / worker_count).max(1))
            .map(|chunk| s.spawn(|| frequency_worker(chunk)))
            .collect::<Vec<_>>();

        handles.into_iter()
            .map(thread::ScopedJoinHandle::join)
            .map(Result::unwrap)
            .reduce(|mut a, b| {
                for (k, v) in b {
                    *a.entry(k).or_default() += v;
                }
                a
            })
            .unwrap_or_default()
    })
}
