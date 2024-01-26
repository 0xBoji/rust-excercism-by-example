#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }
    if is_subsequence(first_list, second_list) {
        return Comparison::Sublist;
    }
    if is_subsequence(second_list, first_list) {
        return Comparison::Superlist;
    }
    Comparison::Unequal
}

fn is_subsequence<T: PartialEq>(short_list: &[T], long_list: &[T]) -> bool {
    if short_list.is_empty() {
        return true;
    }
    if short_list.len() > long_list.len() {
        return false;
    }

    for window in long_list.windows(short_list.len()) {
        if window == short_list {
            return true;
        }
    }

    false
}


pub fn run() {
    let list_a: Vec<i32> = vec![1, 2, 3];
    let list_b: Vec<i32> = vec![];

    println!("List A: {:?}", list_a);
    println!("List B: {:?}", list_b);
    match sublist(&list_a, &list_b) {
        Comparison::Equal => println!("List A and List B are equal"),
        Comparison::Sublist => println!("List A is a sublist of List B"),
        Comparison::Superlist => println!("List A is a superlist of List B"),
        Comparison::Unequal => println!("List A and List B are unequal"),
    }

    let list_c: Vec<i32> = vec![2, 3];
    let list_d: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("\nList C: {:?}", list_c);
    println!("List D: {:?}", list_d);
    match sublist(&list_c, &list_d) {
        Comparison::Equal => println!("List C and List D are equal"),
        Comparison::Sublist => println!("List C is a sublist of List D"),
        Comparison::Superlist => println!("List C is a superlist of List D"),
        Comparison::Unequal => println!("List C and List D are unequal"),
    }
}
