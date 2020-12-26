use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::collections::HashMap;

fn play_until_hashmap(input: &[u8], stop_at: u32) -> u32 {
    let mut nums: HashMap<u32, u32> = HashMap::new();
    let mut round = 0;
    let mut last_num: u32 = 0;
    for num in input.iter() {
        round += 1;
        nums.insert(*num as u32, round);
        last_num = *num as u32;
    }
    loop {
        round += 1;

        let num_to_say = match nums.get(&last_num) {
            Some(&last) => round - 1 - last,
            None => 0,
        };

        nums.insert(last_num, round - 1);

        last_num = num_to_say;
        if round >= stop_at {
            return last_num;
        }
    }
}

fn play_until_vec(input: &[u8], stop_at: u32) -> u32 {
    let mut nums: Vec<(u32, u32)> = Vec::new();
    let mut round = 0;
    let mut last_num: u32 = 0;
    for num in input.iter() {
        round += 1;
        nums.retain(|(i, _)| *i != *num as u32);
        nums.push((*num as u32, round));
        last_num = *num as u32;
    }
    loop {
        round += 1;

        let num_to_say = match nums.iter().find(|(i, _)| *i == last_num) {
            Some((_, last)) => round - 1 - *last,
            None => 0,
        };

        nums.push((last_num, round - 1));

        last_num = num_to_say;
        if round >= stop_at {
            return last_num;
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day15.2");
    for &input in [2020u32, 10_000, 100_000, 1_000_000 /*, 30_000_000*/].iter() {
        group.throughput(criterion::Throughput::Elements(input.into()));

        group.bench_with_input(BenchmarkId::new("HashMap", input), &input, |b, &i| {
            b.iter(|| play_until_hashmap(&[9, 19, 1, 6, 0, 5, 4], i))
        });

        group.bench_with_input(BenchmarkId::new("Vec", input), &input, |b, &i| {
            b.iter(|| play_until_vec(&[9, 19, 1, 6, 0, 5, 4], i))
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
