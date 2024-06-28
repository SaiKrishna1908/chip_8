
/// Decomposes a u16 type to two u8 types
/// index 0 contains the first 8 bits of the u16
/// index 1 contains the last 8 bits of the u16
pub fn decompose(code: u16) -> (u8, u8) {
    let left_code = ((code &  0xff00) >> 8) as u8;
    let right_code  = (code & 0xff) as u8;
    return (left_code, right_code);
}