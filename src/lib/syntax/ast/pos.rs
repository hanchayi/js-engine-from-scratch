#[derive(Clone, PartialEq)]
// js中的代码位置
pub struct Position {
    // 行号
    pub column_number: u64,
    // 列号
    pub line_number: u64,
}

impl Position {
    // 创建一个代码位置
    pub fn new(line_number: u64, column_number: u64) -> Position {
        Position {
            line_number: line_number,
            column_number: column_number,
        }
    }
}