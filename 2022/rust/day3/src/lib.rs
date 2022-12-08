use std::collections::HashSet;

pub trait ItemPriority {
    fn priority(&self) -> u8;
}

impl ItemPriority for char {
    fn priority(&self) -> u8 {
        if !self.is_ascii() {
            0
        } else {
            if self.is_uppercase() {
                (*self as u8) - 64 + 26
            } else {
                (*self as u8) - 96
            }
        }
    }
}

pub fn get_shared_item(val: &str) -> Option<char> {
    let (c1, c2) = val.split_at(val.len() / 2);

    let mut c1_contents = HashSet::new();
    for item in c1.chars() {
        c1_contents.insert(item);
    }

    let mut rc = None;
    for item in c2.chars() {
        if c1_contents.get(&item).is_some() {
            rc = Some(item);
            break;
        }
    }

    rc
}

pub fn get_badge_item(val: &[String]) -> Option<char> {
    if val.len() < 3 {
        return None;
    }

    let c1: HashSet<char> = val[0].chars().collect();
    let c2: HashSet<char> = val[1].chars().collect();

    for item in val[2].chars() {
        if c1.get(&item).is_some() && c2.get(&item).is_some() {
            return Some(item);
        }
    }

    None

}
