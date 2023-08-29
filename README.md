# sheepit 🐑

A simple rust tool for releasing projects 🚀.

# Installation

```bash
cargo install sheepit
```

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

