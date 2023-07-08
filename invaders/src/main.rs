use std::error::Error;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use crossterm::event::{Event, KeyCode};
use invaders::frame::{new_frame, Drawable};
use invaders::invaders::Invaders;
use invaders::player::Player;
use invaders::render::render;
use rusty_audio::Audio;
use std::{io, thread};
use crossterm::{terminal, event};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::cursor::{Show, Hide};
use crossterm::ExecutableCommand;

fn main() -> Result <(), Box<dyn Error>> {
    let mut audio = Audio::new();

    audio.add("explode", "sounds/explode.wav");
    audio.add("lose", "sounds/lose.wav");
    audio.add("move", "sounds/move.wav");
    audio.add("pew", "sounds/pew.wav");
    audio.add("startup", "sounds/startup.wav");
    audio.add("win", "sounds/win.wav");

    audio.play("startup");

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?; // ? -> Crash if something is wrong
    stdout.execute(EnterAlternateScreen)?; // Enter in an alternative screen while playing the game
    stdout.execute(Hide)?; // Hide cursor

    // Render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();

    let render_handle = thread::spawn(move || {
        let mut last_frame = new_frame();
        let mut stdout = io::stdout();
        render(&mut stdout, &last_frame, &last_frame, true);

        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();

    // Game loop
    'gameloop: loop {
        let delta = instant.elapsed();
        instant = Instant::now();

        let mut curr_frame = new_frame();

        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    },
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }
                    _ => {}
                }
            }
        }

        // Updates
        player.update(delta);

        if invaders.update(delta) {
            audio.play("move");
        }

        if player.detect_hits(&mut invaders) {
            audio.play("explode");
        }

        // Draw & render
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];

        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1)); // We put a sleep here as the game loop is much faster than rendering

        // Win or lose
        if invaders.all_killed() {
            audio.play("win");
            break 'gameloop;
        }
        
        if invaders.reached_bottom() {
            audio.play("lose");
            break 'gameloop;
        }
    }


    // Cleanup
    drop(render_tx); // Remove transmitting channel, this will trigger an error on the receiver causing the loop to break
    render_handle.join().unwrap();

    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())

}
