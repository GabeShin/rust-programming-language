fn main() {
    // Both String and &str is UTF-8 encoded

    /* Creating String */
    let mut s = String::new();
    // from string literal
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    /* Updating a String */
    let mut s = String::new();
    s.push_str("foo");
    s.push_str("bar");
    println!("s is {s}");

    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);
    println!("result is {} and {}", s, s2);

    // push_str does not take ownership of s2, it takes a string slice.

    // push method takes a char and adds it to the String.
    let s3 = 'b';
    let s4 = 'a';
    s.push(s3);
    s.push(s4);
    s.push('z');
    println!("result is {}, {}, {}", s, s3, s4);

    /* Concatenation with + operator */
    let s1 = String::from("foo");
    let s2 = String::from("bar");
    let mut s3 = s1 + &s2;
    println!("{}, {}", s2, s3);
    /*
    + operator is bascially add method. note s1 has been moved here and can no longer be used.
    While `let s3 = s1 + &s2` looks like copy both string and create a new one, it takes ownership of s1 and appends a copy of the contents of s2 and returns ownership of the result.
    The implementation is much more efficient than copying.
     */
    // Other tests
    s3 += &s2;
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{s}");

    /* format! macro */
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{s}");

    /* Indexing into Strings */
    // rust does not support String Indexing

    let hello = String::from("안녕하세요");
    /* Test sample
    let h = hello[0]; // this will crash
    */

    /*
    - Rust encodes char into UTF-8 and stores as byte data.
    - so when asking for hello[0], it is essentially looking for the first byte.
    - You could ask for string slice
    */

    let s = &hello[0..3];
    println!("{s}"); // returns 안
                     /* This would also crash if index is [0..4] very risky... */

    // Best way to work on pieces of string is by being explicit about whether you want char or bytes.
    for c in hello.chars() {
        println! {"{c}"};
    }

    for b in hello.bytes() {
        println! {"{b}"};
    }
}
