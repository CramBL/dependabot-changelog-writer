use std::error::Error;

#[derive(Debug)]
pub struct EntryPattern {
    cooked_pattern: String,
    // The length of the pattern string minus the tokens
    min_len: usize,
    pr_link_token_occurrences: usize
}

impl Default for EntryPattern {
    fn default() -> Self {
        Self::new(Self::DEFAULT_PATTERN).expect("Failed to construct default entry pattern")
    }
}

impl EntryPattern {
    const DEFAULT_PATTERN: &str = "[dep]: [old] → [new] ([pr-link])";

    // Markdown prefix for a list entry
    const LINE_PREFIX: &'static str = "- ";

    pub const DEPENDENCY_TOKEN: &str = "[dep]";
    pub const OLD_VERSION_TOKEN: &str = "[old]";
    pub const NEW_VERSION_TOKEN: &str = "[new]";
    pub const PULL_REQUEST_LINK_TOKEN: &str = "[pr-link]";

    pub const ORDERED_TOKENS: &[&str] = &[
        Self::DEPENDENCY_TOKEN,
        Self::OLD_VERSION_TOKEN,
        Self::NEW_VERSION_TOKEN,
    ];

    pub const DEPENDENCY_TOKEN_HARDENED: &str = "{{dep}}";
    pub const OLD_VERSION_TOKEN_HARDENED: &str = "{{old}}";
    pub const NEW_VERSION_TOKEN_HARDENED: &str = "{{new}}";
    pub const PULL_REQUEST_LINK_TOKEN_HARDENED: &str = "{{pr-link}}";

    /// # Arguments
    ///
    /// `pattern`: Template string defining how dependency updates are formatted in changelog entries.
    /// Uses [dep], [old], [new], [pr-link] as placeholder tokens for dependency name, old version, new version,
    /// and Pull request link respectively. Tokens must appear in order: [dep], [old], [new]. With [pr-link]
    /// being an exception to that rule.
    ///
    /// e.g. 'Bump [dep] from [old] to [new] ([pr-link])'
    ///
    /// # Errors
    ///
    /// If the pattern does not contain all the expected tokens exactly once and in order.
    pub fn new(pattern: &str) -> Result<Self, Box<dyn Error>> {
        if pattern.is_empty() {
            return Err(
                "Missing entry pattern. Expected template string such as: '[dep]: [old] → [new] ([pr-link])'"
                    .into(),
            );
        }

        let pr_link_token_occurrences = pattern.matches(Self::PULL_REQUEST_LINK_TOKEN).count();

        let mut last_index = 0;

        // Check that tokens appear exactly once and in order
        for token in Self::ORDERED_TOKENS.iter() {
            let occurrences = pattern.matches(token).count();
            if occurrences > 1 {
                return Err(
                    format!("{occurrences} occurrences of {token}, expected exactly 1").into(),
                );
            }

            if let Some(index) = pattern[last_index..].find(token) {
                last_index += index + token.len();
            } else if pattern.contains(token) {
                return Err(format!(
                    "out-of-order token: {token}. Expected order: [dep], [old], [new]."
                )
                .into());
            } else {
                return Err(format!("Missing token: {token}").into());
            }
        }

        // Replace tokens with collision-proof versions
        let hardened_pattern = pattern
            .replace(Self::DEPENDENCY_TOKEN, Self::DEPENDENCY_TOKEN_HARDENED)
            .replace(Self::OLD_VERSION_TOKEN, Self::OLD_VERSION_TOKEN_HARDENED)
            .replace(Self::NEW_VERSION_TOKEN, Self::NEW_VERSION_TOKEN_HARDENED)
            .replace(
                Self::PULL_REQUEST_LINK_TOKEN,
                Self::PULL_REQUEST_LINK_TOKEN_HARDENED,
            );

        let mut cooked_pattern = Self::LINE_PREFIX.to_owned();
        cooked_pattern.push_str(&hardened_pattern);
        cooked_pattern.push('\n');

        let min_len = cooked_pattern.len()
            - Self::DEPENDENCY_TOKEN_HARDENED.len()
            - Self::OLD_VERSION_TOKEN_HARDENED.len()
            - Self::NEW_VERSION_TOKEN_HARDENED.len()
            - pr_link_token_occurrences * Self::PULL_REQUEST_LINK_TOKEN_HARDENED.len();

        Ok(Self {
            cooked_pattern,
            min_len,
            pr_link_token_occurrences
        })
    }

    /// The length of the template if all tokens are replaced by an empty string.
    pub const fn min_len(&self) -> usize {
        self.min_len
    }

    /// How many times was the [PULL_REQUEST_LINK_TOKEN](Self::PULL_REQUEST_LINK_TOKEN) found in the pattern 
    pub fn pull_request_link_token_occurrences(&self) -> usize {
        self.pr_link_token_occurrences
    }

