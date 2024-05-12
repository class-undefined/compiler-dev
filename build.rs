use lalrpop::Configuration;

fn main() {
    Configuration::new()
        .set_out_dir(".")
        .process_dir("src/ebnf")
        .unwrap();
}
