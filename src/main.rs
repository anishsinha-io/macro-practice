use macros::Builder;

#[derive(Debug, Builder)]
pub struct Command {
    pub command: String,
    #[builder(each = "arg")]
    pub args: Vec<String>,
    pub b: Option<Vec<String>>,
    #[builder(each = "env")]
    pub environment: Vec<String>,
}

fn main() {
    let cmd = Command::builder()
        .command("cargo".into())
        .b(vec!["".into()])
        .environment(vec![])
        .args(vec![])
        .build()
        .unwrap();

    dbg!(cmd);
}
