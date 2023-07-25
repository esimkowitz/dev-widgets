/// Adds a delimiter to a number string at a given frequency.
pub fn add_number_delimiters(number_str: String, delimiter: char, frequency: usize) -> String {
    number_str
        .chars()
        .rev()
        .enumerate()
        .flat_map(|(i, c)| {
            if i != 0 && i % frequency == 0 {
                Some(delimiter)
            } else {
                None
            }
            .into_iter()
            .chain(std::iter::once(c))
        })
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>()
}

/// Removes all non-alphanumeric characters from a string.
pub fn sanitize_string(string: String) -> String {
    string
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .collect::<String>()
}