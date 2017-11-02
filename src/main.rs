#[macro_use]
extern crate log;
extern crate neovim_lib;
extern crate simplelog;

mod args;
mod handler;
mod event;
mod position;

use handler::NeovimHandler;
use event::Event;

use std::error::Error;
use std::sync::mpsc;

use neovim_lib::neovim::Neovim;
use neovim_lib::neovim_api::NeovimApi;
use neovim_lib::session::Session;

use simplelog::{Config, LogLevel, LogLevelFilter, WriteLogger};


fn main() {
    use std::process;

    init_logging().expect("nvimpam: unable to initialize logger.");

    match start_program() {
        Ok(_) => process::exit(0),

        Err(msg) => {
            error!("{}", msg);
            process::exit(1);
        }
    };
}

fn init_logging() -> Result<(), Box<Error>> {
    use std::env;
    use std::env::VarError;
    use std::fs::File;

    let log_level_filter = match env::var("LOG_LEVEL")
        .unwrap_or_else(|_| String::from("trace"))
        .to_lowercase()
        .as_ref() {
        "debug" => LogLevelFilter::Debug,
        "error" => LogLevelFilter::Error,
        "info" => LogLevelFilter::Info,
        "trace" => LogLevelFilter::Trace,
        "warn" => LogLevelFilter::Warn,
        _ => LogLevelFilter::Off,
    };

    let config = Config {
        time: Some(LogLevel::Error),
        level: Some(LogLevel::Error),
        target: Some(LogLevel::Error),
        location: Some(LogLevel::Error),
    };

    let filepath = match env::var("LOG_FILE") {
        Err(err) => {
            match err {
                VarError::NotPresent => return Ok(()),
                e @ VarError::NotUnicode(_) => {
                    return Err(Box::new(e));
                }
            }
        }
        Ok(path) => path.to_owned(),
    };

    let log_file = File::create(filepath)?;

    WriteLogger::init(log_level_filter, config, log_file).unwrap();

    Ok(())
}

fn start_program() -> Result<(), Box<Error>> {
    info!("connecting to neovim via stdin/stdout");

    let (sender, receiver) = mpsc::channel();
    let mut session = try!(Session::new_parent());
    session.start_event_loop_handler(NeovimHandler(sender));

    let mut nvim = Neovim::new(session);

    info!("let's notify neovim the plugin is connected!");
    nvim.command("echom \"rust client connected to neovim\"")
        .unwrap();
    info!("notification complete!");

    nvim.subscribe("LiveUpdateStart").expect(
        "error: cannot subscribe to event: LiveUpdateStart",
    );
    nvim.subscribe("LiveUpdate").expect(
        "error: cannot subscribe to event: LiveUpdate",
    );
    nvim.subscribe("LiveUpdateTick").expect(
        "error: cannot subscribe to event: LiveUpdateTick",
    );
    nvim.subscribe("LiveUpdateEnd").expect(
        "error: cannot subscribe to event: LiveUpdateEnd",
    );
    nvim.subscribe("quit").expect(
        "error: cannot subscribe to event: quit",
    );

    start_event_loop(&receiver, nvim);

    Ok(())
}

fn makeafold(nvim: &mut Neovim, lines: &Vec<String>) {
    nvim.command("normal! zE").unwrap();

    let mut firstline: u64 = 0;
    let mut lastline: u64 = 0;
    for (i, ref line) in lines.iter().enumerate() {
        if line.starts_with("NODE") && firstline == 0 {
            firstline = i as u64;
        }
        if !line.starts_with("N") && firstline > 0 {
            lastline = i as u64;
        }
        if firstline > 0 && lastline > 0 {
            nvim.command(&format!("{},{}fo", firstline + 1, lastline)).unwrap();
            firstline = 0;
            lastline = 0;
        }
    }

    nvim.command("silent! qa").unwrap();
}

fn start_event_loop(receiver: &mpsc::Receiver<Event>, mut nvim: Neovim) {
    use std::process;
    let curbuf = nvim.get_current_buf().unwrap();
    debug!("Before call");
    match curbuf.live_updates(&mut nvim, true) {
        Ok(_) => {},
        Err(e) => {
            error!("{:?}", e);
            process::exit(1);
        }
    }
    debug!("after call");
        
    loop {
        match receiver.recv() {
            Ok(Event::LiveUpdateStart { ref linedata, .. }) => makeafold(&mut nvim, linedata),
            Ok(Event::Quit) => break,
            _ => {}
        }
    }
    info!("quitting");
    nvim.command("echom \"rust client disconnected from neovim\"")
        .unwrap();

}
