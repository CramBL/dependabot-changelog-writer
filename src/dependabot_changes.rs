#[cfg(test)]
mod dependabot_example_bodies;

#[derive(Debug)]
struct DependabotChange<'s> {
    name: &'s str,
    old_version: &'s str,
    new_version: &'s str,
}

pub fn parse_body(body: &str) {
    let changes = parse_changes(body);
    for change in &changes {
        println!("{:?}", change);
    }
    println!("{}", format_changes(changes));
}

fn format_changes<'b>(changes: Vec<DependabotChange<'b>>) -> String {
    // Start with an empty string to accumulate the formatted result
    let mut markdown = String::new();

    // Iterate over each change and format it into the markdown string
    for change in changes {
        // For each change, add a list item in markdown format
        markdown.push_str(&format!(
            "- {}: {} → {}\n",
            change.name, change.old_version, change.new_version
        ));
    }

    // Return the final markdown string
    markdown
}

const UPDATE_LINE_KEYWORD: &str = "Updates";

/// Update keyword + 1 for the whitespace separator
const fn from_version_character_pos() -> usize {
    UPDATE_LINE_KEYWORD.len() + 1
}

fn parse_changes<'b>(body: &'b str) -> Vec<DependabotChange<'b>> {
    let mut changes = Vec::new();

    // Split the body into lines
    let lines = body.lines();

    // Loop through the lines
    for line in lines {
        // Look for lines that match the pattern of a version change
        if let Some(start) = line.find(UPDATE_LINE_KEYWORD) {
            let remaining = &line[start..];

            // Try to extract the dependency name, old version, and new version
            if let Some(from_pos) = remaining.find("from") {
                let to_pos = remaining.find("to").unwrap_or(remaining.len());

                // Extract the dependency name (before "from")
                let name = &remaining[from_version_character_pos()..from_pos].trim();

                // Extract the old version (between "from" and "to")
                let old_version = &remaining[from_pos + 4..to_pos].trim();

                // Extract the new version (after "to")
                let new_version = &remaining[to_pos + 2..].trim();

                changes.push(DependabotChange {
                    name,
                    old_version,
                    new_version,
                });
            }
        }
    }

    changes
}

#[cfg(test)]
mod tests {
    use super::*;
    use dependabot_example_bodies::*;

    #[test]
    fn test_parse_body() {
        parse_body(EXAMPLE_DEPENDABOT_BODY_SETTINGS_MANAGER);
    }

    #[test]
    fn test_parse_example_to_changes() {
        let changes = parse_changes(EXAMPLE_DEPENDABOT_BODY_SETTINGS_MANAGER);
        assert_eq!(changes.len(), 2);
    }

    #[test]
    fn test_parse_example_to_changes_plotinator() {
        let changes = parse_changes(EXAMPLE_DEPENDABOT_BODY_PLOTINATOR);
        assert_eq!(changes.len(), 8);
    }
}
