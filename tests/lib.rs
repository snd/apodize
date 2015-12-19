extern crate apodize;
use apodize::{Float, blackman, hamming, hanning, nuttall};

#[macro_use]
extern crate nalgebra;
use nalgebra::{ApproxEq, DVec};

#[test]
#[should_panic]
fn test_panic_too_short() {
    let _ = hanning(1);
}

#[test]
fn test_hanning() {
    let expected: Vec<Float> = vec![
        0.0,
        0.0,
    ];
    assert_approx_eq_ulps!(DVec::from_slice(expected.len(),
                                            &expected[..]),
                           hanning(2).collect::<DVec<Float>>(),
                           10);

    let expected: Vec<Float> = vec![
        0.0,
        1.0,
        0.0,
    ];
    assert_approx_eq_ulps!(DVec::from_slice(expected.len(),
                                            &expected[..]),
                           hanning(3).collect::<DVec<Float>>(),
                           10);

    let expected: Vec<Float> = vec![
        0.0,
        0.11697777844051094,
        0.4131759111665348,
        0.7499999999999999,
        0.9698463103929542,
        0.9698463103929542,
        0.7499999999999999,
        0.4131759111665348,
        0.11697777844051094,
        0.0,
    ];
    assert_approx_eq_ulps!(DVec::from_slice(expected.len(),
                                            &expected[..]),
                           hanning(10).collect::<DVec<Float>>(),
                           10);
}

#[test]
fn test_hamming() {
    let expected: Vec<Float> = vec![0.08000000000000002,
                                    0.08000000000000002];
    assert_approx_eq_ulps!(DVec::from_slice(expected.len(),
                                            &expected[..]),
                           hamming(2).collect::<DVec<Float>>(),
                           10);

    let expected: Vec<Float> = vec![0.08000000000000002,
                                    1.0,
                                    0.08000000000000002];
    assert_approx_eq_ulps!(DVec::from_slice(expected.len(),
                                            &expected[..]),
                           hamming(3).collect::<DVec<Float>>(),
                           10);

    let expected: Vec<Float> = vec![0.08000000000000002,
                                    0.1876195561652701,
                                    0.46012183827321207,
                                    0.7699999999999999,
                                    0.9722586055615179,
                                    0.9722586055615179,
                                    0.7700000000000002,
                                    0.46012183827321224,
                                    0.1876195561652702,
                                    0.08000000000000002];
    assert_approx_eq_ulps!(DVec::from_slice(expected.len(),
                                            &expected[..]),
                           hamming(10).collect::<DVec<Float>>(),
                           10);
}

#[test]
fn test_blackman() {
    let expected: Vec<Float> = vec![0.000060000000000004494,
                                    0.000060000000000004494];
    assert_approx_eq_ulps!(DVec::from_slice(expected.len(),
                                            &expected[..]),
                           blackman(2).collect::<DVec<Float>>(),
                           10);

    let expected: Vec<Float> = vec![0.000060000000000004494,
                                    1.0,
                                    0.000060000000000004494];
    assert_approx_eq_ulps!(DVec::from_slice(expected.len(),
                                            &expected[..]),
                           blackman(3).collect::<DVec<Float>>(),
                           10);

    let expected: Vec<Float> = vec![0.000060000000000004494,
                                    0.015071173410218106,
                                    0.14703955786238146,
                                    0.5205749999999999,
                                    0.9316592687274005,
                                    0.9316592687274005,
                                    0.5205750000000003,
                                    0.1470395578623816,
                                    0.015071173410218144,
                                    0.000060000000000004494];
    assert_approx_eq_ulps!(DVec::from_slice(expected.len(),
                                            &expected[..]),
                           blackman(10).collect::<DVec<Float>>(),
                           10);
}

#[test]
fn test_nuttall() {
    let expected: Vec<Float> = vec![-0.000000000000000027755575615628914,
                                    -0.000000000000000027755575615628914];
    assert_approx_eq_ulps!(DVec::from_slice(expected.len(),
                                            &expected[..]),
                           nuttall(2).collect::<DVec<Float>>(),
                           10);

    let expected: Vec<Float> = vec![-0.000000000000000027755575615628914,
                                    1.0,
                                    -0.000000000000000027755575615628914];
    assert_approx_eq_ulps!(DVec::from_slice(expected.len(),
                                            &expected[..]),
                           nuttall(3).collect::<DVec<Float>>(),
                           10);

    let expected: Vec<Float> = vec![-0.000000000000000027755575615628914,
                                    0.013748626562839233,
                                    0.14190082671665563,
                                    0.5147459999999997,
                                    0.930560546720505,
                                    0.930560546720505,
                                    0.5147460000000003,
                                    0.14190082671665577,
                                    0.013748626562839275,
                                    -0.000000000000000027755575615628914];
    assert_approx_eq_ulps!(DVec::from_slice(expected.len(),
                                            &expected[..]),
                           nuttall(10).collect::<DVec<Float>>(),
                           10);
}
