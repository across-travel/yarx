use std::{env, io, path, process};

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();

    if args.len() >= 3 {
        let root = path::Path::new(&args[1]);
        let src = path::Path::new(&args[1]);
        let dest = path::Path::new(&args[2]);

        let mut extensions = Vec::new();
        let mut args_iter = args[3..].iter();

        while let Some(a) = args_iter.next() {
            if a == "--ext" {
                match args_iter.next() {
                    Some(e) => {
                        extensions.push(e.as_str());
                    }

                    None => {
                        println!("--ext must be followed by an extension");
                        process::exit(1);
                    }
                }
            } else {
                println!("unknown argument: {}", a);
                process::exit(1);
            }
        }

        yarx::run(&root, &src, &dest, &extensions)?;

        process::exit(0)
    } else {
        println!("build version: {}", env!("CARGO_PKG_VERSION"));
        println!(
            "usage: {} <src-dir> <dest-dir> <name-pattern>",
            args.get(0).map(|s| s.as_str()).unwrap_or("build")
        );
        process::exit(1);
    }
}