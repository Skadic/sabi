use core::panic;
use std::{ops::{Add, Mul, Div, Sub}, process::Output};

use lzma_rs::lzma_compress;
use num::{cast::cast, Float, NumCast};
use num::One;

pub fn interpolate_linear<I, F>(start_pos: (I, I), end_pos: (I, I), lambda: F) -> (I, I)
where
    I: NumCast,
    F: Float,
{
    let lambda = clamp_float(lambda, F::zero(), F::one());

    let start_pos: (F, F) = cast_tuple(start_pos).unwrap();
    let end_pos: (F, F) = cast_tuple(end_pos).unwrap();

    let x = end_pos.0 * lambda + start_pos.0 * (F::one() - lambda);
    let y = end_pos.1 * lambda + start_pos.1 * (F::one() - lambda);

    (cast(x).unwrap(), cast(y).unwrap())
}

pub fn interpolate_bezier<I, F>(points: &[(I, I)], lambda: F) -> (I, I)
where
    I: NumCast + Copy,
    F: Float,
{
    if points.len() <= 1 {
        panic!("Not enough points given (minimum is 2)");
    }

    // Use De-Casteljau's Algorithm
    let lambda = clamp_float(lambda, F::zero(), F::one());

    let mut interpolated = points
        .iter()
        .copied()
        .map(cast_tuple::<I, F>)
        .map(Option::unwrap)
        .collect::<Vec<(F, F)>>();

    for _ in 1..points.len() {
        for i in 0..interpolated.len() - 1 {
            interpolated[i] = interpolate_linear(interpolated[i], interpolated[i + 1], lambda);
        }
        interpolated.pop();
    }

    cast_tuple(interpolated.pop().unwrap()).unwrap()
}

pub fn interpolate_centripetal_catmull<I, F>(
    p0: (I, I),
    p1: (I, I),
    p2: (I, I),
    p3: (I, I),
    lambda: F,
) -> (I, I)
where
    I: NumCast + Copy,
    F: Float,
{
    let p0 = cast_tuple::<I, F>(p0).unwrap();
    let p1 = cast_tuple::<I, F>(p1).unwrap();
    let p2 = cast_tuple::<I, F>(p2).unwrap();
    let p3 = cast_tuple::<I, F>(p3).unwrap();

    // Use Barry and Goldman's pyramidal formulation:

    // Knots
    let t0 = F::zero();
    let t1 = ((p1.0 - p0.0).powi(2) + (p1.1 - p0.1).powi(2))
        .sqrt()
        .sqrt()
        + t0;
    let t2 = ((p2.0 - p1.0).powi(2) + (p2.1 - p1.1).powi(2))
        .sqrt()
        .sqrt()
        + t1;
    let t3 = ((p3.0 - p2.0).powi(2) + (p3.1 - p2.1).powi(2))
        .sqrt()
        .sqrt()
        + t2;

    let a1 = tuple_add(
        tuple_mul_scalar(p0, (t1 - lambda) / (t1 - t0)),
        tuple_mul_scalar(p1, (lambda - t0) / (t1 - t0)),
    );
    let a2 = tuple_add(
        tuple_mul_scalar(p1, (t2 - lambda) / (t2 - t1)),
        tuple_mul_scalar(p2, (lambda - t1) / (t2 - t1)),
    );
    let a3 = tuple_add(
        tuple_mul_scalar(p2, (t3 - lambda) / (t3 - t2)),
        tuple_mul_scalar(p3, (lambda - t2) / (t3 - t2)),
    );

    let b1 = tuple_add(
        tuple_mul_scalar(a1, (t2 - lambda) / (t2 - t0)),
        tuple_mul_scalar(a2, (lambda - t0) / (t2 - t0)),
    );
    let b2 = tuple_add(
        tuple_mul_scalar(a2, (t3 - lambda) / (t3 - t1)),
        tuple_mul_scalar(a3, (lambda - t1) / (t3 - t1)),
    );

    let c = tuple_add(
        tuple_mul_scalar(b1, (t2 - lambda) / (t2 - t1)),
        tuple_mul_scalar(b2, (lambda - t1) / (t2 - t1)),
    );

    cast_tuple(c).unwrap()
}

