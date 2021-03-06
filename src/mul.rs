use num::Num;


#[inline]
pub fn mul<'a, 'b, T: Copy + Num>(out: &'a mut [T; 4], a: &'b [T; 4], b: &'b [T; 4]) ->  &'a mut [T; 4] {
    let a11 = a[0];
    let a12 = a[2];
    let a21 = a[1];
    let a22 = a[3];

    let b11 = b[0];
    let b12 = b[2];
    let b21 = b[1];
    let b22 = b[3];

    out[0] = a11 * b11 + a21 * b12;
    out[1] = a12 * b11 + a22 * b12;
    out[2] = a11 * b21 + a21 * b22;
    out[3] = a12 * b21 + a22 * b22;
    out
}
#[test]
fn test_mul() {
    let mut v = [0, 0, 0, 0];
    mul(&mut v, &[1, 0, 0, 1], &[1, 0, 0, 1]);
    assert!(v[0] == 1);
    assert!(v[1] == 0);
    assert!(v[2] == 0);
    assert!(v[3] == 1);
}

#[inline]
pub fn smul<'a, 'b, T: Copy + Num>(out: &'a mut [T; 4], a: &'b [T; 4], s: T) ->  &'a mut [T; 4] {
    out[0] = a[0] * s;
    out[1] = a[1] * s;
    out[2] = a[2] * s;
    out[3] = a[3] * s;
    out
}
#[test]
fn test_smul() {
    let mut v = [0, 0, 0, 0];
    smul(&mut v, &[1, 1, 1, 1], 1);
    assert!(v[0] == 1);
    assert!(v[1] == 1);
    assert!(v[2] == 1);
    assert!(v[3] == 1);
}
