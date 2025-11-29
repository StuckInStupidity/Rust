#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // `&'static str` is a reference to a string allocated in read only memory
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book) {
    println!("I immutably borrowed {} - {} edition", book.title, book.year);
}

fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    //Mutable borrow closure
    let mut borrows_mutably = || list.push(7);
    borrows_mutably();
    println!("After calling closure: {:?}", list);

    //Immutable borrow closure
    let print_len = || println!("Length: {}", list.len());
    print_len();
    let sum = || list.iter().sum::<i32>();
    println!("Sum of numbers: {}", sum());

    //Move closure (takes ownership)
    let moved = move || {
        println!("Owned inside closure: {:?}", list);
    };
    moved();
    //println!("{:?}", list); // cannot be used anymore because moved

    /*error compilor:
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; //immutable borrow occurs here
    v.push(6); //mutable borrow occurs here
    println!("The first element is: {first}"); // immutable borrow later used here
    - You cannot have a mutable borrow while an immutable borrow is still active
    - The immutable borrow (first) is considered active until the last time it’s accessed in the code 
    (Rust checks everything).*/

    //Instead the following logic is correct:
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; //immutable borrow occurs here
    println!("The first element is: {first}"); //no other use of immutable borrow -> 'inactive'
    v.push(6); //mutable borrow occurs here (when no immutable borrows impacted are active)
    println!("{:?}", v);

    //Create an immutable Book named `immutabook`
    let immutabook = Book {
        // string literals have type `&'static str`
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };
    //Create a mutable copy of `immutabook` and call it `mutabook`
    let mut mutabook = immutabook;

    //Immutably borrow an immutable object
    borrow_book(&immutabook);
    //Immutably borrow a mutable object
    borrow_book(&mutabook);

    //Borrow a mutable object as mutable
    new_edition(&mut mutabook);
    //Error! Cannot borrow an immutable object as mutable
    //new_edition(&mut immutabook);


    let immutable_box = Box::new(5u32);
    println!("immutable_box contains {}", immutable_box);
    // Mutability error
    //*immutable_box = 4;

    //Move the box, changing the ownership (and mutability)
    let mut mutable_box = immutable_box;
    println!("mutable_box contains {}", mutable_box);
    // Modify the contents of the box
    *mutable_box = 4;
    println!("mutable_box now contains {}", mutable_box);
}
