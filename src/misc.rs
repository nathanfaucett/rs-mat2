use num::Num;
use approx::Approx;

use set::identity;


#[inline(always)]
pub fn inverse<'a, 'b, T: Num>(out: &'a mut [T; 4], a: &'b [T; 4]) -> &'a mut [T; 4] {
    let m11 = a[0];
    let m12 = a[2];
    let m21 = a[1];
    let m22 = a[3];

    let d = m11 * m22 - m12 * m21;
    if d != T::zero() {
        let inv_d = T::one() / d;

        out[0] = m22 * inv_d;
        out[1] = -m12 * inv_d;
        out[2] = -m21 * inv_d;
        out[3] = m11 * inv_d;
        out
    } else {
        identity(out)
    }
}
#[test]
fn test_inverse() {
    let mut v = [0, 0, 0, 0];
    inverse(&mut v, &[1, 0, 0, 1]);
    assert!(v == [1, 0, 0, 1]);
}

#[inline(always)]
pub fn determinant<'a, 'b, T: Num>(out: &'b [T; 4]) -> T {
    out[0] * out[3] - out[2] * out[1]
}
#[test]
fn test_determinant() {
    assert_eq!(determinant(&[1, 0, 0, 1]), 1);
}

#[inline(always)]
pub fn transpose<'a, 'b, T: Num>(out: &'a mut [T; 4], a: &'b [T; 4]) -> &'a mut [T; 4] {
    out[0] = a[0];
    out[1] = a[2];
    out[2] = a[1];
    out[3] = a[3];
    out
}
#[test]
fn test_transpose() {
    let mut v = [0, 0, 0, 0];
    transpose(&mut v, &[1, 0, 0, 1]);
    assert_eq!(v, [1, 0, 0, 1]);
}


#[inline(always)]
pub fn eq<'a, T: Num>(a: &'a [T; 4], b: &'a [T; 4]) -> bool {
    !nq(a, b)
}

#[inline(always)]
pub fn nq<'a, T: Num>(a: &'a [T; 4], b: &'a [T; 4]) -> bool {
    !a[0].approx_eq(b[0]) ||
    !a[1].approx_eq(b[1]) ||
    !a[2].approx_eq(b[2]) ||
    !a[3].approx_eq(b[3])
}
#[test]
fn test_nq() {
    assert_eq!(nq(&[1f32, 1f32, 1f32, 1f32], &[1f32, 1f32, 1f32, 1f32]), false);
    assert_eq!(nq(&[0f32, 0f32, 0f32, 0f32], &[1f32, 1f32, 1f32, 1f32]), true);
}
