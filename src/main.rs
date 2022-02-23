#![warn(warnings)]

use clap::Parser;

#[derive(Parser)]
struct Opt {
    input: String,
}

fn main() {
    pretty_env_logger::init();

    let opt = Opt::parse();
    let file = std::fs::File::open(opt.input).unwrap();

    let d = evdev_rs::Device::new_from_file(file).unwrap();

    let mut last_event_time = evdev_rs::TimeVal::new(0, 0);

    loop {
        let event = match d.next_event(evdev_rs::ReadFlag::NORMAL | evdev_rs::ReadFlag::BLOCKING) {
            Ok((_, event)) => event,
            Err(_) => continue,
        };

        if let evdev_rs::enums::EventCode::EV_KEY(key) =  event.event_code {
            use evdev_rs::enums::EV_KEY::*;

            if event.time.tv_sec == last_event_time.tv_sec {
                continue;
            }

            match key {
                KEY_PLAYPAUSE => exec("mpc toggle"),
                KEY_NEXTSONG => exec("mpc next"),
                KEY_VOLUMEDOWN => exec("mpc volume -5"),
                KEY_VOLUMEUP => exec("mpc volume +5"),
                _ => continue,
            }

            last_event_time = event.time;
        }
    }
}

fn exec(command: &str) {
    let mut args = command.split(' ').collect::<Vec<_>>();
    let program = args.remove(0);

    log::info!("{} {:?}", program, args);
    match std::process::Command::new(program)
        .args(args)
        .spawn() {
        Ok(_) => (),
        Err(err) => log::error!("{}", err),
    }
}
