use std::collections::{VecDeque, HashSet};

fn check_unique(data: &[char]) -> bool {
    let mut unq = HashSet::new();
    for i in data {
        if unq.get(&i).is_some() {
            return false;
        }
        unq.insert(i);
    }

    true
}

pub fn get_marker_position(data: &str, amt: usize) -> usize {
    let mut marker_check = VecDeque::new();
    for (i, c) in data.chars().enumerate() {
        marker_check.push_back(c);
        if marker_check.len() < amt {
            continue;
        } else if marker_check.len() > amt {
            marker_check.pop_front();
        }

        if check_unique(&marker_check.make_contiguous()) {
            return i + 1;
        }
    }

    0
}
