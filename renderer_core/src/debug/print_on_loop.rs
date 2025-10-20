use std::{fmt::Arguments, io::{stdout, Write}, sync::{Arc, LazyLock, Mutex}};
use crossterm::{cursor, terminal::{Clear, ClearType}, QueueableCommand};
pub struct PrintOnLoop {
    lines:u16
}

impl PrintOnLoop {
    fn new() -> Self {
        Self { lines: 0 }
    }
    pub fn println(&mut self, msg:Arguments<'_>) -> std::io::Result<()> {
        let msg= msg.to_string();
        stdout().write(msg.as_bytes())?;
        stdout().write("\n".as_bytes())?;
        self.lines += 1;
        Ok(())
    }
    pub fn flush(&mut self) -> std::io::Result<()> {
        let mut stdout = stdout();
        stdout.flush()?;
        stdout.queue(cursor::MoveToPreviousLine(self.lines))?;
        stdout.queue(Clear(ClearType::FromCursorDown))?;
        self.lines = 0;
        Ok(())
    } 
}

pub static LOGGER_ON_LOOP: LazyLock<Arc<Mutex<PrintOnLoop>>> =
    LazyLock::new( || 
    Arc::new(
        Mutex::new(PrintOnLoop::new())
    )
);