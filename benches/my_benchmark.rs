use criterion::{black_box, criterion_group, criterion_main, Criterion};
use orst::bubblesort::BubbleSort;
use orst::insertionsort::InsertionSort;
use orst::Sorter;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn run<F>(upper_limit: u32, sort: F)
where
    F: Fn(&mut Vec<u32>),
{
    let mut vec: Vec<u32> = (0..upper_limit).collect();
    vec.shuffle(&mut thread_rng());
    sort(&mut vec);
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let upper_limit = black_box(10000);
    c.bench_function("bubblesort", |b| {
        b.iter(|| run(upper_limit, |v| BubbleSort.sort(v)))
    });

    c.bench_function("insertionsort_non_binary", |b| {
        b.iter(|| {
            run(upper_limit, |v| {
                let sorter = InsertionSort::new(false);
                sorter.sort(v)
            })
        })
    });

    c.bench_function("insertionsort_binary", |b| {
        b.iter(|| {
            run(upper_limit, |v| {
                let sorter = InsertionSort::new(true);
                sorter.sort(v)
            })
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
