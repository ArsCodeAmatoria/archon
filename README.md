# Meow Box Telnet Client

A Rust-based Telnet client designed for the Meow box challenge on Hack The Box. This tool demonstrates basic Telnet interaction and can be used for educational purposes in penetration testing.

## Features

- Connects to Telnet services
- Handles connection timeouts
- Attempts unauthenticated login
- Executes basic commands
- Graceful error handling
- Buffer management for responses

## Prerequisites

- Rust installed on your system
- Network access to the target machine

## Usage

1. Update the target IP in `main.rs`:
```rust
let target = "10.10.10.10:23"; // Replace with actual target IP
```

2. Compile the program:
```bash
rustc main.rs
```

3. Run the program:
```bash
./main
```

## Code Structure

The program follows this sequence:
1. Establishes connection to target
2. Reads and displays Telnet banner
3. Attempts login with empty credentials
4. Executes basic commands
5. Attempts to read flag file

## Error Handling

The program includes comprehensive error handling for:
- Connection failures
- Read/write timeouts
- Invalid responses
- Network issues

## Security Note

This tool is intended for educational purposes and should only be used on systems you have permission to test. Always ensure you have proper authorization before using this tool.

## License

This project is open source and available under the MIT License. 