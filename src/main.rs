use copypasta::{ClipboardContext, ClipboardProvider};
use spinners::{Spinner, Spinners};
use std::time::Instant;
use suggest::{
    context::Context,
    git::{get_commit_id, get_diff},
    llm::run,
};

fn main() {
    let start = Instant::now();

    let mut clipboard_ctx = ClipboardContext::new().unwrap();
    let context = Context::load();
    let commit_id = get_commit_id(std::env::args().last());
    let diff = get_diff(commit_id);

    let mut sp = Spinner::new(Spinners::Dots, "Analyzing staged changes...".into());

    // Generate description
    let description_prompt = format!(
        r"Based on the context of the application: {}, describe the changes in my current commit from the following 'git diff --staged' output. Provide a detailed description that summarizes what the changes do and why they were made.

---START OF THE GIT-DIFF---
{}
---END OF THE GIT-DIFF---",
        serde_json::to_string(&context.unwrap_or_default()).unwrap(),
        diff.unwrap_or_default()
    );
    let description = run(&description_prompt).unwrap_or_default();

    // Generate git commit command
    let commit_prompt = format!(
        "I want you to act as a commit message generator. Based on the provided summary of the changes, generate a commit message in the conventional commit format. The commit message should include a detailed multi-paragraph body. Format the message so it can be used directly in a 'git commit -m' shell command, with appropriate escaping for double quotes and new lines. Ensure the output is a single line with '\\\"' for double quotes, '\\n' for new lines '\\\'' for backticks '`'. Do not include any explanations!

---START OF THE DESCRIPTION---
{}
---END OF THE DESCRIPTION---",
        description
    );
    let commit = run(&commit_prompt).unwrap_or_default();
    clipboard_ctx
        .set_contents(commit_prompt.to_owned())
        .unwrap();

    let duration = start.elapsed();
    sp.stop_with_message(format!("Done in {:?}!", duration));

    println!("{}", commit);
}
