# git-10x-history
Generate a repository that gives a nice full GitHub history.

## Usage
```bash
$ ./git-10x-history <repo-path> <days-to-commit>
```
This will create a new repository at `<repo-path>` or open an existing one. Then it will generate 0-5 commits each day for `<days-to-commit>` days, counting back from the current day.

Weekends (saturdays, sundays) have only a 10% chance of generating commits.

## Building
Just run
```bash
$ cargo build --release
```