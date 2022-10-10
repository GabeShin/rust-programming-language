use std::vec;
use iterators;

fn main() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    for value in v1_iter {
        println!("Value is {value}");
    }

    // assert_eq!(v1_iter.next(), Some(&1)) ;
    // assert_eq!(v1_iter.next(), Some(&2)) ;
    // assert_eq!(v1_iter.next(), Some(&3)) ;
    // assert_eq!(v1_iter.next(), None) ;

    // iterator with mutable reference
    let mut v1 = vec![1, 2, 3];

    let mut v1_iter_2 = v1.iter_mut();
    for value in v1_iter_2 {
        *value += 1;
        println!("Value is {value}");
    }

    for value in v1.iter_mut() {
        println!("Remaining value is updated1 {value}");
    }

    // methods that consume the iterator
    // let v1 = vec![1, 2, 3];
    let total: i32 = v1.iter().sum();
    println!("sum is {total}");

    /* method that produce other iterators 
    - `map` takes closure.
    - have to use `collect()` method to consumer the iterator.
    - could chain multiple iterator adaptors to perform complex action in readable way.
    */
    let v2: Vec<i32> = v1.iter().map(|x| x+ 1).collect();
    assert_eq!(v2, vec![3, 4, 5]);

    /* Using closures that capture their environment*/

}
