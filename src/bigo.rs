
fn sum_char_codes(input: String) -> u32 {
    let mut i = 0;
    for char in input.chars() {
        i += char as u32;
    }
    i
}
