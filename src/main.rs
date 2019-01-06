
pub mod lexer;
pub use crate::lexer::*;

// def plus(x, y)
//  x + y
//
// plus (1 2)
//
// extern sin(x);
//
// sin(1)

fn main() {
    let program = "
# Comment line
plus(1 2) # another comment  
";
    let tokens = tokenize(program);
    println!("TOKENS");
    println!("{:#?}", tokens);
}
