use std::fs::File;
use std::io::Read;
use std::{io, io::ErrorKind};

fn main() {
    /* Recoverable Error with Result Enum */
    let greeting_file_result = File::open("hello.txt");
    // return type of File::open is Result<T, E>

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    /* Alternative to using match with Result<T, E>
    - using closure and unwrap_or_else
    - above and below is essentially the same, use whichever has better readability.
     */
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    /* Alternative to match - using unwrap() and expect()
    - unwrap() returns value inside T
    - expect() does the same, but also allows to choose error message.
    - use expect() and gives context on why the operation should always succeed.
    */
    let greeting_file = File::open("hello.txt").unwrap();
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");

    /* Propagating Error */
    read_username_from_file();

    /* Propagating Error using ? operator
    - The ? placed after a Result value is defined to work in almost the same way as the match expressions
    - If the value of the Result is an Ok, the value inside the Ok will get returned from this expression, and the program will continue.
    - If the value is an Err, the Err will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code.
    */
    read_username_from_file_2();

    /* Chaining method calls after ? operator */
    read_username_from_file_3();

    /* Where ? operator can be used
    - The ? operator can only be used in functions whose return type is compatible with the value the ? is used on.
    - ? operator is defined to perform an early return of a value out of the function.
    - For example, ? operator cannot be used on main() function.
    - The function should return Result<T, E> or Option<T>
    - You could make main function return Result<T, E> like following...
        fn main() -> Result<(), Box<dyn Error>> {
            ...
        }
    */
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
