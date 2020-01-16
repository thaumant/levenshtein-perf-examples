use std::cell::RefCell;

static DEFAULT_CAPACITY: usize = 20;


pub struct Levenshtein {
    row:    RefCell<Vec<usize>>,
    chars2: RefCell<Vec<char>>,
}


impl Levenshtein {
    pub fn new() -> Levenshtein {
        Levenshtein {
            row:    RefCell::new(Vec::with_capacity(DEFAULT_CAPACITY)),
            chars2: RefCell::new(Vec::with_capacity(DEFAULT_CAPACITY)),
        }
    }

    pub fn distance(&self, str1: &str, str2: &str) -> usize {
        if str1.is_empty() { return str2.chars().count(); }
        if str2.is_empty() { return str1.chars().count(); }

        let row    = &mut *self.row.borrow_mut();
        let chars2 = &mut *self.chars2.borrow_mut();

        store(chars2, str2.chars());
        store(row, 1 .. chars2.len() + 1);
    
        for (i, ch1) in str1.chars().enumerate() {
            let mut dist_del = i + 1;
            let mut dist_sub = i;
    
            for (&ch2, row_item) in chars2.iter().zip(row.iter_mut()) {
                dist_del = min!(
                    *row_item + 1,
                    dist_del + 1,
                    dist_sub + (ch1 != ch2) as usize
                );
                dist_sub = *row_item;
                *row_item = dist_del;
            }
        }
    
        row[row.len() - 1]
    }
}


fn store<T: Clone, Iter: Iterator<Item=T>>(buffer: &mut Vec<T>, iter: Iter) {
    buffer.clear();
    let mut cap = buffer.capacity();
    let mut i = 0;
    for item in iter {
        if i >= cap {
            buffer.reserve(1);
            cap = buffer.capacity();
        }
        unsafe { *buffer.get_unchecked_mut(i) = item; }
        i += 1;
    }
    unsafe { buffer.set_len(i); }
}


thread_local! {
    static LEVEN: Levenshtein = Levenshtein::new();
}


pub fn levenshtein5(str1: &str, str2: &str) -> usize {
    LEVEN.with(|leven| leven.distance(&str1, &str2))
}