use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_benchmarks::{generate_random_special_char_string, remove_special_chars_replace, remove_special_chars_with_capacity, remove_special_chars_without_capacity};
use rand_pcg::Pcg64;
use rand::SeedableRng;

pub fn bench(c: &mut Criterion) {
    let mut rng = Pcg64::seed_from_u64(0);
    let input10k = generate_random_special_char_string(&mut rng,10_000);
    let input100k = generate_random_special_char_string(&mut rng,100_000);
    let input1m = generate_random_special_char_string(&mut rng,1_000_000);
    let input10m = generate_random_special_char_string(&mut rng,10_000_000);

    let mut group = c.benchmark_group("remove_special_chars");
    group.bench_function("replace10k", |b| b.iter(|| remove_special_chars_replace(black_box(&input10k))));
    group.bench_function("replace100k", |b| b.iter(|| remove_special_chars_replace(black_box(&input100k))));
    group.bench_function("replace1m", |b| b.iter(|| remove_special_chars_replace(black_box(&input1m))));
    group.bench_function("replace10m", |b| b.iter(|| remove_special_chars_replace(black_box(&input10m))));

    group.bench_function("without_capacity10k", |b| b.iter(|| remove_special_chars_without_capacity(black_box(&input10k))));
    group.bench_function("without_capacity100k", |b| b.iter(|| remove_special_chars_without_capacity(black_box(&input100k))));
    group.bench_function("without_capacity1m", |b| b.iter(|| remove_special_chars_without_capacity(black_box(&input1m))));
    group.bench_function("without_capacity10m", |b| b.iter(|| remove_special_chars_without_capacity(black_box(&input10m))));

    group.bench_function("with_capacity10k", |b| b.iter(|| remove_special_chars_with_capacity(black_box(&input10k))));
    group.bench_function("with_capacity100k", |b| b.iter(|| remove_special_chars_with_capacity(black_box(&input100k))));
    group.bench_function("with_capacity1m", |b| b.iter(|| remove_special_chars_with_capacity(black_box(&input1m))));
    group.bench_function("with_capacity10m", |b| b.iter(|| remove_special_chars_with_capacity(black_box(&input10m))));
}

criterion_group!(benches, bench);
criterion_main!(benches);