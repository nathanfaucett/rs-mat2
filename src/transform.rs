use num::Num;


#[inline(always)]
pub fn set_rotation<'a, 'b, T: Num>(out: &'a mut [T; 4], angle: T) -> &'a mut [T; 4] {
    let c = angle.cos();
    let s = angle.sin();

    out[0] = c;
    out[1] = s;
    out[2] = -s;
    out[3] = c;
    out
}

#[inline(always)]
pub fn get_rotation<'a, 'b, T: Num>(out: &'b [T; 4]) -> T {
    out[1].atan2(out[0])
}

#[inline(always)]
pub fn rotate<'a, 'b, T: Num>(out: &'a mut [T; 4], a: &'b [T; 4], angle: T) -> &'a mut [T; 4] {
    let m11 = a[0];
    let m12 = a[2];
    let m21 = a[1];
    let m22 = a[3];
    let c = angle.cos();
    let s = angle.sin();

    out[0] = m11 * c + m12 * s;
    out[1] = m11 * -s + m12 * c;
    out[2] = m21 * c + m22 * s;
    out[3] = m21 * -s + m22 * c;
    out
}
