use num::Num;


#[inline]
pub fn new<T: Copy + Num>(m11: T, m12: T, m21: T, m22: T) -> [T; 4] {[m11, m12, m21, m22]}
#[inline]
pub fn create<T: Copy + Num>(m11: T, m12: T, m21: T, m22: T) -> [T; 4] {new(m11, m12, m21, m22)}
#[test]
fn test_new() {
    let m = new(1, 0, 0, 1);
    assert!(m[0] == 1);
    assert!(m[1] == 0);
    assert!(m[2] == 0);
    assert!(m[3] == 1);
}

#[inline]
pub fn new_identity<T: Copy + Num>() -> [T; 4] {
    new(
        T::one(), T::zero(),
        T::zero(), T::one()
    )
}
#[inline]
pub fn new_zero<T: Copy + Num>() -> [T; 4] {
    new(
        T::zero(), T::zero(),
        T::zero(), T::zero()
    )
}

#[inline]
pub fn clone<'b, T: Copy + Num>(m: &'b [T; 4]) -> [T; 4] {new(m[0], m[1], m[2], m[3])}

#[inline]
pub fn copy<'a, 'b, T: Copy + Num>(out: &'a mut [T; 4], a: &'b [T; 4]) -> &'a mut [T; 4] {
    out[0] = a[0];
    out[1] = a[1];
    out[2] = a[2];
    out[3] = a[3];
    out
}
#[test]
fn test_copy() {
    let mut v = [0, 0, 0, 0];
    copy(&mut v, &[1, 2, 3, 4]);
    assert!(v == [1, 2, 3, 4]);
}
