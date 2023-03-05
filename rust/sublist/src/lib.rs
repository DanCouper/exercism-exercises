#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_sublist<T: PartialEq>(needle: &[T], haystack: &[T]) -> bool {
    if needle.len() == 0 {
        return true;
    }

    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match (first_list, second_list) {
        (fl, sl) if fl == sl => Comparison::Equal,
        (fl, sl) if is_sublist(fl, sl) => Comparison::Sublist,
        (fl, sl) if is_sublist(sl, fl) => Comparison::Superlist,
        _ => Comparison::Unequal,
    }
}
