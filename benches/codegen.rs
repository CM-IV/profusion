use profusion::{conn::profusion_conn, CodegenSettings};
use criterion::Criterion;

fn bench(c: &mut Criterion) {
    profusion::container::cleanup(false).ok();
    profusion::container::setup(false).unwrap();
    let client = &mut profusion_conn().unwrap();

    profusion::load_schema(client, &["../codegen_test/schema.sql"]).unwrap();
    c.bench_function("codegen_sync", |b| {
        b.iter(|| {
            profusion::generate_live(
                client,
                "../test_codegen/queries",
                None,
                CodegenSettings {
                    gen_sync: true,
                    gen_async: false,
                    derive_ser: true,
                },
            )
            .unwrap()
        })
    });
    c.bench_function("codegen_async", |b| {
        b.iter(|| {
            profusion::generate_live(
                client,
                "../test_codegen/queries",
                None,
                CodegenSettings {
                    gen_sync: true,
                    gen_async: false,
                    derive_ser: true,
                },
            )
            .unwrap()
        })
    });
    profusion::container::cleanup(false).unwrap();
}
criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
