/// Parse a markdown table into headers and rows
pub struct MarkdownTable<'a> {
    pub headers: Vec<&'a str>,
    pub rows: Vec<Vec<&'a str>>,
}

impl<'a> MarkdownTable<'a> {
    /// Parse a markdown-formatted table string
    pub fn parse(markdown: &'a str) -> Option<Self> {
        let lines: Vec<&str> = markdown.trim().lines().collect();

        // Need at least header and separator line (2 lines minimum)
        if lines.len() < 2 {
            return None;
        }

        // Parse header (first line)
        let headers: Vec<&str> = lines[0]
            .split('|')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        if headers.is_empty() {
            return None;
        }

        // Parse data rows (skip header and separator line, then process rest)
        let mut rows: Vec<Vec<&str>> = Vec::new();
        for line in lines.iter().skip(2) {
            let cells: Vec<&str> = line
                .split('|')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect();
            if !cells.is_empty() {
                rows.push(cells);
            }
        }

        Some(MarkdownTable { headers, rows })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_markdown_table() {
        let markdown = r#"| Name | Age | City |
|------|-----|------|
| Alice | 30 | NYC |
| Bob | 25 | LA |"#;

        let table = MarkdownTable::parse(markdown).expect("Should parse table");

        assert_eq!(table.headers, vec!["Name", "Age", "City"]);
        assert_eq!(table.rows.len(), 2);
        assert_eq!(table.rows[0], vec!["Alice", "30", "NYC"]);
        assert_eq!(table.rows[1], vec!["Bob", "25", "LA"]);
    }

    #[test]
    fn test_parse_empty_table() {
        let markdown = "| Header |\n|--------|\n";
        let table = MarkdownTable::parse(markdown).expect("Should parse empty table");

        assert_eq!(table.headers, vec!["Header"]);
        assert_eq!(table.rows.len(), 0);
    }

    #[test]
    fn test_parse_invalid_markdown() {
        let markdown = "not a table";
        assert!(MarkdownTable::parse(markdown).is_none());
    }
}
