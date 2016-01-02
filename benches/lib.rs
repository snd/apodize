#![feature(test)]

use std::ops::Mul;

extern crate test;

extern crate nalgebra;
use nalgebra::{DVec};

extern crate apodize;
use apodize::CanRepresentPi;

#[bench]
fn bench_cosine_at_f32(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        apodize::cosine_at::<f32>(
            0.355768,
            0.487396,
            0.144232,
            0.012604,
            1024,
            400
        )
    });
}

#[bench]
fn bench_cosine_at_f64(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        apodize::cosine_at::<f64>(
            0.355768,
            0.487396,
            0.144232,
            0.012604,
            1024,
            400
        )
    });
}

#[bench]
fn bench_collect_hanning_iter_1024_f32(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        apodize::hanning_iter(1024).collect::<Vec<f32>>()
    });
}

#[bench]
fn bench_collect_hanning_iter_1024_f64(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        apodize::hanning_iter(1024).collect::<Vec<f64>>()
    });
}

#[bench]
fn bench_multiply_hanning_window_1024_f32_with_data_as_dvec(bencher: &mut test::Bencher) {
    let window = apodize::hanning_iter(1024).collect::<DVec<f32>>();
    let data = (0..1024).map(|x| x as f32).collect::<DVec<f32>>();
    bencher.iter(|| {
        window.clone().mul(data.clone())
    });
}

#[bench]
fn bench_multiply_hanning_window_1024_f32_with_data_as_vec_iter(bencher: &mut test::Bencher) {
    let window = apodize::hanning_iter(1024).collect::<Vec<f32>>();
    let data = (0..1024).map(|x| x as f32).collect::<Vec<f32>>();
    bencher.iter(|| {
        let mut copy = data.clone();
        for (dst, src) in copy.iter_mut().zip(window.iter()) {
            *dst *= *src;
        }
        copy
    });
}

#[bench]
fn bench_multiply_hanning_window_1024_f32_with_data_as_vec_index(bencher: &mut test::Bencher) {
    let window = apodize::hanning_iter(1024).collect::<Vec<f32>>();
    let data = (0..1024).map(|x| x as f32).collect::<Vec<f32>>();
    bencher.iter(|| {
        let mut copy = data.clone();
        for i in 0..window.len() {
            copy[i] *= window[i];
        }
        copy
    });
}

#[bench]
fn bench_multiply_hanning_window_1024_f64_with_data_as_dvec(bencher: &mut test::Bencher) {
    let window = apodize::hanning_iter(1024).collect::<DVec<f64>>();
    let data = (0..1024).map(|x| x as f64).collect::<DVec<f64>>();
    bencher.iter(|| {
        window.clone().mul(data.clone())
    });
}

#[bench]
fn bench_multiply_hanning_window_1024_f64_with_data_as_vec_iter(bencher: &mut test::Bencher) {
    let window = apodize::hanning_iter(1024).collect::<Vec<f64>>();
    let data = (0..1024).map(|x| x as f64).collect::<Vec<f64>>();
    bencher.iter(|| {
        let mut copy = data.clone();
        for (dst, src) in copy.iter_mut().zip(window.iter()) {
            *dst *= *src;
        }
        copy
    });
}

#[bench]
fn bench_multiply_hanning_window_1024_f64_with_data_as_vec_index(bencher: &mut test::Bencher) {
    let window = apodize::hanning_iter(1024).collect::<Vec<f64>>();
    let data = (0..1024).map(|x| x as f64).collect::<Vec<f64>>();
    bencher.iter(|| {
        let mut copy = data.clone();
        for i in 0..window.len() {
            copy[i] *= window[i];
        }
        copy
    });
}

// this should take 0 ns/iter
#[bench]
fn bench_pi_f32_function_call_optimized_away(bencher: &mut test::Bencher) {
    bencher.iter(|| { f32::pi() });
}

// this should take 0 ns/iter
#[bench]
fn bench_pi_f64_function_call_optimized_away(bencher: &mut test::Bencher) {
    bencher.iter(|| { f64::pi() });
}
