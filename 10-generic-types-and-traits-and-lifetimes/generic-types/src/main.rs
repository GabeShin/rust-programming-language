fn find_largest_int32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn find_largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let item_list = vec![34, 50, 25, 100, 65];
    let largest = find_largest_int32(&item_list);
    println!("The largest int32 is {}", largest);

    let item_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = find_largest_int32(&item_list);
    println!("The largest int32 is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = find_largest_char(&char_list);
    println!("The largest char is {}", result);

    /* Generic Data Types in Function definition  */
    {
        fn find_largest<T>(list: &[T]) -> &T
        where
            T: std::cmp::PartialOrd,
        {
            let mut largest = &list[0];

            for item in list {
                if item > largest {
                    largest = item;
                }
            }
            largest
        }

        let result_1 = find_largest(&item_list);
        let result_2 = find_largest(&char_list);
        println!("The largest is {result_1} and {result_2}");
    }

    /* Generic Data Types in Struct definition  */
    {
        struct Point<T> {
            x: T,
            y: T,
        }
        struct Point2<T, U> {
            x: T,
            y: U,
        }

        let both_integer = Point { x: 5, y: 10 };
        let both_float = Point { x: 1.0, y: 4.0 };
        // let wont_work = Point { x: 5, y: 4.0 };
        let both_float = Point2 { x: 1, y: 4.0 };
    }

    /* Generic Data Types in Enum definition
    - examples we've seen in earlier chapters
    */
    {
        enum Option<T> {
            Some(T),
            None,
        }
        enum Result<T, E> {
            Ok(T),
            Err(E),
        }
    }

    /* Generic Data Types in Method definition */
    {
        struct Point<X, Y> {
            x: X,
            y: Y,
        }

        // By declaring T as a generic type after impl, Rust can identify
        // that the type in the angle brackets in Point is a generic type
        // rather than a concrete type.
        impl<X, Y> Point<X, Y> {
            fn x(&self) -> &X {
                &self.x
            }

            fn y(&self) -> &Y {
                &self.y
            }

            fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X, Y2> {
                Point {
                    x: self.x,
                    y: other.y,
                }
            }
        }
    }

    /* Performance of Code using Generics
    Rust accomplishes this by performing monomorphization of the code using generics at compile time.

    Monomorphization is the process of turning generic code into specific code by filling in the concrete
    types that are used when compiled.

    In this process, the compiler does the opposite of the steps we used to create the generic function
    in Listing 10-5: the compiler looks at all the places where generic code is called and generates code
    for the concrete types the generic code is called with.

     Because Rust compiles generic code into code that specifies the type in each instance, we pay no
     runtime cost for using generics.
    */
}
