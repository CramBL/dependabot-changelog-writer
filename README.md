<div align=right>Table of Contents↗️</div>

<h1 align=center><code>Dependabot Changelog Writer</code></h1>

<div align=center>
  <a href=https://github.com/CramBL/dependabot-changelog-writer/actions>
    <img src=https://github.com/CramBL/dependabot-changelog-writer/actions/workflows/CI.yml/badge.svg alt="CI status">
  </a>
  <a href=https://codecov.io/github/CramBL/dependabot-changelog-writer>
    <img src=https://codecov.io/github/CramBL/dependabot-changelog-writer/graph/badge.svg?token=YBFSKWY0HI alt=codecov>
  </a><a href=https://github.com/CramBL/dependabot-changelog-writer/releases>
    <img src=https://img.shields.io/github/downloads/CramBL/dependabot-changelog-writer/total alt="GitHub Downloads (all assets, all releases)">
  </a>

<br>
<br>

<b>Generate a changelog entry</b> from a dependabot PR, <b>commit</b> & <b>push</b> the changes.

</div>

<br>

## Minimal workflow example

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
    - uses: crambl/dependabot-changelog-writer@trunk # Always use the latest RELEASED version of this action
```

If you have CI checks that are invalidated by in-workflow pushes you can add a PAT to make CI trigger on the push

```yaml
- uses: crambl/dependabot-changelog-writer@trunk
  with:
    push-token: ${{ secrets.PAT_PUSH }} # Just needs 'public_repo' scope if your repo is public otherwise needs 'repo'
```

## Exhaustive Example usage

```yaml
jobs:
  update-changelog:
    if: github.actor == 'dependabot[bot]'
    runs-on: ubuntu-latest
    permissions:
      contents: write # Needed for pushing commit
    steps:
    - uses: actions/checkout@v4
    - uses: crambl/dependabot-changelog-writer@trunk # or specific tag
      with:
        # Path of the changelog relative to the project root
        # default: './CHANGELOG.md'
        changelog: ''

        # Template string defining how dependency updates are formatted in changelog entries.
        # Uses [dep], [old], [new] as placeholder tokens for dependency name, old version, 
        # and new version respectively. Tokens must appear in order: [dep], [old], [new].

        # Examples:
        # - Pattern: '[dep]: [old] → [new]' produces 'npm: 1.0 → 1.2'
        # - Pattern: 'Bump [dep] from [old] to [new]' produces 'Bump npm from 1.0 to 1.2'
        # default: '[dep]: [old] → [new]'
        update-entry-pattern: ''

        # The commit message for the changelog entry
        # default: 'Updated changelog with updated dependencies'
        commit-message: ''

        # The name of the custom committer you want to use
        # default: 'github-actions[bot]'
        committer-name: ''

        # The email of the custom committer you want to use
        # default: 'github-actions[bot]@users.noreply.github.com'
        committer-email: ''

        # The version/H2 header to find in the CHANGELOG to add dependabot entries to
        # matches whether or not the version is in brackets e.g. [0.1.0] and 0.1.0 are
        # both valid. if set to 'unreleased' it will match 'unreleased' case-insensitive.
        # default: 'unreleased'
        version: ''

        # The section/H3 header to add the changelog entry under
        # default: 'Dependencies'
        section-header: ''

        # Whether or not to add, commit, & push the modified changelog
        # Note: If no changes are made, no commit is made either
        # default: 'true'
        push-changes: ''

        # The GitHub token to use for git push
        # default: ${{ github.token }}
        # Note: Default token won't re-trigger workflows. Use a PAT if workflows should be re-triggered
        push-token: ''

        # The GitHub token to use for downloading the action
        # default: ${{ github.token }}
        action-download-token: ''
```

## Description

From a dependabot PR:

1. **Write a changelog entry** that describes the update actions performed by dependabot on the given PR
2. **Commit & push** the changes to the remote.

By default `dependabot-changelog-writer` generates entries under `unreleased` of the form:

```markdown
### Dependencies

- `dep`: [semver|SHA] → [semver|SHA]
- `foo`: 0.1.0 → 0.1.1
- `bar`: a05e0b3f9c28fe07bcde3e39bbb5765700925e49 -> 0ec44a1a9af25375e675218f48f0aaa1026ffc6d
- `baz`: `9618fa7` -> `2ef0ff8`
```

Then commits and pushes the changes to the current branch.

`dependabot-changelog-writer` runs are _idempotent_, if you rerun it there will be no new changes to the changelog and no commit is created.

### Handles edge cases

- [x] Dependabot bumped a dependency from one **Git SHA** to another
- [x] A submodule was updated to another **short SHA**
- [x] The `Dependencies` section already contains an entry like `bump X from A to B` - Handled by replacing that entry with `bump X from A to C`
- [x] Branch is checked out detached from HEAD (e.g. workflow triggered by opened pull request) - Locates HEAD from remote before committing

## Alternatives

An alternative to this action is to combine a solution to generate changelog entries with a solution to add, commit, & push changes. If you have or know of a solution and it is not listed here, please make an issue so I can add it.

###  Generating dependabot changelog entries:

- [dangoslen/dependabot-changelog-helper](https://github.com/dangoslen/dependabot-changelog-helper)

### Adding, committing, & pushing changes

- [EndBug/add-and-commit](https://github.com/EndBug/add-and-commit/)
- [stefanzweifel/git-auto-commit-action](https://github.com/stefanzweifel/git-auto-commit-action)
