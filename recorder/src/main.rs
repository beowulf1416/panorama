extern crate log;
extern crate simplelog;

extern crate daemonize;

use log::{info/*, trace*/, warn};
use simplelog::*;
use std::fs::File;

use daemonize::Daemonize;

use std::{thread, time};


fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(
                LevelFilter::Warn, 
                Config::default(), 
                TerminalMode::Mixed
            ),
            WriteLogger::new(
                LevelFilter::Info, 
                Config::default(), 
                File::create("/tmp/panorama.log").unwrap()
            ),
        ]
    ).unwrap();

    info!("starting up ...");

    // let stdout = File::open("/tmp/panorama.log").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("/tmp/panorama-recorder.pid")
        // .chown_pid_file(true)      // is optional, see `Daemonize` documentation
        .working_directory("/tmp") // for default behaviour.
        // .user("nobody")
        // .group("panorama")
        // .group(2)
        .umask(0o027)
        // .stdout(stdout)  // Redirect stdout to `/tmp/daemon.out`.
        // .stderr(stderr)  // Redirect stderr to `/tmp/daemon.err`.
        .exit_action(|| info!("exiting"))
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => info!("successfully daemonized"),
        Err(e) => warn!("unable to daemonize: {}", e),
    }

    let delay = time::Duration::from_millis(3000);
    // let now = time::Instant::now();

    thread::sleep(delay);
    info!("continuing");
}
