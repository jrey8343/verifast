// Fn item as an operand (not coerced to a fn pointer). On master without this change:
// Fatal error: exception Failure("Todo: Functions as operand in rvalues are not supported yet")
#![no_std]
#![allow(dead_code)]
fn identity(x: i32) -> i32 { x }
fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32
//@ req true;
//@ ens true;
{
    //@ assume(false);
    f(x)
}
fn main()
//@ req true;
//@ ens true;
{
    let _result = apply(identity, 42);
}
