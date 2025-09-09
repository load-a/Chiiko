pub mod numeral_parser {
    pub fn parse_str(slice: &str) -> Option<usize> {
        if !is_numeric(slice) {
            return None
        }

        let slice = slice.to_uppercase();

        Some(match &slice[0..=1] {
            "0X" => usize::from_str_radix(&slice[2..], 16).unwrap(),
            "0O" => usize::from_str_radix(&slice[2..], 8).unwrap(),
            "0B" => usize::from_str_radix(&slice[2..], 2).unwrap(),
            _ => usize::from_str_radix(&slice, 10).unwrap(),
        })
    }

    pub fn is_numeric(slice: &str) -> bool {
        let slice = slice.to_uppercase();

        if let Some(rest) = slice.strip_prefix("0X") {
            usize::from_str_radix(rest, 16).is_ok()
        } else if let Some(rest) = slice.strip_prefix("0O") {
            usize::from_str_radix(rest, 8).is_ok()
        } else if let Some(rest) = slice.strip_prefix("0B") {
            usize::from_str_radix(rest, 2).is_ok()
        } else {
            usize::from_str_radix(&slice, 10).is_ok()
        }
    }
}
