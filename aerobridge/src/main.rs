use aerobridge::app::{App, AppResult};
use aerobridge::event::{Event, EventHandler};
use aerobridge::handler::handle_key_events;
use aerobridge::rpc;
use aerobridge::tui::Tui;
use core::time;
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use tokio::sync::mpsc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[tokio::main]
async fn main() -> AppResult<()> {
    let (tx, mut rx) = mpsc::unbounded_channel();
    let tx2 = tx.clone();

    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Create a shared flag to signal the RPC thread to stop
    let running = Arc::new(AtomicBool::new(true));
    let running_rpc = running.clone();

    // Spawn RPC task in a separate thread
    let rpc_handle = std::thread::spawn(move || {
        rpc::run(tx2, running_rpc)
    });

    // Start the main loop.
    'main: loop {
        // Render the user interface.
        tui.draw(&mut app)?;

        // Handle events.
        tokio::select! {
            Some(message) = rx.recv() => {
                app.update_message(message);
            }
            Ok(event) = tui.events.next() => {
                match event {
                    Event::Tick => app.tick(),
                    Event::Key(key_event) => {
                        if handle_key_events(key_event, &mut app)? {
                            break 'main;
                        }
                    }
                    Event::Mouse(_) => {}
                    Event::Resize(_, _) => {}
                }
            }
            else => break,
        }

        if !app.running {
            break;
        }
    }

    // Exit the user interface.
    tui.exit()?;

    // Signal shutdown
    running.store(false, Ordering::SeqCst);

    // Wait for RPC thread to complete
    if let Err(e) = rpc_handle.join() {
        eprintln!("RPC thread error: {:?}", e);
    }

    Ok(())
}