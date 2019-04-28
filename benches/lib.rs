#![feature(test)]

extern crate test;

extern crate apodize;

#[bench]
fn bench_cosine(bencher: &mut test::Bencher) {
    bencher.iter(|| apodize::cosine_at(0.355768, 0.487396, 0.144232, 0.012604, 1024, 400));
}

#[bench]
fn bench_collect_hanning_iter_1024(bencher: &mut test::Bencher) {
    bencher.iter(|| apodize::hanning_iter(1024).collect::<Vec<_>>());
}

#[bench]
fn bench_multiply_hanning_window_1024_with_data_as_vec_iter(bencher: &mut test::Bencher) {
    let window = apodize::hanning_iter(1024).collect::<Vec<_>>();
    let data = (0..1024).map(|x| x as f64).collect::<Vec<_>>();
    bencher.iter(|| {
        let mut copy = data.clone();
        for (dst, src) in copy.iter_mut().zip(window.iter()) {
            *dst *= *src;
        }
        copy
    });
}

#[bench]
fn bench_multiply_hanning_window_1024_with_data_as_vec_index(bencher: &mut test::Bencher) {
    let window = apodize::hanning_iter(1024).collect::<Vec<_>>();
    let data = (0..1024).map(|x| x as f64).collect::<Vec<_>>();
    bencher.iter(|| {
        let mut copy = data.clone();
        for i in 0..window.len() {
            copy[i] *= window[i];
        }
        copy
    });
}
