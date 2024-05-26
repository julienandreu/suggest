use std::{env, io::Error, process::Command};

pub fn get_commit_id(input: Option<String>) -> Option<String> {
    if let Some(input) = input {
        let output = Command::new("git")
            .current_dir(env::current_dir().unwrap())
            .args(&["cat-file", "-t", &input])
            .output()
            .expect("Failed to execute git cat-file");

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("{:?}", stdout);

            if stdout.contains("commit") {
                return Some(input);
            }
        }
    }

    None
}

pub fn get_diff(input: Option<String>) -> Result<String, Error> {
    let output = match input {
        Some(input) => Command::new("git")
            .current_dir(env::current_dir().unwrap())
            .args(&["show", &input])
            .output()
            .expect("Failed to execute git show"),
        None => Command::new("git")
            .current_dir(env::current_dir().unwrap())
            .args(&["diff", "HEAD", "HEAD~1"])
            .output()
            .expect("Failed to execute git diff"),
    };

    if output.status.success() {
        return Ok(String::from_utf8_lossy(&output.stdout).into_owned());
    }

    Err(Error::new(
        std::io::ErrorKind::Other,
        String::from_utf8_lossy(&output.stderr).into_owned(),
    ))
}
