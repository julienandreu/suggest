use copypasta::{ClipboardContext, ClipboardProvider};
use spinners::{Spinner, Spinners};
use suggest::{
    context::Context,
    git::{get_commit_id, get_diff},
    llm::run,
};

fn main() {
    let mut clipboard_ctx = ClipboardContext::new().unwrap();
    let context = Context::load();
    let commit_id = get_commit_id(std::env::args().last());
    let diff = get_diff(commit_id);

    let mut sp = Spinner::new(Spinners::Dots, "Analyzing staged changes...".into());

    // Generate description
    let description_prompt = format!(
        r"Based on the context of the application: {:?}. Describe what's happening in my current change from the follow 'git diff --staged':\n\n ---START OF THE GIT-DIFF:\n{}\n ---END OF THE GIT-DIFF",
        serde_json::to_string(&context.unwrap_or_default()),
        diff.unwrap_or_default()
    );
    let description = run(&description_prompt).unwrap_or_default();

    // Generate git commit command
    let commit_prompt = format!(
        r"I want you to act as a commit message generator. I will provide you with information about the changes summary, and I would like you to generate an appropriate commit message using the conventional commit format. I would like you to generate the prefix and the commit message with multi-paragraph body. Do not write any explanations or other words, just reply with the 'git commit command' with the associated message and double-quote escaped multiline commit message body.\n\n ---START OF THE DESCRIPTION:\n{}\n ---END OF THE DESCRIPTION",
        description
    );
    let commit = run(&commit_prompt).unwrap_or_default();
    clipboard_ctx.set_contents(commit.to_owned()).unwrap();

    sp.stop_with_message(format!("Done in {}ms!", 12));

    println!("{}", commit);
}
