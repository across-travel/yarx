use std::{
    env, process,
    sync::mpsc::channel,
    thread,
};

fn main() {
    let mut args = env::args();

    if args.len() < 2 {
        let args = args.collect::<Vec<String>>();
        println!("watch: version: {}", env!("CARGO_PKG_VERSION"));
        println!(
            "watch: usage: {} <cmd> [<arg>]...",
            args.get(0).map(|s| s.as_str()).unwrap_or("watch")
        );
        process::exit(1);
    }

    let cmd = args.nth(1).unwrap();
    let args = args.collect::<Vec<String>>();

    let (sender, receiver) = channel();

    {
        let sender = sender.clone();

        thread::spawn(move || yarx::handle(sender, receiver, cmd, args));
    }

    if let Err(_e) = yarx::watch(sender) {
        process::exit(1);
    }

}