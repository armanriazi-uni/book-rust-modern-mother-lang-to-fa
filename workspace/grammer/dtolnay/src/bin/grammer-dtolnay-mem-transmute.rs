/// grammer-dtolnay
///
/// ## Commands
///
/// ```cargo run -q -p grammer-dtolnay_bin --bin grammer-dtolnay-mem-transmute```
///
/// ```cargo doc  --package grammer-dtolnay_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package grammer-dtolnay_bin```
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `2`
///
/// ## Example
/// `TODO`
/// //```rust,no_run,ignore,compile_fail
/// **

#[repr(u8)]
enum Enum {
    First, //0u8
    Second,//1u8
}
impl Enum {
    fn p(self) {
        match self {
            Enum::First => print!("1"),
            Enum::Second => print!("2"),
        }
    }
}
fn main() {
    Enum::p(unsafe {
        std::mem::transmute(1u8)
    });
}
/*
1

Filling in the implicit discriminants, the definition of Enum is equivalent to:

#[repr(u8)]
enum Enum {
    First = 0u8,
    Second = 1u8,
}
The unsafe transmute is a red herring. The attribute #[repr(u8)] guarantees that our type has the same representation as u8, and the discriminant on Enum::Second guarantees that Enum::Second has the same representation as 1u8. The transmute is well-defined and evaluates to Enum::Second.

If the method p had been written as:

match self {
    Enum::First => print!("1"),
    Enum::Second => print!("2"),
}
then this program would print 2.

However, as written, both arms of the match expression are wildcard matches that successfully match any value and bind a variable with the name First or Second. Match arms are applied in order so the wildcard match in the first arm is always the one matched.

The compiler helps us out with not one but two relevant warnings. First it describes exactly how this match is parsed and why that is probably silly.

warning: unreachable pattern
  --> questions/007.rs:11:13
   |
10 |             First => print!("1"),
   |             ----- matches any value
11 |             Second => print!("2"),
   |             ^^^^^^ unreachable pattern
Second, it recognizes what the programmer has done wrong and what they probably meant to write instead.

warning[E0170]: pattern binding `First` is named the same as one of the variants of the type `Enum`
  --> questions/007.rs:10:13
   |
10 |             First => print!("1"),
   |             ^^^^^ help: to match on the variant, qualify the path: `Enum::First`
An alternative to writing qualified paths in the pattern is to bring the variants into scope.

use Enum::*;

match self {
    First => print!("1"),
    Second => print!("2"),
}
Having variants brought into scope by the standard library prelude is what allows us to write Ok and Some in match arms, rather than the qualified paths Result::Ok and Option::Some.
*/
