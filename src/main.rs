use ratatui::crossterm::event;

mod grid;
use grid::Grid;

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;

    let tui_size = terminal.size()?;
    let mut grid = Grid::new(tui_size.width, tui_size.height);

    loop {
        terminal.draw(|frame| {
            frame.render_widget(&grid, frame.area());
        })?;

        let new_event = event::poll(std::time::Duration::from_millis(2)).unwrap();
        if new_event {
            break;
        }
        grid.generate();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    ratatui::restore();
    Ok(())
}
