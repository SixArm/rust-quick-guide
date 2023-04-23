# Git branch name

A Git branch name can be very helpful for developers, and many project developers choose to use a git branch name convention. Here are some we've seen on projects.

If you like the Git commit message convention that uses imperative phrases such as "Add help button" or "Fix page layout", then a branch name can be `add-help-button` or `fix-page-layout`.

Consider variations such as kebab case `add-help-button`, snake case `add_help_button`, title case `AddHelpButton`, camel case `addHelpButton`, etc.

If you like to align with a project tracker issue number, then a branch name can be `issue-100-add-help-button`. Caveat: to avoid confusion in third-party tools, it's best to start a branch name with a letter.


## Grouping

If you like grouping branches, then consider using a slash "/" to simulate a directory and subdirectory. Example: if you like grouping by "features" versus "bugs", then a branch name can be `features/add-help-button`. Note: if your Git client  treats a slash "/" as a directory separator, then this works especially well. Caveat 1: Many git clients and sites ignore the slash. Caveat 2: Git branch names are implemented as paths, which means branch names "foo" and "foo/bar" cannot coexist.

If you like grouping but don't like a slash "/", then consider any other separator, such as double hyphens `features--add-help-button`. You can use special characters if you escape them. Or, if you want to be safe, then use English alphanumerics (A-Z, a-z, 0-9), and punctuation characters period, hyphen, underscore, and forward slash.


## GitHub

GitHub restricts branch names: no names which look like Git object IDs (40 characters only 0-9 and A-F), and no names beginning with "refs/".