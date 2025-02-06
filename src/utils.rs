use anyhow::Result;

pub fn read_string_from_bytes(bytes: Vec<u8>) -> Result<String> {
    let output = bytes.split(|&c| c == 0).next().unwrap_or_default().to_vec();
    let s = String::from_utf8(output).unwrap_or_default();

    Ok(s)
}
