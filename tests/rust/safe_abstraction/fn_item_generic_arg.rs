// Fn item as an operand (not coerced to a fn pointer). On master without this change:
// Fatal error: exception Failure("Todo: Functions as operand in rvalues are not supported yet")
#![no_std]
#![allow(dead_code)]
fn identity(x: i32) -> i32 { x }
// `F: Copy` rather than `F: Fn(..)`: an `Fn` bound needs a typeid for the `<F as FnOnce>::Output`
// projection, which VeriFast does not have yet (separate gap). A fn item is Copy.
fn apply<F: Copy>(f: F, x: i32) -> i32
{
    //@ assume(false); // safe fn, default spec; only the operand translation is under test
    x
}
fn main()
{
    let _result = apply(identity, 42);
}
