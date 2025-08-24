#[derive(Debug)]
pub struct Opcode {
    pub id: String
}

impl Opcode {
    pub fn is_macro(code: &String) -> bool {
        matches!(code.as_str(), "STRING" | "ARRAY" | "VAR" | "LINK")
    }
}
