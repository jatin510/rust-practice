enum Command {
    Undo,
    Redo,
    AddText(String),
    MoveCursor(i32, i32),
    Replace { from: String, to: String },
}

impl Command {
    fn serialize(&self) -> String {
        String::from("dummy string")
    }
}

fn main() {
    let cmd = Command::Undo;
    let cmd = Command::Redo;
    let cmd = Command::AddText(String::from("Hello"));
    let cmd = Command::MoveCursor(0, 0);
    let cmd = Command::Replace {
        from: String::from("Hello"),
        to: String::from("hello"),
    };

    let json_string = cmd.serialize();
    println!("{}", json_string);
}
