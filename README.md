# Meow Box Telnet Client

A Rust-based Telnet client designed for the Meow box challenge on Hack The Box. This tool demonstrates basic Telnet interaction and can be used for educational purposes in penetration testing.

## Features

- Automated Telnet connection and authentication
- Comprehensive system enumeration
- Multiple flag location checks
- Interesting file discovery
- Network and process information gathering
- Robust error handling and connection verification
- Buffer management for large responses
- Command retry mechanism

## Prerequisites

- Rust installed on your system
- Network access to the target machine
- Basic understanding of penetration testing concepts

## Installation

1. Clone the repository:
```bash
git clone https://github.com/ArsCodeAmatoria/archon.git
cd archon
```

2. Compile the program:
```bash
rustc main.rs
```

## Usage

Run the program with target IP and port:
```bash
./main <target_ip> <port>
```

Example:
```bash
./main 10.10.10.10 23
```

## Features Breakdown

### System Enumeration
- User information (`whoami`, `id`)
- System information (`uname -a`, `cat /proc/version`)
- Directory listings (`ls -la /`, `ls -la /root`)
- Network information (`netstat -tuln`, `ifconfig`)
- Process information (`ps aux`)
- System files (`/etc/passwd`, `/etc/shadow`)

### Flag Search
Automatically checks multiple locations:
- `/flag.txt`
- `/root/flag.txt`
- `/root/root.txt`
- `/home/user/flag.txt`
- And more backup/alternative locations

### File Discovery
Searches for files matching patterns:
- `*flag*`
- `*root*`
- `*user*`
- `*.txt`

### Interesting Files
Checks common system and user files:
- `/etc/passwd`
- `/etc/shadow`
- `/etc/hosts`
- `/etc/issue`
- `/etc/motd`
- `/root/.bash_history`
- `/root/.ssh/authorized_keys`

## Error Handling

The program includes comprehensive error handling for:
- Connection failures
- Read/write timeouts
- Invalid responses
- Network issues
- Connection verification
- Command execution failures

## Security Note

This tool is intended for educational purposes and should only be used on systems you have permission to test. Always ensure you have proper authorization before using this tool.

## License

This project is open source and available under the MIT License. 