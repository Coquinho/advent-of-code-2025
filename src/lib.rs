pub fn read_from_str_one_liner<T: std::str::FromStr>(input: &str, split_value: char) -> Vec<T> {
    input
        .split(split_value)
        .filter_map(|line| line.trim().parse().ok())
        .collect()
}

pub fn read_from_str<T: std::str::FromStr>(input: &str) -> Vec<T> {
    input
        .lines()
        .filter_map(|line| line.trim().parse().ok())
        .collect()
}

pub fn read_from_file<T: std::str::FromStr, P: AsRef<std::path::Path>>(
    file_path: P,
) -> std::io::Result<Vec<T>> {
    let input = std::fs::read_to_string(file_path)?;
    Ok(read_from_str(&input))
}
