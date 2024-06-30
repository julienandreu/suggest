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
    let model = std::env::args().nth(1).unwrap_or("llama3".to_owned());

    let mut clipboard_ctx = ClipboardContext::new().unwrap();
    let context = Context::load();
    let commit_id = get_commit_id(std::env::args().last());
    let diff = get_diff(commit_id);

    let mut sp = Spinner::new(
        Spinners::Dots,
        format!("Analyzing staged changes using {}...", model),
    );

    // Generate description
    let description_prompt = format!(
        r"Based on the context of the application: {},
        Describe the changes in my current commit from the following
        'git diff --staged' output.
        Provide a detailed description that summarizes what the changes do and
        why they were made.
        Prioritize what seems to have an impact and what is not important.
        The description should highlight the main/core changes to be used
        in a git commit message after this.

---START OF THE GIT-DIFF---
{}
---END OF THE GIT-DIFF---",
        serde_json::to_string(&context.unwrap_or_default()).unwrap(),
        diff.unwrap_or_default()
    );
    let description = run(&model, &description_prompt).unwrap_or_default();

    let duration = start.elapsed();
    sp.stop_with_message(format!("Done in {:?}!", duration));

    let mut sp = Spinner::new(
        Spinners::Dots,
        format!("Generating commit message using {}...", model),
    );

    // Generate git commit command
    let commit_prompt = format!(
        r"Based on the following description summary, please write the ideal
        git commit command that includes all staged changes.
        The commit message should adhere to the Conventional Commits
        specification outlined here:
        https://www.conventionalcommits.org/en/v1.0.0/#specification

        Ensure the commit message is human-readable and provides an exhaustive
        description of the changes made.
        Providing a clear and descriptive commit message is crucial for
        maintaining project history and facilitating collaboration.

        This message should be correctly escaped so I can use it directly in
        my terminal.

        It should also highlight the most important changes and be exhaustive
        as well, so I could see really quickly what is onboarded in the changes
        and I could then generate proper Changelog files based on them.

---START OF THE GIT-DIFF SUMMARY DESCRIPTION---
{}
---END OF THE GIT-DIFF SUMMARY DESCRIPTION---",
        description
    );
    let commit = run(&model, &commit_prompt).unwrap_or_default();
    clipboard_ctx
        .set_contents(commit_prompt.to_owned())
        .unwrap();

    let duration = start.elapsed();
    sp.stop_with_message(format!("Done in {:?}!", duration));

    println!("{}", commit);
}
