# Tasker CLI

Tasker CLI is a command line application for keeping track of your daily tasks.

It saves your tasks as local files that can be accessed from
[Tasker TUI]() and [Tasker](). No personal information is asked or gathered
by the program.

## Installation

From [crates.io](https://crates.io/).

```bash
cargo install tasker-cli
```

Alternatively, you can download a binary from the
[releases](https://github.com/DavoReds/tasker-cli/releases) page.

## Usage

```bash
Usage: tasker-cli <COMMAND>

Commands:
  create    Creates a new task
  edit      Edits an existing task
  complete  Marks a task as complete
  delete    Deletes a task, whether completed or not
  clean     Cleans all completed tasks
  list      Lists all tasks
  config    Configures the application
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[GNU General Public License v3.0](https://choosealicense.com/licenses/gpl-3.0/)
