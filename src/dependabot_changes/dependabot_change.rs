use super::old_version::OldVersion;


#[derive(Debug, Clone, PartialEq)]
pub struct DependabotChange<'s> {
    pub name: &'s str,
    old_version: OldVersion<'s>,
    pub new_version: &'s str,
}

impl<'s> DependabotChange<'s> {
    const PREFIX: &'static str = "- ";
    const NAME_OLD_VER_SEPARATOR: &'static str = ": ";
    const OLD_VER_NEW_VER_SEPARATOR: &'static str = " → ";

    pub const fn new(name: &'s str, old_version: &'s str, new_version: &'s str) -> Self {
        Self {
            name,
            old_version: OldVersion::FromDependabot(old_version),
            new_version,
        }
    }

    pub fn formatted_len(&self) -> usize {
        Self::PREFIX.len()
            + self.name.len()
            + Self::NAME_OLD_VER_SEPARATOR.len()
            + self.old_version.len()
            + Self::OLD_VER_NEW_VER_SEPARATOR.len()
            + self.new_version.len()
            + "\n".len()
    }

    pub fn from_str(haystack: &'s str) -> Option<Self> {
        // Try to extract the dependency name, old version, and new version
        if let Some(from_pos) = haystack.find("from") {
            let to_pos = haystack.find("to").unwrap_or(haystack.len());

            // Extract the dependency name (before "from")
            let name = &haystack[..from_pos].trim();

            // Extract the old version (between "from" and "to")
            let old_version = &haystack[from_pos + 4..to_pos].trim();

            // Extract the new version (after "to")
            let new_version = &haystack[to_pos + 2..].trim().trim_end_matches(".");

            Some(DependabotChange::new(name, old_version, new_version))
        } else {
            None
        }
    }

    pub fn replace_old_version(&mut self, old_version: String) {
        self.old_version = OldVersion::FromChangelog(old_version)
    }

    pub fn old_version(&self) -> &str {
        match self.old_version {
            OldVersion::FromDependabot(s) => s,
            OldVersion::FromChangelog(ref s) => s,
        }
    }
}

impl std::fmt::Display for DependabotChange<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{pre}{name}{sep1}{old_ver}{sep2}{new_ver}",
            pre = Self::PREFIX,
            sep1 = Self::NAME_OLD_VER_SEPARATOR,
            sep2 = Self::OLD_VER_NEW_VER_SEPARATOR,
            name = self.name,
            old_ver = self.old_version(),
            new_ver = self.new_version
        )
    }
}