
pub(crate) fn bit_from_u16_as_u11(input: u16, position: u16) -> bool {
    if position < 11 {
        input & (1 << (10 - position)) != 0
    } else {
        false
    }
}
