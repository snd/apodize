#[macro_use]
extern crate apodize;
use apodize::{blackman_iter, hamming_iter, hanning_iter, nuttall_iter};

#[macro_use]
extern crate nalgebra;
use nalgebra::{ApproxEq, DVec};

const UNITS_IN_LAST_PLACE: u32 = 20;

#[test]
#[should_panic]
fn test_panic_too_short() {
    let _ = hanning_iter::<f64>(1);
}

#[test]
fn test_hanning() {
    assert_approx_eq_ulps!(
       hanning_iter(2).collect::<DVec<f64>>(),
       dvec![
           0.0,
           0.0,
       ],
       UNITS_IN_LAST_PLACE);

    assert_approx_eq_ulps!(
        hanning_iter(3).collect::<DVec<f64>>(),
        dvec![
            0.0,
            1.0,
            0.0,
        ],
        UNITS_IN_LAST_PLACE);

    assert_approx_eq_ulps!(
        hanning_iter(10).collect::<DVec<f64>>(),
        dvec![
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
        ],
        UNITS_IN_LAST_PLACE);

    assert_approx_eq_ulps!(
        hanning_iter(11).collect::<DVec<f64>>(),
        dvec![
            0.0,
            0.09549150281252627,
            0.3454915028125263,
            0.6545084971874737,
            0.9045084971874737,
            1.0,
            0.9045084971874737,
            0.6545084971874737,
            0.3454915028125264,
            0.09549150281252633,
            0.0
        ],
        UNITS_IN_LAST_PLACE);
}

#[test]
fn test_hamming() {
    assert_approx_eq_ulps!(
        hamming_iter(2).collect::<DVec<f64>>(),
        dvec![
            0.08000000000000002,
            0.08000000000000002
        ],
        UNITS_IN_LAST_PLACE);

    assert_approx_eq_ulps!(
        hamming_iter(3).collect::<DVec<f64>>(),
        dvec![
            0.08000000000000002,
            1.0,
            0.08000000000000002
        ],
        UNITS_IN_LAST_PLACE);

    assert_approx_eq_ulps!(
        hamming_iter(10).collect::<DVec<f64>>(),
        dvec![
            0.08000000000000002,
            0.1876195561652701,
            0.46012183827321207,
            0.7699999999999999,
            0.9722586055615179,
            0.9722586055615179,
            0.7700000000000002,
            0.46012183827321224,
            0.1876195561652702,
            0.08000000000000002
        ],
        UNITS_IN_LAST_PLACE);
}

#[test]
fn test_blackman() {
    assert_approx_eq_ulps!(
        blackman_iter(2).collect::<DVec<f64>>(),
        dvec![
            0.000060000000000004494,
            0.000060000000000004494
        ],
        UNITS_IN_LAST_PLACE);

    assert_approx_eq_ulps!(
        blackman_iter(3).collect::<DVec<f64>>(),
        dvec![
            0.000060000000000004494,
            1.0,
            0.000060000000000004494
        ],
        UNITS_IN_LAST_PLACE);

    assert_approx_eq_ulps!(
        blackman_iter(10).collect::<DVec<f64>>(),
        dvec![
            0.000060000000000004494,
            0.015071173410218106,
            0.14703955786238146,
            0.5205749999999999,
            0.9316592687274005,
            0.9316592687274005,
            0.5205750000000003,
            0.1470395578623816,
            0.015071173410218144,
            0.000060000000000004494
        ],
        UNITS_IN_LAST_PLACE);
}

#[test]
fn test_nuttall() {
    assert_approx_eq_ulps!(
        nuttall_iter(2).collect::<DVec<f32>>(),
        dvec![0.0, 0.0],
        UNITS_IN_LAST_PLACE);

    assert_approx_eq_ulps!(
        nuttall_iter(3).collect::<DVec<f32>>(),
        dvec![0.0, 1.0, 0.0],
        UNITS_IN_LAST_PLACE);

    assert_approx_eq_ulps!(
        nuttall_iter(10).collect::<DVec<f32>>(),
        dvec![
            0.0,
            0.013748631,
            0.14190082,
            0.51474607,
            0.9305606,
            0.93056047,
            0.51474595,
            0.14190066,
            0.013748631,
            0.0
        ],
        UNITS_IN_LAST_PLACE);
}
