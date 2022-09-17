fn main() {
    let mut v = Vec::new(); // unknown type! make habit of declaring Type

    let v2 = vec![1, 2, 3]; // when using vec! macro, type annotation isn't necessary.

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v.push(9);

    let seven = &v[2];
    println!("The third element is {}", seven);

    let seven = v.get(2);
    match seven {
        Some(seven) => println!("The third element is {}", seven),
        None => println!("There's no third element"),
    }

    // let invalid_index = &v[1245]; // this would crash
    let invalid_index = v.get(1245); // this is fine

    /* Borrow Rule */
    let mut v2 = vec![1, 2, 3]; // when using vec! macro, type annotation isn't necessary.
    let first = &v2[0];
    // v2.push(6); // this would crash!
    /*
    While this looks like it should work, adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the elements next to each other where the vector is currently stored. In that case, the reference to the first element would be pointing to deallocated memory. The borrowing rules prevent programs from ending up in that situation.
    */

    /* Iteration */
    let mut v = vec![100, 32, 57];

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }

    /* Use Enum to store multiple types */
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn foo() {}
