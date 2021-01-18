use core::time::Duration;
use crossterm::{cursor::Show, event::{self, Event, KeyCode}};
use crossterm::{ExecutableCommand, cursor::Hide, terminal};
use invaders::{frame, render};
use rusty_audio::Audio;
use terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use std::{error::Error, io, sync::mpsc, thread};

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("move", "move.wav");
    audio.add("pew", "pew.wav");
    audio.add("startup", "startup.wav");
    audio.add("win", "win.wav");
    audio.play("startup");

    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handler = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut imitate_curr_frame = last_frame.clone();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &mut last_frame, &mut imitate_curr_frame, true);
        loop {
            let mut curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &mut last_frame, &mut curr_frame, false);
            last_frame = curr_frame;
        }
        
    });

    // Game loop
    'gameloop: loop {
        // Per frame init
        let curr_frame = frame::new_frame();
        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    },
                    _ => {},
                }

            }
        }
        // Draw and Render
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }


    drop(render_tx);
    render_handler.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
