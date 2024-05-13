use std::fs;

pub fn preprocess_input(file_path: &str) -> String {
    let content = fs::read_to_string(file_path).expect("Failed to read file");
    let lines = content.lines();
    let mut result = Vec::new();
    let mut carry_over = String::new();

    for line in lines {
        let trimmed = line.trim();
        match trimmed.chars().next() {
            Some('+') if !carry_over.is_empty() => {
                // 移除行首的 '+'，并与前一行内容连接
                carry_over.push(' '); // 添加一个空格作为连线
                carry_over.push_str(&trimmed[1..].trim_start());
            }
            _ => {
                if !carry_over.is_empty() {
                    // 将处理后的行添加到结果中
                    result.push(carry_over.clone());
                    carry_over.clear();
                }
                // 开始新的行处理
                carry_over = trimmed.to_string();
            }
        }
    }

    // 添加最后一行到结果中，如果它非空
    if !carry_over.is_empty() {
        result.push(carry_over);
    }

    result.join("\n")
}

mod tests {
    #[test]
    fn test_preprocess_input() {
        let path = "/Users/class-undefined/code/aucdl-parser/src/tests/cp.aucdl";
        let result = super::preprocess_input(path);
        println!("{}", result);
    }
}
