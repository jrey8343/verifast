// Fn item as an operand (not coerced to a fn pointer). On master without this change:
// Fatal error: exception Failure("Todo: Functions as operand in rvalues are not supported yet")
#![no_std]
#![allow(dead_code)]
fn check(r: Result<u32, u32>) -> Result<(), u32>
{
    //@ assume(false); // safe fn, default spec; only the operand translation is under test
    r.err().map_or(Ok(()), Err)
}
