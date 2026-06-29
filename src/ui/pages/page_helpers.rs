use ellipse::Ellipse;

pub fn get_column_string(text: &str, width: usize) -> String {
    let len = text.len();
    match width.cmp(&len) {
        std::cmp::Ordering::Equal => return text.to_string(),
        std::cmp::Ordering::Greater => {
            let mut ellipsized_text = text.to_string();
            for _ in 0..width - text.len() {
                ellipsized_text.push(' ');
            }
            return ellipsized_text;
        }
        std::cmp::Ordering::Less => {
            if width <= 3 {
                let mut ellipsized_text = "".to_string();
                for _ in 0..width {
                    ellipsized_text.push('.');
                }
                return ellipsized_text;
            }
            return text.truncate_ellipse(width - 3).to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_column_string() {
        let text1 = "";
        let text2 = "test";
        let text3 = "testme";
        let text4 = "testmetest";

        let width = 0;

        assert_eq!(get_column_string(text4, width), "".to_owned());

        let width = 1;

        assert_eq!(get_column_string(text4, width), ".".to_owned());

        let width = 2;

        assert_eq!(get_column_string(text4, width), "..".to_owned());

        let width = 3;

        assert_eq!(get_column_string(text4, width), "...".to_owned());

        let width = 4;

        assert_eq!(get_column_string(text4, width), "t...".to_owned());

        let width = 6;

        assert_eq!(get_column_string(text1, width), "      ".to_owned());
        assert_eq!(get_column_string(text2, width), "test  ".to_owned());
        assert_eq!(get_column_string(text3, width), "testme".to_owned());
        assert_eq!(get_column_string(text4, width), "tes...".to_owned());
    }
}
