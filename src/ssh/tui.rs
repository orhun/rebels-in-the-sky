use crate::app::App;
use crate::ssh::backend::SSHBackend;
use crate::types::AppResult;
use crate::ui::ui::Ui;
use crate::world::world::World;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::Terminal;

use super::event::TickEventHandler;

pub struct SSHTui {
    pub terminal: Terminal<SSHBackend>,
    pub events: TickEventHandler,
}

impl SSHTui {
    /// Constructs a new instance of [`SSHTui`].
    pub fn new(backend: SSHBackend, events: TickEventHandler) -> AppResult<Self> {
        let terminal = Terminal::new(backend)?;
        let mut tui = Self { terminal, events };
        tui.init()?;

        Ok(tui)
    }

    /// Initializes the terminal interface.
    ///
    /// It enables the raw mode and sets terminal properties.
    fn init(&mut self) -> AppResult<()> {
        crossterm::execute!(
            self.terminal.backend_mut(),
            EnterAlternateScreen,
            EnableMouseCapture
        )?;

        self.terminal.clear()?;
        Ok(())
    }

    pub fn draw(&mut self, ui: &mut Ui, world: &World) -> AppResult<()> {
        self.terminal.draw(|frame| App::render(ui, world, frame))?;
        Ok(())
    }

    /// Resizes the terminal interface.
    pub fn resize(&mut self, width: u16, height: u16) -> AppResult<()> {
        self.terminal.backend_mut().size = (width, height);
        self.terminal.clear()?;
        Ok(())
    }

    /// Resets the terminal interface.
    fn reset(&mut self) -> AppResult<()> {
        crossterm::execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.terminal.clear()?;
        self.terminal.show_cursor()?;

        Ok(())
    }

    /// Exits the terminal interface.
    ///
    /// It disables the raw mode and reverts back the terminal properties.
    pub async fn exit(&mut self) -> AppResult<()> {
        self.reset()?;
        self.terminal.backend().close().await
    }
}
