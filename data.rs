use std::io;
use std::collections::HashMap;

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("The tuple is: {five_hundred} {six_point_four} {one}");

    let numbers = [10, 20, 30, 40, 50];
    let slice = &numbers[1..4]; // includes indices 1,2,3 -> [20, 30, 40]
    println!("Original array: {:?}", numbers);
    println!("Slice of array: {:?}", slice);

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    let el: &i32 = &v[2];
    println!("The nth element is {el}");
    let nth: Option<&i32> = v.get(2);
    match nth {
        Some(nth) => println!("The nth element is {nth}"),
        None => println!("There is no nth element."),
    }

    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let safe_index = index % a.len();
    let element = a[safe_index];
    println!("The value of the element at index {safe_index} is : {element}");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
