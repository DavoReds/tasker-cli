# Tasker CLI

[![Continuous integration](https://github.com/DavoReds/tasker-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/DavoReds/tasker-cli/actions/workflows/ci.yml)

Tasker CLI is a command line application for keeping track of your daily tasks.

It saves your tasks as local files that can be accessed from
[Tasker TUI](https://github.com/DavoReds/tasker-tui) and [Tasker]().
No personal information is asked or gathered by the program.

## Installation

From [crates.io](https://crates.io/).

```bash
cargo install tasker-cli
```

Alternatively, you can download a binary from the
[releases](https://github.com/DavoReds/tasker-cli/releases) page.

## Usage

```bash
Usage: tasker-cli [COMMAND]

Commands:
  create    Creates a new task
  edit      Edits an existing task
  complete  Marks a task as complete
  delete    Deletes a task, whether completed or not
  clean     Cleans all completed tasks
  list      Lists all tasks (Default)
  config    Configures the application
  path      Get path to the configuration file
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Demonstration

[![asciicast](https://asciinema.org/a/582063.svg)](https://asciinema.org/a/582063)

## Configuration

All of the program's configuration is saved within one `.yml` file. You can
find out where said file is saved with the `path` subcommand.

Only two fields are configurable: `name` and `language`.

### Example

This is an example of a `tasker_cli.yml` file:

```yaml
---
name: Dalia
language: Spanish
```

You may change this configuration manually or through the `config` command.
It takes two arguments: `name` and `language`. Currently, only English and
Spanish are supported. But your welcome to make a PR with your language!

### Demonstration

[![asciicast](https://asciinema.org/a/582064.svg)](https://asciinema.org/a/582064)

## Tasks

Similar to the configuration, tasks themselves are just a file. They are saved
in you user folder, inside a `.tasker` directory. You may also edit these files
manually and see changes reflected in the program without problems, as long as
you don't break the structure of the data.

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[GNU General Public License v3.0](https://choosealicense.com/licenses/gpl-3.0/)