    pub fn format(
        &self,
        dependency: &str,
        old_version: &str,
        new_version: &str,
        markdown_pull_request_link: &str,
    ) -> String {
        self.cooked_pattern
            .replace(Self::DEPENDENCY_TOKEN_HARDENED, dependency)
            .replace(Self::OLD_VERSION_TOKEN_HARDENED, old_version)
            .replace(Self::NEW_VERSION_TOKEN_HARDENED, new_version)
            .replace(
                Self::PULL_REQUEST_LINK_TOKEN_HARDENED,
                markdown_pull_request_link,
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_str_eq;

    #[test]
    fn test_valid_pattern_default() {
        let pattern = EntryPattern::DEFAULT_PATTERN;
        let entry_pattern = EntryPattern::new(pattern).unwrap();
        assert_str_eq!(
            entry_pattern.cooked_pattern,
            "- {{dep}}: {{old}} → {{new}} ({{pr-link}})\n"
        );
        assert_eq!(entry_pattern.min_len(), "- :  →  ()\n".len());
    }

    #[test]
    fn test_valid_pattern_simple() {
        let pattern = "Bump [dep] from [old] to [new] ([pr-link])";
        let entry_pattern = EntryPattern::new(pattern).unwrap();
        assert_str_eq!(
            entry_pattern.cooked_pattern,
            "- Bump {{dep}} from {{old}} to {{new}} ({{pr-link}})\n"
        );
        assert_eq!(entry_pattern.min_len(), "- Bump  from  to  ()\n".len());
    }

    #[test]
    fn test_valid_pattern_simple_pr_link_at_beginning() {
        let pattern = "([pr-link]) Bump [dep] from [old] to [new]";
        let entry_pattern = EntryPattern::new(pattern).unwrap();
        assert_str_eq!(
            entry_pattern.cooked_pattern,
            "- ({{pr-link}}) Bump {{dep}} from {{old}} to {{new}}\n"
        );
        assert_eq!(entry_pattern.min_len(), "- ()  Bump  from  to\n".len());
    }

    #[test]
    fn test_valid_pattern_simple_without_pr_link() {
        let pattern = "Bump [dep] from [old] to [new]";
        let entry_pattern = EntryPattern::new(pattern).unwrap();
        assert_str_eq!(
            entry_pattern.cooked_pattern,
            "- Bump {{dep}} from {{old}} to {{new}}\n"
        );
        assert_eq!(entry_pattern.min_len(), "- Bump  from  to \n".len());
    }

    #[test]
    fn test_valid_pattern_multiple_pr_link() {
        let pattern = "[pr-link] bumps [dep] from [old] to [new] ([pr-link])";
        let entry_pattern = EntryPattern::new(pattern).unwrap();
        assert_str_eq!(
            entry_pattern.cooked_pattern,
            "- {{pr-link}} bumps {{dep}} from {{old}} to {{new}} ({{pr-link}})\n"
        );
        assert_eq!(entry_pattern.min_len(), "-  bumps  from  to  ()\n".len());
    }

    #[test]
    fn test_valid_pattern_emojies() {
        let pattern = "📝 Update [dep] from [old] 🚀 [new]🍄";
        let entry_pattern = EntryPattern::new(pattern).unwrap();
        assert_str_eq!(
            entry_pattern.cooked_pattern,
            "- 📝 Update {{dep}} from {{old}} 🚀 {{new}}🍄\n"
        );
        assert_eq!("📝".len(), 4);
        assert_eq!("🚀".len(), 4);
        assert_eq!("🍄".len(), 4);
        assert_eq!(entry_pattern.min_len(), "- 📝 Update  from  🚀 🍄\n".len());
    }

    #[test]
    fn test_missing_token() {
        let pattern = "Bump [dep] from [old]";
        let result = EntryPattern::new(pattern);
        assert!(result.is_err());
        assert_str_eq!(result.unwrap_err().to_string(), "Missing token: [new]");
    }

    #[test]
    fn test_out_of_order_tokens() {
        let pattern = "Bump [old] to [new] for [dep]";
        let result = EntryPattern::new(pattern);
        assert!(result.is_err());
        assert_str_eq!(
            result.unwrap_err().to_string(),
            "out-of-order token: [old]. Expected order: [dep], [old], [new]."
        );
    }

    #[test]
    fn test_duplicate_tokens() {
        let pattern = "Bump [dep] from [old] to [new] and then back to [old]";
        let result = EntryPattern::new(pattern);
        assert_str_eq!(
            result.unwrap_err().to_string(),
            "2 occurrences of [old], expected exactly 1"
        );
    }

    #[test]
    fn test_edge_case_empty_pattern() {
        let pattern = "";
        let result = EntryPattern::new(pattern);
        assert_str_eq!(
            result.unwrap_err().to_string(),
            "Missing entry pattern. Expected template string such as: '[dep]: [old] → [new] ([pr-link])'"
        );
    }
}
