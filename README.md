# dependabot-changelog-writer

Generate a changelog entry from a dependabot PR

## Purpose

From a dependabot PR, write a changelog entry that describes the update actions performed by dependabot on the given PR and commit + push it to the remote.

By default `dependabot-changelog-writer` generates entries under `unreleased` of the form:

```markdown
### Dependencies

- `dep`: [semver|SHA] → [semver|SHA]
- `foo`: 0.1.0 → 0.1.1
```

Then commits and pushes the changes to the current branch.

### Handles edge cases

- [x] Dependabot bumped a dependency from one Git SHA to another 
- [x] A submodule was updated to another short SHA
- [x] The `Dependencies` section already contains an entry like `bump X from A to B` - Handled by replacing that entry with `bump X from A to C`

## Using this action

If your project has a `CHANGELOG.md` at the project root that always contains an `unreleased` section where you want Dependabot updates to be written under `### Dependencies`:

```yaml
name: Dependabot Changelog Entry

on:
  pull_request:
    types: [opened, reopened]
    branches: [main] # Your default branch

jobs:
  update-changelog:
    if: github.actor == 'dependabot[bot]'
    runs-on: ubuntu-latest
    permissions:
      contents: write # needed for pushing changes

    steps:
    - uses: actions/checkout@v4
    - uses: crambl/dependabot-changelog-writer@latest
```

If you have checks that are gets invalidated by in-workflow pushes you can add a PAT to make CI trigger on the push

```yaml
- uses: crambl/dependabot-changelog-writer@v0.1.2
      with:
        push_token: ${{ secrets.PAT_PUSH }} # Just needs 'public_repo' scope if your repo is public otherwise needs 'repo'
```

## TODO

- [ ] Edit the triggered release to set release notes and publish the action to GitHub Marketplace
