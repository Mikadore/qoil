mod reference;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use std::fs::DirEntry;

fn for_dir_entry<F>(c: &mut Criterion, run: F)
where
    F: Fn(DirEntry, &mut Criterion),
{
    let dir = std::fs::read_dir("./benches/images").expect("Couldn't read the images directory");
    for file in dir {
        if let Ok(file) = file {
            assert!(file.path().extension().unwrap() == "qoi");
            run(file, c);
        }
    }
}

fn compare_decode_impls(c: &mut Criterion) {
    for_dir_entry(c, |file, c| {
        let image = std::fs::read(file.path()).expect("Couldn't read image");

        let path = file.path();
        let filename = path.file_name().unwrap().to_str().unwrap();
        let mut group = c.benchmark_group(filename);

        group.bench_with_input(BenchmarkId::new("qoi", ""), image.as_slice(), |b, input| {
            let header = qoi::decode_header(input).unwrap();
            let bytes_per_pixel = header.channels.as_u8() as usize;
            let size = (header.width as usize * header.height as usize) * bytes_per_pixel;
            let mut out_buf = vec![0; size];
            b.iter(|| qoi::decode_to_buf(out_buf.as_mut_slice(), input).unwrap());
        });

        group.bench_with_input(
            BenchmarkId::new("qoi-reference", ""),
            image.as_slice(),
            |b, input| {
                let channels = qoi::decode_header(input).unwrap().channels.as_u8();
                b.iter(|| reference::decode(input, channels))
            },
        );

        group.finish()
    });
}

fn compare_encode_impls(c: &mut Criterion) {
    for_dir_entry(c, |file, c| {
        let image = std::fs::read(file.path()).expect("Couldn't read image");

        let path = file.path();
        let filename = path.file_name().unwrap().to_str().unwrap();
        let mut group = c.benchmark_group(filename);

        let (header, raw) = qoi::decode_to_vec(&image).unwrap();

        group.bench_with_input(
            BenchmarkId::new("qoi", ""),
            &(raw.as_slice(), header.clone()),
            |b, (data, header)| {
                let mut buf = vec![0; image.len()];
                b.iter(|| qoi::encode_to_buf(&mut buf, data, header.width, header.height))
            },
        );

        group.bench_with_input(
            BenchmarkId::new("qoi-reference", ""),
            &(raw.as_slice(), header.clone()),
            |b, (data, header)| {
                b.iter(||reference::encode(data, *header))
            },
        );
    })
}

criterion_group!(benches, compare_decode_impls, compare_encode_impls);
criterion_main!(benches);
