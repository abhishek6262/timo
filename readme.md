# Timo

Timo is a command-line application that allows you to capture and manage your thoughts and ideas directly from your terminal. It provides a simple and efficient way to jot down your thoughts, search through them, and keep them organized.

## Features

- **Add Thoughts**: Easily add new thoughts to the list with a single command.
- **Clear Thoughts**: Remove all thoughts from the list with a single command.
- **Remove Thoughts**: Remove specific thoughts from the list by providing their indexes.
- **Search Thoughts**: Search for thoughts in the list by providing a keyword or phrase.
- **List Thoughts**: Print all thoughts in the list to the terminal.
- **Labels**: Add labels to your thoughts to categorize them and make them easier to search and list.

## Roadmap

Here's a glimpse into what's coming next for Timo:

1. **Labels (Done)**: Add labels to your thoughts to categorize them and make them easier to search and list.
2. **Clear Confirmation (Done)**: Implement a confirmation step before clearing all thoughts to prevent accidental data loss.
3. **Reminders (Planned)**: Set reminders for your thoughts and get notified at the specified time.

**Note**: The roadmap is subject to change based on priorities and development progress.

## Installation

To use Timo, you can install it using Homebrew or build it from source using Rust and Cargo.

### Install using Homebrew (Recommended)

```bash
brew tap abhishek6262/timo
brew install timo
```

### Build from Source

1. Clone the Timo repository:

   ```bash
   git clone https://github.com/abhishek6262/timo
   ```

2. Navigate to the project directory:

   ```bash
   cd timo
   ```

3. Install Timo globally using `cargo install --path .` command.

## Usage

After installing Timo, you can use the following commands:

### Add a New Thought

```bash
timo add <thought> [-l <label> | --label=<label>]
```

Replace <thought> with your thought. Multiple words will be combined into a single thought. Optionally, use the -l flag followed by the desired label to assign it to the thought. For example:

```bash
timo add Buy groceries -l errands
timo add Work on report
```

### Clear All Thoughts

```bash
timo clear --confirmed
```

This command will remove all thoughts from the list.

### Remove Specific Thoughts

```bash
timo remove <id1> <id2> ...
```

Replace `<id1>`, `<id2>`, etc., with the ids of the thoughts you want to remove. The ids are displayed when you list the thoughts.

### Search for Thoughts

```bash
timo search <keyword> [-l <label> | --label=<label>] [-s | --show-labels]
```

Replace `<keyword>` with the word or phrase you want to search for. Optionally use the -l flag followed by the desired label to filter your search by a specific label. Also, use the -s flag to display the labels in the output. Timo will display all thoughts that contain the specified keyword or phrase. For example:

```bash
timo search meeting
timo search important -l work
```

### List All Thoughts

```bash
timo list [-l <label> | --label=<label>] [-s | --show-labels]
```

This command will print all thoughts in the list, along with their ids. Optionally use the -l flag followed by the desired label to print only the specified list. Also, use the -s flag to display the labels in the output. For example:

```bash
timo list
timo list -l work
```

### Show Help

```bash
timo help
```

## Contributing

Contributions to Timo are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request on the GitHub repository.

## License

Timo is released under the MIT License.
