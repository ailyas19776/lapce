//handling delete keyword
//use std::cmp;
use unicode_segmentation::UnicodeSegmentation;
//use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{RawTerminal};

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    cursor_position: Position,
    document: Document,
}
impl Editor {
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Delete => self.document.delete(&self.cursor_position),/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            
            _ => (),
        }
        Ok(())
    }
}


#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub file_name: Option<String>,
}

impl Document {
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn delete(&mut self, at: &Position) {///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        if at.y >= self.len() {
            return;
        }
        let row = self.rows.get_mut(at.y).unwrap();
        row.delete(at.x);
    }
}


#[derive(Default)]
pub struct Row {
    string: String,
    len: usize,
}

impl Row {
    pub fn len(&self) -> usize {
        self.len
    }
    fn update_len(&mut self) {
        self.len = self.string[..].graphemes(true).count();
    }
    pub fn delete(&mut self, at: usize) {///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        if at >= self.len() {
            return;
        } else {
            let mut result: String = self.string[..].graphemes(true).take(at).collect();
            let remainder: String = self.string[..].graphemes(true).skip(at + 1).collect();
            result.push_str(&remainder);
            self.string = result;
        }
        self.update_len();
    }
}


pub struct Terminal {
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
pub fn read_key() -> Result<Key, std::io::Error> {////////////////////////////////////////////////////////////////////////////////////////////////////////
        loop {
            if let Some(key) = std::io::stdin().lock().keys().next() {
                return key;
            }
        }
	}

}

