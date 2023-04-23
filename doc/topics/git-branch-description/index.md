# Git branch description

Each Git branch can have a description. For example, a git branch description can explain the purpose of the branch, or provide contact information for the branch maintainers, or include links to the branch's specifications, or show any other text that you want.

To get a branch description:

```sh
git config branch.<name>.description
```

To set a branch description:

```sh
git config branch.<name>.description "My example text"
```

To edit the current branch's description via your preferred editor:

```sh
git branch --edit-description
```

## Git alias

To create a branch description git alias, edit your git config file (such as `$HOME/.gitconfig`) and add this:

```sh
[alias]
bd = !"git config branch."\
"$(git rev-parse --abbrev-ref HEAD 2>/dev/null)"\
".description"
```

To get the current branch's description:

```sh
git bd
```

To set the current branch's description:

```sh
git bd "My example text"
```
