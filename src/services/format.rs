//! Formatting services.

/// Formats a number, adding commas.
pub fn format_with_commas(num: usize) -> String {
    let mut num_formatted = String::new();
    let num_str = num.to_string();

    for (idx, val) in num_str.chars().rev().enumerate() {
        if idx != 0 && idx % 3 == 0 {
            num_formatted.insert(0, ',');
        }

        num_formatted.insert(0, val);
    }

    num_formatted
}
