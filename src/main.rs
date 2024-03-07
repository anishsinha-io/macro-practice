use macros::Builder;

#[derive(Debug, Builder)]
pub struct Command {
    pub command: String,
    pub args: Vec<String>,
    pub a: Option<String>,
}

fn main() {
    let cmd = Command::builder()
        .command("cargo".into())
        .args(vec!["test".into(), "--".into(), "--nocapture".into()])
        .a(None)
        .build()
        .unwrap();

    dbg!(cmd);
}
