fn main() {
    variables_and_mutability();
    data_types();
}

fn variables_and_mutability() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    const THREE_HOURS_IN_SECONDS: i32 = 10800; // 60 * 60 * 3;
                                               // Does not compile. Check Constant Evaluation
                                               // https://doc.rust-lang.org/reference/const_eval.html
    println!("Three hours is {THREE_HOURS_IN_SECONDS} seconds");

    // Shadowing
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in inner scope is: {x}"); // 12
    }
    println!("The value of x is {x}"); // 6

    /*
    The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name. For example, say our program asks a user to show how many spaces they want between some text by inputting space characters, and then we want to store that input as a number.
    */

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The space is {spaces}");

    // let mut spaces = "   ";
    // spaces = spaces.len(); // compile error
}

fn data_types() {
    /* Scalar Types
    integers
        - i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
    floating-point numbers
    Booleans,
    characters
     */
    integer_overflowing();
    numeric_operations();

    /* Compound Types */
    tuple_and_array();

    /* Functions */
    print_labeled_measurement(16, 'd');
    statement_and_expression();
    control_flow();
}

fn integer_overflowing() {
    /* Preventing integer overflowing */
    let first_integer: u8 = 210;
    let second_integer: u8 = 125;
    // let result = first_integer + second_integer; // compile error due to overflow
    let result = first_integer.wrapping_add(second_integer);
    println!("wrapping_add: {result}");
    let result = first_integer.checked_add(second_integer);
    let result_1 = result.is_none();
    let result_2 = result.is_some();
    println!("checked_add: {result_1}, {result_2}");
    let result = first_integer.overflowing_add(second_integer);
    let result_1 = result.0;
    let result_2 = result.1;
    println!("overflowing_add: {result_1}, {result_2}");
    let result = first_integer.saturating_add(second_integer);
    println!("saturating_add: {result}");
}

fn numeric_operations() {
    /* Numeric operations */
    // addition
    let sum = 5 + 10;
    println!(" 5 + 10 is {sum}");
    // subtraction
    let difference = 95.5 - 4.3;
    println!(" 95.5 - 4.3 is {difference}");
    // multiplication
    let product = 4 * 30;
    println!(" 4 * 30 is {product}");
    // division
    let quotient = 56.7 / 32.2;
    println!(" 56.7 / 32.2 is {quotient}");
    let floored = 2 / 3; // Results in 0
    println!(" 2 / 3  is {floored}");

    // remainder
    let remainder = 43 % 5;
    println!(" 43 % 5 is {remainder}");
}

fn tuple_and_array() {
    /* Compound Types */
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x, y, z is: {x} {y} {z}");

    // Array has fixed length in Rust
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("Please enter a number of the month.");

    // let mut index = String::new();

    // std::io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");

    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("Index entered was not a number");
    let index = 6;
    let element = months[index];

    println!("The value of the element at index {index} is: {element}");
}

fn statement_and_expression() {
    /* Statements and Expression
    Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value.
    */
    let y = 6; // Statement
    println!("y is {y}");
    let y = {
        let x = 3;
        x + 1 // adding semicolon would make this a statement and produce compile error
    };
    println!("y is {y}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn control_flow() {
    /* if Statement */
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    // let number = if condition { 5 } else { "six" };
    // compile error
    println!("The value of number is: {number}");

    /* Loop */
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The counter is {result}");

    /* Loop label */
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
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
}
