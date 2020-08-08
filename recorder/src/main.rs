extern crate config;
extern crate log;
extern crate simplelog;

extern crate daemonize;

mod settings;
// use settings::{Source};

use std::fs::File;
// use std::collections::HashMap;
use std::{thread, time};

use config::Config;

use log::{info/*, trace*/, warn};
use simplelog::*;

use daemonize::Daemonize;


fn main() {
    let mut settings = Config::default();
    settings.merge(config::File::with_name("conf/sources.json")).unwrap();

    // let sources = settings.get::<Vec<Source>>("cameras").unwrap();
    // println!("\n{:?}\n", sources[0].name);

    CombinedLogger::init(
        vec![
            TermLogger::new(
                LevelFilter::Warn, 
                simplelog::Config::default(), 
                TerminalMode::Mixed
            ),
            WriteLogger::new(
                LevelFilter::Info, 
                simplelog::Config::default(), 
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
