use copypasta::{ClipboardContext, ClipboardProvider};
use suggest::{
    context::Context,
    git::{get_commit_id, get_diff},
};

fn main() {
    let mut clipboard_ctx = ClipboardContext::new().unwrap();

    let context = Context::load();

    let commit_id = get_commit_id(std::env::args().last());

    let diff = get_diff(commit_id);

    let prompt = format!(
      "Based on the shortened package.json context provided below: {:?}\nand the following git diff summary: {}\nPlease write the ideal git commit command that includes all staged changes.The commit message should adhere to the Conventional Commits specification outlined here:\nhttps://www.conventionalcommits.org/en/v1.0.0/#specification\nEnsure the commit message is human-readable and provides an exhaustive description of the changes made.\nProviding a clear and descriptive commit message is crucial for maintaining project history and facilitating collaboration.",
      serde_json::to_string(&context.unwrap_or_default()),
      diff.unwrap_or_default()
    );

    clipboard_ctx.set_contents(prompt.to_owned()).unwrap();

    println!("{}", prompt);
}
