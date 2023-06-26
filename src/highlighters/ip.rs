use crate::color;
use crate::color::to_ansi;
use crate::config_parser::Style;
use crate::highlight_utils::highlight_with_awareness;
use crate::highlighters::HighlightFn;
use crate::line_info::LineInfo;
use regex::{Captures, Regex};

pub fn highlight(segment: &Style, separator: &Style) -> HighlightFn {
    let segment_color = to_ansi(segment);
    let separator_color = to_ansi(separator);

    Box::new(move |input: &str, line_info: &LineInfo| -> String {
        highlight_ip_addresses(
            &segment_color,
            &separator_color,
            input,
            line_info,
            &ip_address_regex(),
        )
    })
}

fn ip_address_regex() -> Regex {
    Regex::new(r"(\b\d{1,3})(\.)(\d{1,3})(\.)(\d{1,3})(\.)(\d{1,3}\b)")
        .expect("Invalid IP address regex pattern")
}

fn highlight_ip_addresses(
    segment_color: &str,
    separator_color: &str,
    input: &str,
    _line_info: &LineInfo,
    ip_address_regex: &Regex,
) -> String {
    let highlight_groups = [
        (segment_color, 1),
        (separator_color, 2),
        (segment_color, 3),
        (separator_color, 4),
        (segment_color, 5),
        (separator_color, 6),
        (segment_color, 7),
    ];

    highlight_with_awareness(input, &ip_address_regex, |caps: &Captures<'_>| {
        let mut output = String::new();
        for &(color, group) in &highlight_groups {
            output.push_str(&format!("{}{}{}", color, &caps[group], color::RESET));
        }
        output
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_ip_addresses() {
        let line_info = &LineInfo {
            dashes: 0,
            dots: 0,
            slashes: 0,
            double_quotes: 0,
            colon: 0,
        };

        let ip_address = "192.168.0.1";
        let segment_color = "\x1b[31m"; // ANSI color code for red
        let separator_color = "\x1b[32m"; // ANSI color code for green

        let highlighted = highlight_ip_addresses(
            segment_color,
            separator_color,
            ip_address,
            line_info,
            &ip_address_regex(),
        );

        let expected = format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            segment_color,
            "192",
            color::RESET,
            separator_color,
            ".",
            color::RESET,
            segment_color,
            "168",
            color::RESET,
            separator_color,
            ".",
            color::RESET,
            segment_color,
            "0",
            color::RESET,
            separator_color,
            ".",
            color::RESET,
            segment_color,
            "1",
            color::RESET,
        );
        assert_eq!(highlighted, expected);
    }

    #[test]
    fn test_highlight_ip_addresses_no_ip() {
        let line_info = &LineInfo {
            dashes: 0,
            dots: 0,
            slashes: 0,
            double_quotes: 0,
            colon: 0,
        };

        let text = "this is a test string with no IP address";
        let segment_color = "\x1b[31m";
        let separator_color = "\x1b[32m";

        let highlighted = highlight_ip_addresses(
            segment_color,
            separator_color,
            text,
            line_info,
            &ip_address_regex(),
        );

        // The input string does not contain an IP address, so it should be returned as-is
        assert_eq!(highlighted, text);
    }
}
