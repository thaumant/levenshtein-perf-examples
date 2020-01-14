pub fn levenshtein1(str1: &str, str2: &str) -> usize {
    if str1.is_empty() { return str2.chars().count(); }
    if str2.is_empty() { return str1.chars().count(); }
    
    let row: &mut Vec<usize> = {
        let start = 1;
        let end = str2.chars().count() + 1;
        &mut (start..end).collect()
    };

    for (i, ch1) in str1.chars().enumerate() {
        let mut dist_del = i + 1;
        let mut dist_sub = i;

        for (j, ch2) in str2.chars().enumerate() {
            dist_del = min!(
                row[j] + 1,
                dist_del + 1,
                dist_sub + (ch1 != ch2) as usize
            );
            dist_sub = row[j];
            row[j] = dist_del;
        }
    }

    row[row.len() - 1]
}