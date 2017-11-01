extern crate neovim_lib;
extern crate rmp;
extern crate tempdir;

use neovim_lib::session::Session;
use neovim_lib::neovim::Neovim;
use neovim_lib::neovim_api::NeovimApi;

#[cfg(unix)]
use std::process::Command;
#[cfg(unix)]
use tempdir::TempDir;

#[ignore]
#[test]
fn start_stop_test() {
    let mut session = if cfg!(target_os = "windows") {
        Session::new_child_path("E:\\Neovim\\bin\\nvim.exe").unwrap()
    } else {
        Session::new_child().unwrap()
    };

    session.start_event_loop();

    let mut nvim = Neovim::new(session);
    println!("{:?}", nvim.get_api_info().unwrap());
}

#[ignore]
#[test]
fn remote_test() {
    let mut session = Session::new_tcp("127.0.0.1:6666").unwrap();
    session.start_event_loop();
    let mut nvim = Neovim::new(session);
    nvim.command("echo \"Test\"").unwrap();
}

#[ignore]
#[test]
fn edit_test() {
    let mut session = Session::new_tcp("127.0.0.1:6666").unwrap();
    session.start_event_loop();
    let mut nvim = Neovim::new(session);
    let buffers = nvim.list_bufs().unwrap();
    buffers[0].set_lines(&mut nvim, 0, 0, true, vec!["replace first line".to_owned()]).unwrap();
    nvim.command("vsplit").unwrap();
    let windows = nvim.list_wins().unwrap();
    windows[0].set_width(&mut nvim, 10).unwrap();
}

#[cfg(unix)]
#[ignore]
#[test]
fn can_connect_via_unix_socket() {
    use std::path::Path;
    use std::thread::sleep;
    use std::time::{Duration, Instant};

    let dir = TempDir::new("neovim-lib.test").expect("Cannot create temporary directory for test.");

    let socket_path = dir.path().join("unix_socket");

    let _child = Command::new("nvim")
        .arg("--embed")
        .env("NVIM_LISTEN_ADDRESS", &socket_path)
        .spawn()
        .expect("Cannot start neovim");

    // wait at least 1 second for neovim to start and create socket path.
    {
        let start = Instant::now();
        let one_second = Duration::from_secs(1);
        loop {
            sleep(Duration::from_millis(100));

            if let Ok(_) = std::fs::metadata(&socket_path) {
                break;
            }

            if one_second <= start.elapsed() {
                panic!(format!("neovim socket not found at '{:?}'", &socket_path));
            }
        }
    }


    let mut session = Session::new_unix_socket(&socket_path)
        .expect(&format!("Unable to connect to neovim's unix socket at {:?}",
                         &socket_path));

    session.start_event_loop();

    let mut nvim = Neovim::new(session);

    let servername = nvim.get_vvar("servername")
        .expect("Error retrieving servername from neovim over unix socket");

    // let's make sure the servername string and socket path string both match.
    match servername.as_str() {
        Some(ref name) => {
            if Path::new(name) != socket_path {
                panic!(format!("Server name does not match socket path! {} != {}",
                               name,
                               socket_path.to_str().unwrap()));
            }
        }
        None => {
            panic!(format!("Server name does not match socket path! {:?} != {}",
                           servername,
                           socket_path.to_str().unwrap()))
        }
    }
}
