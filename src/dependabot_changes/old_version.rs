/// Represents the Old/former version of a dependency that is updates.
#[derive(Debug, Clone, PartialEq)]
pub enum OldVersion<'s> {
    /// The Old version described by the dependabot Pull Request body
    FromDependabot(&'s str),
    /// If the changelog already contains an entry for the dependency
    /// in the h3 section we're gonna write an entry to, this will be the
    /// old version from the existing changelog entry
    FromChangelog(String),
}

impl OldVersion<'_> {
    pub fn len(&self) -> usize {
        match self {
            // Comes from parsing the Dependabot PR body
            OldVersion::FromDependabot(s) => s.len(),
            // Comes from the Changelog (the section already mentions upgrading this version)
            // and replaces the version that comes from the Dependabot PR body
            OldVersion::FromChangelog(s) => s.len(),
        }
    }
}
