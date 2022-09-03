fn main() {
    /* The Stack and the Heap
    Stack
    - last in, first out
    - known, fixed size
    - pushing onto or popping off of the stack

    Heap
    - allocating on the Heap
    - returns pointer
    - pointer to the heap is known, fixed size
    - pointer is stored on the stack

    - pushing onto stack is faster than allocating on the heap, because allocator must find a place to store data.
    - accessing data in the heap is slower because processor needs to follow a pointer to the data.
    - values passed into the functions get pushed onto the stack. the values pop off the stack when the function ends.
    */

    /* Ownership Rule
    1. Each value in Rust has an owner.
    2. There can only be one owner at a time.
    3. When the owner goes out of scrop, the value will be dropped.
    */
    ownership();
    references_and_borrowing();
    slice_type();
}

fn ownership() {
    /* The String Type */
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`

    /*
    let mut s = "Hello";
    s.push_str(", world!"); // does not compile.

    - literals content is known at compile time and is immutable.
    - In order to support a mutalbe piece of text, we need to allocate an amount of memory on the heap unkown at compile time to hold the contents.
        1. The memory must be requested from the memory allocator at runtime.
        2. We need a way of returning this memory to the allocator when we're done with our String.
    - In Rust, memory is automatically returned once the variable that owns it goes out of the scop.
     */

    {
        let _s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no
      // longer valid

    {
        let _x = 5;
        let _y = _x; // copy of the value in _x is bind to _y

        println!("x is {}!", _x); // works no problem
                                  // _x have known size at compile time and is stored entirely on the stack
                                  // stack-only data is copied
    }

    {
        let s1 = String::from("hello");
        let _s2 = s1; // s1 is moved to s2, not copied.

        // println!("{}, world!", s1); // does not compile. Rust considers s1 as no longer valid.
    }

    {
        let s1 = String::from("hello");
        let s2 = s1.clone(); // heap data is copied

        println!("s1 = {}, s2 = {}", s1, s2); // works find, but the code may be expensive.
    }

    /* Ownership Example */
    {
        fn takes_ownership(some_string: String) {
            // some_string comes into scope
            println!("{}", some_string);
        } // Here, some_string goes out of scope and `drop` is called. The backing
          // memory is freed.

        fn makes_copy(some_integer: i32) {
            // some_integer comes into scope
            println!("{}", some_integer);
        } // Here, some_integer goes out of scope. Nothing special happens.

        let s = String::from("hello"); // s comes into scope

        takes_ownership(s); // s's value moves into the function...
                            // ... and so is no longer valid here

        let x: i32 = 5; // x comes into scope

        makes_copy(x); // x would move into the function,
                       // but i32 is Copy, so it's okay to still
                       // use x afterward
    }

    /* Return Values Example */
    {
        fn gives_ownership() -> String {
            // gives_ownership will move its
            // return value into the function
            // that calls it

            let some_string = String::from("yours"); // some_string comes into scope

            some_string // some_string is returned and
                        // moves out to the calling
                        // function
        }

        // This function takes a String and returns one
        fn takes_and_gives_back(a_string: String) -> String {
            // a_string comes into
            // scope

            a_string // a_string is returned and moves out to the calling function
        }
        let _s1 = gives_ownership(); // gives_ownership moves its return
                                     // value into s1

        let s2 = String::from("hello"); // s2 comes into scope

        let _s3 = takes_and_gives_back(s2); // s2 is moved into
                                            // takes_and_gives_back, which also
                                            // moves its return value into s3
    } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
      // happens. s1 goes out of scope and is dropped.
}

fn references_and_borrowing() {
    /* Reference */
    {
        fn calculate_length(s: &String) -> usize {
            // s is a reference to a String
            s.len()
        } // Here, s goes out of scope. But because it does not have ownership of what
          // it refers to, it is not dropped.

        let s1 = String::from("hello");

        let len = calculate_length(&s1);

        println!("The length of '{}' is {}.", s1, len);
    }

    /* Mutable References */
    {
        fn change(some_string: &mut String) {
            some_string.push_str(", world!")
        }

        let mut s = String::from("hello");
        change(&mut s);
    }

    // {
    //     let mut s = String::from("hello");

    //     let r1 = &mut s;
    //     let r2 = &mut s; // there cannot be multiple mutable borrow

    //     println!("{}, {}", r1, r2);
    //     // Rust does not allow mutiple mutable references to prevent data races at compile time.
    // }

    // {
    //     fn dangle() -> &String {
    //         // dangle returns a reference to a String

    //         let s = String::from("hello"); // s is a new String

    //         &s // we return a reference to the String, s
    //     } // Here, s goes out of scope, and is dropped. Its memory goes away.
    //       // Danger!

    //     let reference_to_nothing = dangle();
    // }
}

fn slice_type() {
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
