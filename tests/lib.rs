#[macro_use]
extern crate apodize;

#[macro_use]
extern crate nalgebra;
use nalgebra::{ApproxEq, DVec};

const UNITS_IN_LAST_PLACE: u32 = 50;

#[test]
#[should_panic]
fn test_panic_too_short() {
    let _ = apodize::hanning_iter(1);
}

#[test]
fn test_hanning() {
    assert_approx_eq_ulps!(
       apodize::hanning_iter(2).collect::<DVec<_>>(),
       dvec![
           0.0,
           0.0,
       ],
       UNITS_IN_LAST_PLACE);

    assert_approx_eq_ulps!(
        apodize::hanning_iter(3).collect::<DVec<_>>(),
        dvec![
            0.0,
            1.0,
            0.0,
        ],
        UNITS_IN_LAST_PLACE);

    assert_approx_eq_ulps!(
        apodize::hanning_iter(10).collect::<DVec<_>>(),
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
        apodize::hanning_iter(11).collect::<DVec<_>>(),
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
        apodize::hamming_iter(2).collect::<DVec<_>>(),
        dvec![
            0.08000000000000002,
            0.08000000000000002
        ],
        UNITS_IN_LAST_PLACE);

    assert_approx_eq_ulps!(
        apodize::hamming_iter(3).collect::<DVec<_>>(),
        dvec![
            0.08000000000000002,
            1.0,
            0.08000000000000002
        ],
        UNITS_IN_LAST_PLACE);

    assert_approx_eq_ulps!(
        apodize::hamming_iter(10).collect::<DVec<_>>(),
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
        apodize::blackman_iter(2).collect::<DVec<_>>(),
        dvec![
            0.000060000000000004494,
            0.000060000000000004494
        ],
        UNITS_IN_LAST_PLACE);

    assert_approx_eq_ulps!(
        apodize::blackman_iter(3).collect::<DVec<_>>(),
        dvec![
            0.000060000000000004494,
            1.0,
            0.000060000000000004494
        ],
        UNITS_IN_LAST_PLACE);

    assert_approx_eq_ulps!(
        apodize::blackman_iter(10).collect::<DVec<_>>(),
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
    let epsilon = 0.000001;

    assert_approx_eq_eps!(
        apodize::nuttall_iter(2).map(|x| x as f32).collect::<DVec<f32>>(),
        dvec![0.0, 0.0],
        epsilon);

    assert_approx_eq_eps!(
        apodize::nuttall_iter(3).map(|x| x as f32).collect::<DVec<f32>>(),
        dvec![0.0, 1.0, 0.0],
        epsilon);

    assert_approx_eq_eps!(
        apodize::nuttall_iter(10).map(|x| x as f32).collect::<DVec<f32>>(),
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
        epsilon);
}

#[test]
fn test_triangular() {
    assert_approx_eq_ulps!(
        apodize::triangular_iter(1).collect::<DVec<_>>(),
        dvec![1.0],
        UNITS_IN_LAST_PLACE);

    assert_approx_eq_ulps!(
        apodize::triangular_iter(2).collect::<DVec<_>>(),
        dvec![0.5, 0.5],
        UNITS_IN_LAST_PLACE);

    assert_approx_eq_ulps!(
        apodize::triangular_iter(3).collect::<DVec<_>>(),
        dvec![0.3333333333333333, 1.0, 0.3333333333333333],
        UNITS_IN_LAST_PLACE);

    assert_approx_eq_ulps!(
        apodize::triangular_iter(10).collect::<DVec<_>>(),
        dvec![
            0.09999999999999998,
            0.30000000000000004,
            0.5,
            0.7,
            0.9,
            0.9,
            0.7,
            0.5,
            0.30000000000000004,
            0.09999999999999998
        ],
        UNITS_IN_LAST_PLACE);
    assert_approx_eq_ulps!(
        apodize::triangular_iter(11).collect::<DVec<_>>(),
        dvec![
            0.09090909090909094,
            0.2727272727272727,
            0.4545454545454546,
            0.6363636363636364,
            0.8181818181818181,
            1.,
            0.8181818181818181,
            0.6363636363636364,
            0.4545454545454546,
            0.2727272727272727,
            0.09090909090909094
        ],
        UNITS_IN_LAST_PLACE);
}
