use compiler_dev::ebnf::sysy;
mod ast;
fn main() {
    let node = sysy::ExprParser::new().parse("+(- -!6)").unwrap();
    println!("{:?}", node);
    println!("Hello, world!");
}
