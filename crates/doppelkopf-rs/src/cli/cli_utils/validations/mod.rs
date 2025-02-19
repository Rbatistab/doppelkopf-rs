pub fn valid_pack_size(s: &str) -> Result<u8, String> {
    let invalid_pack_size_message = "Pack size can only be '40' or '48'.";
    let size: u8 = s.parse().map_err(|_| invalid_pack_size_message)?;
    if size == 40 || size == 48 {
        Ok(size)
    } else {
        Err(String::from(invalid_pack_size_message))
    }
}
