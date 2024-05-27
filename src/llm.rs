use std::process::Command;

pub fn run(input: &str) -> Option<String> {
    let escaped_input = format!("\"{}\"", input);
    let output = Command::new("ollama")
        .args(&["run", "llama3", &escaped_input])
        .output()
        .expect("Failed to execute ollama");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);

        return Some(String::from(stdout));
    }

    None
}
