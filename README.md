[![CI](https://github.com/CramBL/dependabot-changelog-writer/actions/workflows/CI.yml/badge.svg)](https://github.com/CramBL/dependabot-changelog-writer/actions/workflows/CI.yml)
[![codecov](https://codecov.io/github/CramBL/dependabot-changelog-writer/graph/badge.svg?token=YBFSKWY0HI)](https://codecov.io/github/CramBL/dependabot-changelog-writer)
![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/CramBL/dependabot-changelog-writer/total)

# dependabot-changelog-writer

**Generate a changelog entry** from a dependabot PR, **commit** & **push** the changes.

## Purpose

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
    - uses: crambl/dependabot-changelog-writer@trunk
```

If you have CI checks that are invalidated by in-workflow pushes you can add a PAT to make CI trigger on the push

```yaml
- uses: crambl/dependabot-changelog-writer@v1.0.0
  with:
    push_token: ${{ secrets.PAT_PUSH }} # Just needs 'public_repo' scope if your repo is public otherwise needs 'repo'
```
