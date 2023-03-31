mod tests;

/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    isbn.chars()
        .filter(|c| c != &'-')
        .enumerate()
        .try_fold((0, 0), |(count, sum), (index, char)| {
            match (index, char) {
                (_, '0'..='9') => Some((index, char.to_digit(10).unwrap())),
                (9, 'X') => Some((index, 10)),
                _ => None,
            }
            .and_then(|(index, digit)| Some((count + 1, sum + (digit * (10 - index as u32)))))
        })
        .map_or(false, |(count, sum)| count == 10 && sum % 11 == 0)
}
