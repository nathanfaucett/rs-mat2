use num::Num;


#[inline(always)]
pub fn set<'a, 'b, T: Num>(out: &'a mut [T; 4], m11: T, m12: T, m21: T, m22: T) -> &'a mut [T; 4] {
    out[0] = m11;
    out[1] = m21;
    out[2] = m12;
    out[3] = m22;
    out
}
#[test]
fn test_set() {
    let mut v = [0, 0, 0, 0];
    set(&mut v, 1, 2, 3, 4);
    assert!(v == [1, 3, 2, 4]);
}

#[inline(always)]
pub fn zero<'a, 'b, T: Num>(out: &'a mut [T; 4]) -> &'a mut [T; 4] {
    set(out, T::zero(), T::zero(), T::zero(), T::zero())
}
#[inline(always)]
pub fn identity<'a, 'b, T: Num>(out: &'a mut [T; 4]) -> &'a mut [T; 4] {
    set(out, T::one(), T::zero(), T::zero(), T::one())
}

#[inline(always)]
pub fn from_mat32<'a, 'b, T: Num>(out: &'a mut [T; 4], m: &'b [T; 6]) -> &'a mut [T; 4] {
    set(
        out,
        m[0], m[2],
        m[1], m[3]
    )
}
#[inline(always)]
pub fn from_mat3<'a, 'b, T: Num>(out: &'a mut [T; 4], m: &'b [T; 9]) -> &'a mut [T; 4] {
    set(
        out,
        m[0], m[3],
        m[1], m[4]
    )
}
#[inline(always)]
pub fn from_mat4<'a, 'b, T: Num>(out: &'a mut [T; 4], m: &'b [T; 16]) -> &'a mut [T; 4] {
    set(
        out,
        m[0], m[4],
        m[1], m[5]
    )
}
