use criterion::{black_box, criterion_group, criterion_main, Criterion, PlotConfiguration};
use runner::day;
use shared::{day_name, Solution};

extern crate shared;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Total: All Days", |b| {
        b.iter(|| {
            for i in 1..=49 {
                let (function, input, _) = day(i);
                function(black_box(input));
            }
        })
    });

    let mut group = c.benchmark_group("Individual");
    group
        .plot_config(PlotConfiguration::default().summary_scale(criterion::AxisScale::Logarithmic));

    for i in 1..=49 {
        let (function, input, name) = day(i);
        if let Solution::None = function(input) {
        } else {
            let title = day_name((i + 1) / 2);
            group.bench_function(&format!("{}: {}", name, title), |b| {
                b.iter(|| function(black_box(input)))
            });
        }
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
