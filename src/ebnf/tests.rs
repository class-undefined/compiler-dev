mod tests {
    #[test]
    fn test_binary() {
        use super::super::sysy;
        let node = sysy::ExprParser::new().parse("1 + 2 * 3").unwrap();
        let (id, ir) = node.to_ir();
        println!("id: {}, \nir:\n{}", id, ir);
    }
}
