use ropey::Rope;

#[derive(Clone, Debug, Default)]
pub struct Backend {
    pub rope: Rope, 
    pub cursor: (u16, u16),
}

impl Backend {
    pub fn command(&mut self, command: Command) {
        match command {
            Command::Edit(edit) => {
                match edit.action {
                    Action::Insert(text) => {
                        self.rope.insert(0, &text);
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }

    pub fn highlight(&self) {

    }
}

pub enum Command {
    Edit(Edit),
    Repeat,
    Scroll{line: u16, top_offset: u8},
}

pub struct Edit {
    pub action: Action,
    pub movement: Movement,
    pub count: u16,
}

pub enum Action {
    Cut,
    Yank,
    Paste,
    Jump,
    Indent,
    Unindent,
    Append(String),
    Insert(String),
    Replace(String),
}

pub enum Movement {
    // w, W
    Word{big: bool},

    // e, E
    End{big: bool},

    // b, B
    Back{big: bool},

    // /search ?search fc tc Fc Tc
    Search{text: String, forward: bool, through: bool},

    // n, ;
    NextResult,

    // hjkl 
    Adjacent(Direction),

    // %
    Match,

    // dd, yy, etc
    Line,

    // $
    EndOfLine,

    // 0
    StartOfLine,

    // ^
    FirstNonBlank,

    // gg, G
    FirstLine,
    LastLine,

    Selection,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
