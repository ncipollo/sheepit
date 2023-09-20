# sheepit 🐑

A simple rust tool for releasing projects 🚀.

# Installation

```bash
cargo install sheepit
```

# Configuration

You can configure Sheepit by placing a `sheepit.toml` or `.sheepit.toml` file in your repository. This file has the
following options:

```toml
[repository]
branch_pattern = 'branch/{version}' # The naming pattern for the branch if one is created.
commit_message = 'Sheeping {version}' # Commit message if we will create a commit
default_branch = 'main' # Default branch in the repo. 
enable_branch = true # When true sheepit will create a release branch before commiting and tagging.
enable_commit = true # When true sheepit will apply transforms and commit changes.
enable_tag = true # When true sheepit will create a tag from your latest commit.
enable_push = true # When true sheepit will push changes to origin (unless you dry-run)
tag_pattern = '{version}' # The naming pattern to use when creating a tag 

# Each of the below transforms will find and replace a single string in the specified file. If you need to 
# replace multiple strings, add multiple transforms.
[[transforms]]
path = 'relative/path.file' # The relative path to the file.
find = 'version: {version}' # An optional, explicit string to find. If omitted, replace will be used for find & replace.
replace = 'version: sheep_{version}' # The replace string.
```

## Version Token

The version token is `{version}`. This can be used in a number of configuration properties and may represent the repo's
current version (as determined by the highest semver tag), or the next version. Generally the following rules apply:

- When `{version}` is used in naming patterns or commit message it will be the **next** version.
- When `{version}` is used within a transform's `find` string it will be the **previous** version.
- When `{version}` is used within a transform's `replace` string it will be the **next** version.
    - If `replace` is used for both `find` and `replace`, `{version}` will expand to the previous version while finding,
      then the next versions file replacing.

## Defaults

You don't need to specify all of the configuration properties listed at the top of the section. Sheepit tries to pick
reasonable defaults. Defaults can be found here: [config.rs](https://github.com/ncipollo/sheepit/blob/50966739b427659f5930c275599c78c21a04e9b0/src/config.rs#L90).

# Bumping Versions

Sheepit supports semantic version bumps. You can bump the major, minor and patch version. During a version bump, sheepit
will do the following:

- Find the lastest version by looking at your repo's tag list.
- Figure out the next version depending on the bump type.
- Optionally create a release branch for you.
- Optionally create a commit after performing any necessary transforms (coming soon!).
- Optionally create a tag.
- Optionally push to a remote.

## Commands

```bash
# Assuming your version is 1.1.1
sheepit major # bumps the version to 2.0.0
sheepit minor # bumps the version to 1.2.0
sheepit patch # bumps the version to 1.1.2
```

