use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    if first_list.len() > second_list.len() {
        return false;
    }
    if first_list == &second_list[..first_list.len()] {
        return true;
    }
    is_sublist(first_list, &second_list[1..])
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match first_list.len().cmp(&second_list.len()) {
        Ordering::Equal if first_list == second_list => Comparison::Equal,
        Ordering::Less if is_sublist(first_list, second_list) => Comparison::Sublist,
        Ordering::Greater if is_sublist(second_list, first_list) => Comparison::Superlist,
        _ => Comparison::Unequal
    }    
}
