use criterion::{black_box, criterion_group, criterion_main, Criterion};
use json_value_remove::Remove;
use serde_json::{from_str, Value};

fn remove(c: &mut Criterion) {
    let mut group = c.benchmark_group("Remove");
    let mut json_value1: Value = from_str(r#"{"my_table":["a","b","c"]}"#).unwrap();
    group.bench_function("remove_in_array", |b| {
        b.iter(|| black_box(json_value1.remove("/my_table/0")))
    });
    let mut json_value2: Value = from_str(r#"{"field1.0":{"field1.1":"value1.1","field1.2":"value1.2"},"field2.0":"value2.0"}"#).unwrap();
    group.bench_function("remove_in_object", |b| {
        b.iter(|| black_box(json_value2.remove("/field1.0/field1.2")))
    });
    group.finish();
}

criterion_group!(benches, remove);
criterion_main!(benches);