pub fn interpolate_perfect_circle<I, F>(start: (I, I), end: (I, I), center: (I, I), radius: I, lambda: F) -> (I, I)
where
    I: NumCast + Copy,
    F: Float,
{
    //if points.len() <= 2 {
    //    panic!("Not enough points given (minimum is 3)");
    //}

    //if points.len() >= 4 {
    //    return interpolate_bezier(points, lambda);
    //}

    // Use De-Casteljau's Algorithm
    let lambda = clamp_float(lambda, F::zero(), F::one());
    let radius = cast::<I, F>(radius).unwrap();

    let start = cast_tuple::<I, F>(start).unwrap();
    let end = cast_tuple::<I, F>(end).unwrap();
    let center = cast_tuple::<I, F>(center).unwrap();

    let start_angle = F::atan2(start.1 - center.1, start.0 - center.0);
    let end_angle = F::atan2(end.1 - center.1, end.0 - center.0);

    let angle = end_angle * lambda + start_angle * (F::one() - lambda);

    let rel_y = F::sin(angle) * radius;
    let rel_x = F::cos(angle) * radius;
    
    cast_tuple((center.0 + rel_x, center.1 + rel_y)).unwrap()
}

fn clamp_float<F: Float>(float: F, min: F, max: F) -> F {
    if float < min {
        min
    } else if float > max {
        max
    } else {
        float
    }
}

fn cast_tuple<I: NumCast, O: NumCast>(tup: (I, I)) -> Option<(O, O)> {
    Some((cast::<I, O>(tup.0)?, cast::<I, O>(tup.1)?))
}

/// https://stackoverflow.com/questions/4103405/what-is-the-algorithm-for-finding-the-center-of-a-circle-from-three-points
pub fn find_circle<I: NumCast, O: NumCast>(p0: (I, I), p1: (I, I), p2: (I, I)) -> ((O, O), O) {

    let p0 = cast_tuple::<I, f64>(p0).unwrap();
    let p1 = cast_tuple::<I, f64>(p1).unwrap();
    let p2 = cast_tuple::<I, f64>(p2).unwrap();

    let y_delta_a = p1.1 - p0.1;
    let x_delta_a = p1.0 - p0.0;
    let y_delta_b = p2.1 - p1.1;
    let x_delta_b = p2.0 - p1.0;

    let slope_a = y_delta_a / x_delta_a;
    let slope_b = y_delta_b / x_delta_b;
    let center_x = (slope_a * slope_b * (p0.1 - p2.1) + slope_b * (p0.0 + p1.0) - slope_a * (p1.0 + p2.0)) / (2.0 * (slope_b - slope_a));
    let center_y = -1.0 * (center_x - (p0.0 + p1.0) / 2.0) / slope_a + (p0.1 + p1.1) / 2.0;

    let radius = ((p0.0 - center_x) * (p0.0 - center_x) + (p0.1 - center_y) * (p0.1 - center_y)).sqrt();
    (cast_tuple((center_x, center_y)).unwrap(), NumCast::from(radius).unwrap())
}


fn tuple_mul_scalar<N>((x, y): (N, N), scalar: N) -> (N, N)
where
    N: Mul<Output = N> + Copy,
{
    (x * scalar, y * scalar)
}

fn tuple_add_scalar<N>((x, y): (N, N), scalar: N) -> (N, N)
where
    N: Add<Output = N> + Copy,
{
    (x + scalar, y + scalar)
}

fn tuple_add<N: Add<Output = N>>((x, y): (N, N), (v, w): (N, N)) -> (N, N) {
    (x + v, y + w)
}
