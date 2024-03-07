use macros::Builder;

#[derive(Debug, Builder)]
pub struct Command {
    pub command: String,
    pub args: Vec<String>,
    pub a: Option<Vec<String>>,
}

fn main() {
    let cmd = Command::builder()
        .command("cargo".into())
        .args(vec!["test".into(), "--".into(), "--nocapture".into()])
        .a(vec!["".into()])
        .build()
        .unwrap();

    dbg!(cmd);
}
