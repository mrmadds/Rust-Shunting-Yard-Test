mod parser;
mod ast;

use parser::Parser;

fn main() {
    let expr = String::from("2 * 4 + 3 * 2");

    let ast = Parser::from(expr)
        .enable_debug()
        .parse_tokens()
        .parse_rpn()
        .disable_debug()
        .parse_ast()
        .collect();

    println!("Output: {:?}", ast.eval().into_i32().unwrap());
}
