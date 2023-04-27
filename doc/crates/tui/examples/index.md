# tui crate - examples

[Runnable project](/projects/crates/tui/terminal_draw_block)

Example to draw a block on the screen:

```
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::widgets::{Borders, Block};
use termion::raw::IntoRawMode;

fn main() {
    let stdout = std::io::stdout().into_raw_mode().unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    // Draw the UI
    terminal.clear();
    terminal.draw(|f| {
        let size = Rect { x: 8, y: 8, width: 8, height: 8 };
        let block = Block::default()
            .title("Block")
            .borders(Borders::ALL);
        f.render_widget(block, size);
    }).unwrap();
}
```
