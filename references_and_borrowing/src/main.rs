fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}");

    let mut s = String::from("hello");
    change(&mut s);

    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // r1 and 2 will not be used beyond this point

    let r3 = &mut s; // no problem
    println!("{r3}");

    assert_eq!(1, 1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}