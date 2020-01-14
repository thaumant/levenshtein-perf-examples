pub fn levenshtein0(str1: &str, str2: &str) -> usize {
    if str1.is_empty() { return str2.chars().count(); }
    if str2.is_empty() { return str1.chars().count(); }

    let len1 = str1.chars().count();
    let len2 = str2.chars().count();

    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
    for i in 0 .. len1 + 1 { matrix[i][0] = i; }
    for j in 0 .. len2 + 1 { matrix[0][j] = j; }

    for (i, ch1) in str1.chars().enumerate() {
        for (j, ch2) in str2.chars().enumerate() {
            let i = i + 1;
            let j = j + 1;
            matrix[i][j] = min!(
                matrix[i][j - 1] + 1,                        // insertion
                matrix[i - 1][j] + 1,                        // deletion
                matrix[i - 1][j - 1] + (ch1 != ch2) as usize // substitution
            );
        }
    }

    matrix[len1][len2]
}
