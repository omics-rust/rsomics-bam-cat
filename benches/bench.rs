use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::path::PathBuf;
use std::process::Command;

fn bench_bam_cat(c: &mut Criterion) {
    let bin = env!("CARGO_BIN_EXE_rsomics-bam-cat");
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let a = manifest.join("tests/golden/part_a.sam");
    let b = manifest.join("tests/golden/part_b.sam");
    c.bench_function("rsomics-bam-cat golden", |b_| {
        b_.iter(|| {
            let out = Command::new(black_box(bin))
                .args([a.to_str().unwrap(), b.to_str().unwrap(), "-o", "/dev/null"])
                .output()
                .unwrap();
            assert!(out.status.success());
        });
    });
}

criterion_group!(benches, bench_bam_cat);
criterion_main!(benches);
