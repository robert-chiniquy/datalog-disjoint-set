use criterion::{black_box, criterion_group, criterion_main, Criterion, SamplingMode};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("crush");
    group.sampling_mode(SamplingMode::Flat);
    group.sample_size(10);
    group.nresamples(1000);
    group.confidence_level(0.6);
    group.warm_up_time(std::time::Duration::from_millis(100));

    group.bench_function("for_loop 2000 -> ".to_string(), |b| {
        b.iter(|| black_box(datalog_disjoint_set::for_loop::run(2000)))
    });
    group.bench_function("for_loop 20000 -> ".to_string(), |b| {
        b.iter(|| black_box(datalog_disjoint_set::for_loop::run(20000)))
    });

    group.bench_function("half 2000 -> ".to_string(), |b| {
        b.iter(|| black_box(datalog_disjoint_set::half_datalog::run(2000)))
    });
    group.bench_function("half 20000 -> ".to_string(), |b| {
        b.iter(|| black_box(datalog_disjoint_set::half_datalog::run(20000)))
    });

    group.bench_function("half and half 2000 -> ".to_string(), |b| {
        b.iter(|| black_box(datalog_disjoint_set::half_and_half::run(2000)))
    });
    group.bench_function("half and half 20000 -> ".to_string(), |b| {
        b.iter(|| black_box(datalog_disjoint_set::half_and_half::run(20000)))
    });

    group.bench_function("all 2000 -> ".to_string(), |b| {
        b.iter(|| black_box(datalog_disjoint_set::all_datalog::run(2000)))
    });
    group.bench_function("all 20000 -> ".to_string(), |b| {
        b.iter(|| black_box(datalog_disjoint_set::all_datalog::run(20000)))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
