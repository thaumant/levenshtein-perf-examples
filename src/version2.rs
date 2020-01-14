use std::cell::RefCell;

static DEFAULT_CAPACITY: usize = 20;


pub struct Levenshtein {
    row: RefCell<Vec<usize>>,
}


impl Levenshtein {
    pub fn new() -> Levenshtein {
        Levenshtein {
            row: RefCell::new(Vec::with_capacity(DEFAULT_CAPACITY)),
        }
    }

    pub fn distance(&self, str1: &str, str2: &str) -> usize {
        if str1.is_empty() { return str2.chars().count(); }
        if str2.is_empty() { return str1.chars().count(); }

        let row = &mut *self.row.borrow_mut();
        store(row, 1 .. str2.chars().count() + 1);
    
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
}


fn store<T: Clone, Iter: Iterator<Item=T>>(buffer: &mut Vec<T>, iter: Iter) {
    buffer.clear();
    for val in iter {
        buffer.push(val);
    }
}


thread_local! {
    static LEVEN: Levenshtein = Levenshtein::new();
}


pub fn levenshtein2(str1: &str, str2: &str) -> usize {
    LEVEN.with(|leven| leven.distance(&str1, &str2))
}