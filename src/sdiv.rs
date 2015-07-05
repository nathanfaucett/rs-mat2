use num::Num;


#[inline(always)]
pub fn sdiv<T: Num>(out: &mut [T; 4], a: [T; 4], s: T) ->  &mut [T; 4] {
    let not_zero = s != T::zero();
    out[0] = if not_zero {a[0] / s} else  {T::zero()};
    out[1] = if not_zero {a[1] / s} else  {T::zero()};
    out[2] = if not_zero {a[2] / s} else  {T::zero()};
    out[3] = if not_zero {a[3] / s} else  {T::zero()};
    out
}
#[test]
fn test_sdiv() {
    let mut v = [0, 0, 0, 0];
    sdiv(&mut v, [1, 1, 1, 1], 1);
    assert!(v[0] == 1);
    assert!(v[1] == 1);
    assert!(v[2] == 1);
    assert!(v[3] == 1);
}
