use num::Num;


#[inline(always)]
pub fn create<T: Num>(m11: T, m12: T, m21: T, m22: T) -> [T; 4] {[m11, m12, m21, m22]}
#[test]
fn test_create() {
    let m = create(1, 0, 0, 1);
    assert!(m[0] == 1);
    assert!(m[1] == 0);
    assert!(m[2] == 0);
    assert!(m[3] == 1);
}

#[inline(always)]
pub fn clone<T: Num>(m: [T; 4]) -> [T; 4] {create(m[0], m[1], m[2], m[3])}

#[inline(always)]
pub fn copy<T: Num>(out: &mut [T; 4], a: [T; 4]) -> &mut [T; 4] {
    out[0] = a[0];
    out[1] = a[1];
    out[2] = a[2];
    out[3] = a[3];
    out
}
#[test]
fn test_copy() {
    let mut v = [0, 0, 0, 0];
    copy(&mut v, [1, 2, 3, 4]);
    assert!(v == [1, 2, 3, 4]);
}
