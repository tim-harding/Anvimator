mod color;

use ropey::Rope;

use syntect::{easy::HighlightLines, highlighting::ThemeSet, parsing::SyntaxSet};

use anyhow::Error;

pub use color::{HexRgba, Nord};

#[derive(Debug)]
pub struct Backend {
    pub rope: Rope,
    pub cursor: (u16, u16),
    themes: ThemeSet,
    syntaxes: SyntaxSet,
}

pub struct ColoredText {
    pub text: String,
    pub color: HexRgba,
}

impl Backend {
    pub fn new() -> Result<Self, Error> {
        let themes = ThemeSet::load_from_folder("D:\\20\\12\\anvimator\\rendering\\src\\assets")?;
        let syntaxes = SyntaxSet::load_defaults_newlines();
        Ok(Self {
            rope: Rope::default(),
            cursor: (0, 0),
            themes,
            syntaxes,
        })
    }

    pub fn command(&mut self, command: Command) {
        match command {
            Command::Edit(edit) => match edit.action {
                Action::Insert(text) => {
                    self.rope.insert(0, &text);
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn highlight(&self) -> Vec<ColoredText> {
        let syntax = self.syntaxes.find_syntax_by_extension("rs").unwrap();
        let theme = &self.themes.themes["Nord"];
        let mut highlighter = HighlightLines::new(syntax, theme);
        let texts: Vec<_> = self
            .rope
            .lines()
            .flat_map(|line| {
                let ranges = highlighter.highlight(line.as_str().unwrap_or("\n"), &self.syntaxes);
                let texts: Vec<_> = ranges
                    .into_iter()
                    .map(|(style, token)| {
                        let color: HexRgba = style.foreground.into();
                        ColoredText {
                            text: token.to_string(),
                            color,
                        }
                    })
                    .collect();
                texts.into_iter()
            })
            .collect();
        texts
    }
}

pub enum Command {
    Edit(Edit),
    Repeat,
    Scroll { line: u16, top_offset: u8 },
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
    Word {
        big: bool,
    },

    // e, E
    End {
        big: bool,
    },

    // b, B
    Back {
        big: bool,
    },

    // /search ?search fc tc Fc Tc
    Search {
        text: String,
        forward: bool,
        through: bool,
    },

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
