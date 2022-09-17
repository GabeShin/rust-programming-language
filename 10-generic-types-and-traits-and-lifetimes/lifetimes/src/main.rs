fn main() {
    /* Lifetime : the scope for which that reference is valid */

    /* 1. Preventing Dangling References with Lifetimes
    - main purpose of lifetime is to prevent dangling references.
    */

    let r;

    {
        let x = 5;
        r = &x; // 'x' does not live long enough
    }

    println!("r: {}", r); // r is referencing memory 'x' that was deallocated
                          // r determins that the code is invalid using borrow checker.

    /* 2. The Borrow Checker
    - compares scopes to determine whether all borrows are valid.

    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
                          // ---------+

    - at compile time, Rust compiler compares the size of the two lifetimes and sees that 'r' has
    lifetime of 'a but that it refers to memory within a lifetime of 'b. The program is rejected
    because 'b is shorter than 'a: the subject of the reference doesn't live as long as the
    reference.
    */

    /* 3. Generic Lifetimes in Functions
    4. Lifetime Annotation Syntax
    5. Lifetime Annotations in Function Signature

    &i32        // a reference
    &'a i32     // a reference with an explicit lifetime
    &'a mut i32 // a mutable reference with an explicit lifetime
     */
    let string1 = String::from("abcd");
    let string2 = "xyz";

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        // The returned reference will be valid as long as both the parameters(x, y) are valid.
        // This is the relationship betwen lifetimes of the parameters and the return value.
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    /* Lifetime Annocations in Struct Definition */
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    /* Lifetime Ellision
        1. The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference. In other words, a function with one parameter gets one lifetime parameter:

        fn foo<'a>(x: &'a i32);

        a function with two parameters gets two separate lifetime parameters:

        fn foo<'a, 'b>(x: &'a i32, y: &'b i32);

        and so on.


        2. The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters:

        fn foo<'a>(x: &'a i32) -> &'a i32.

        3. The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.
    */

    /* Lifetime Annotation in Method Definition */
    impl<'a> ImportantExcerpt<'a> {
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    /* Static Lifetime
    The affected reference can live for the entire duration of the program.
    */
    let s: &'static str = "I have a static lifetime.";
}
