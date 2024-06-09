# Timo
Timo is a command-line application that allows you to capture and manage your thoughts and ideas directly from your terminal. It provides a simple and efficient way to jot down your thoughts, search through them, and keep them organized.

## Features
- **Add Thoughts**: Easily add new thoughts to the list with a single command.
- **Clear Thoughts**: Remove all thoughts from the list with a single command.
- **Remove Thoughts**: Remove specific thoughts from the list by providing their indexes.
- **Search Thoughts**: Search for thoughts in the list by providing a keyword or phrase.
- **List Thoughts**: Print all thoughts in the list to the terminal.
- **Persistent Storage**: Thoughts are stored in a local file for future reference.

## Installation
To use Timo, you can clone the repository locally and build it using Rust and Cargo. Here are the steps:

1. Clone the Timo repository:
   ```bash
   git clone https://github.com/abhishek6262/timo
   ```

2. Navigate to the project directory:
   
   ```bash
   cd timo
   ```

3. You can then install Timo globally using `cargo install --path .` command.


## Usage
After installing Timo, you can use the following commands:

### Add a New Thought
```bash
timo add <thought>
```

Replace `<thought>` with your thought. Multiple words will be combined into a single thought. For example:

```bash
timo add Hack a new idea!
```

### Clear All Thoughts
```bash
timo clear
```

This command will remove all thoughts from the list.

### Remove Specific Thoughts
```bash
timo remove <index1> <index2> ...
```

Replace `<index1>`, `<index2>`, etc., with the indexes of the thoughts you want to remove. The indexes are displayed when you list the thoughts.

### Search for Thoughts
```bash
timo search <keyword>
```

Replace `<keyword>` with the word or phrase you want to search for. Timo will display all thoughts that contain the specified keyword or phrase.

### List All Thoughts
```bash
timo list
```

This command will print all thoughts in the list, along with their indexes.

## Contributing
Contributions to Timo are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request on the GitHub repository.

## License
Timo is released under the MIT License.

