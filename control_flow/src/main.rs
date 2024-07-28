fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;
    // Loop labels must begin with a single quote
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("reamining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count: {count}");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("the value is: {element}");
    }

    // Rev: reverses the iterator's direction
    for number in (1..4).rev() {
        println!("{number}");
    }
}
