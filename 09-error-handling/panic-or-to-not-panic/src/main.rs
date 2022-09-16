pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    /* When to panic!
    Your code panic when it’s possible that your code could end up in a bad state.
    - The bad state is something that is unexpected, as opposed to something that will likely happen occasionally, like a user entering data in the wrong format.
    - Your code after this point needs to rely on not being in this bad state, rather than checking for the problem at every step.
    - There’s not a good way to encode this information in the types you use. We’ll work through an example of what we mean in the “Encoding States and Behavior as Types” section of Chapter 17.
    */

    /* When not to panic!
    However, when failure is expected, it’s more appropriate to return a Result than to make a panic!

    */
    println!("Hello, world!");
}
