fn main() {
    /* Two ways to cause panic!
    1. by taking an action that causes our code to panic (such as accessing an array past the end)
    2. by explicitly calling the panic! macro.
    */

    /* To abort program when panic occurs
    - default is unwinding, which takes a lot of work
    - add following to Cargo.toml

    [profile.release]
    panic = 'abort'
    */

    // deliberately causing panic
    let v = vec![1, 2, 3];

    v[99];

    /* To backtract
    RUST_BACKTRACT=1 cargo run
     */
}
