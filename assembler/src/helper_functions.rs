pub fn remove_commas(input: &str) -> &str {
    input.trim_end_matches(',')
}

pub fn decimal_to_12bit_binary(decimal: u16) -> String {
    format!("{:012b}", decimal)
}
