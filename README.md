# SHA1 Password Cracker

A terminal-based SHA1 password cracking tool with a modern TUI interface built in Rust.

## Features

- **Interactive TUI Interface**: User-friendly terminal interface built with Ratatui
- **Real-time Progress Tracking**: Monitor cracking progress with live updates
- **Log System**: Detailed logging of the cracking process
- **Wordlist Support**: Process standard wordlist files
- **Efficient**: Multi-threaded processing with progress reporting

## Requirements

- Rust 1.70 or higher
- A wordlist file (default: `wordlist.txt`)

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd sha1-cracker
```

2. Build the project:
```bash
cargo build --release
```

## Usage

1. Prepare a wordlist file named `wordlist.txt` in the same directory, or specify a different path in the application.

2. Run the application:
```bash
cargo run
```

3. Using the interface:
   - **Tab**: Navigate between fields (Wordlist path → Hash → Start button)
   - **Enter**: 
     - Select field or start cracking when on the start button
     - Move to next field when on text inputs
   - **Backspace**: Delete characters in text fields
   - **Ctrl+C** or **Esc**: Exit the application

### Interface Components

- **Wordlist Path**: Path to your wordlist file (default: `wordlist.txt`)
- **Hash Input**: Target SHA1 hash to crack (40 hexadecimal characters)
- **Start Button**: Begin the cracking process
- **Progress Bar**: Visual indicator of cracking progress
- **Log Panel**: Real-time logs and status messages

## How It Works

1. The application reads a wordlist file line by line
2. For each password candidate, it computes the SHA1 hash
3. Compares the computed hash with the target hash
4. Reports progress every 1000 lines processed
5. Stops and displays the password when a match is found

## Example

```bash
# Target hash: 5baa61e4c9b93f3f0682250b6cf8331b7ee68fd8 (password: "password")
# Wordlist containing common passwords

# Run the application, enter the hash, and start cracking
```

## Project Structure

```
src/
├── main.rs      # Application entry point and event loop
├── app.rs       # Core application logic and state management
└── ui.rs        # Terminal user interface components
```

## Dependencies

- `ratatui`: Modern TUI framework
- `crossterm`: Cross-platform terminal manipulation
- `sha1`: SHA1 hashing functionality
- `hex`: Hexadecimal encoding/decoding

## Performance

- Processes approximately 1000 passwords per progress update
- Efficient memory usage with bounded log storage
- Multi-threaded design prevents UI blocking

## Limitations

- Only supports SHA1 hashes
- Requires pre-existing wordlists
- Single-threaded cracking (but with non-blocking UI)

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for bugs and feature requests.

## License

[Add your license here]
