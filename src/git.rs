use std::{env, io::Error, process::Command};

pub fn get_commit_id(input: Option<String>) -> Option<String> {
    if let Some(input) = input {
        let output = Command::new("git")
            .current_dir(env::current_dir().unwrap())
            .args(["cat-file", "-t", &input])
            .output()
            .expect("Failed to execute git cat-file");

        if output.status.success() && String::from_utf8_lossy(&output.stdout).contains("commit") {
            return Some(input);
        }
    }

    None
}

pub fn get_diff(input: Option<String>) -> Result<String, Error> {
    let output = match input {
        Some(input) => {
            let input_minus_one = format!("{}~1", &input);
            Command::new("git")
                .current_dir(env::current_dir().unwrap())
                .args(["diff", &input_minus_one, &input])
                .output()
                .expect("Failed to execute git show")
        }
        None => Command::new("git")
            .current_dir(env::current_dir().unwrap())
            .args(["diff", "--cached"])
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
