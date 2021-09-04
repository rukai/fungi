#[derive(Debug)]
pub struct Args {
    pub pie: bool,
    pub statik: bool,
    pub as_needed: bool,
    pub emulation: Emulation,
    pub files: Vec<String>,
}

impl Args {
    pub fn parse() -> Args {
        let mut args = std::env::args();
        args.next();

        let mut result = Args {
            pie: false,
            statik: false,
            as_needed: false,
            emulation: Emulation::X86_64,
            files: vec!(),
        };

        while let Some(arg) = args.next() {
            if arg == "--help" || arg == "-help" {
                println!("Halp!")
            }
            else if arg == "--build-id" {
                // TODO
            }
            else if arg == "--eh-frame-hdr" {
                // TODO
            }
            else if arg == "--hash-style=gnu" {
                // TODO
            }
            else if arg == "-m" {
                match args.next().unwrap().as_str() {
                    "elf_x86_64" => result.emulation = Emulation::X86_64,
                    "elf_i386" => result.emulation = Emulation::I386,
                    "aarch64linux" => result.emulation = Emulation::AARCH64,
                  _ => unimplemented!(),
                }
            }
            else if arg == "-pie" {
                result.pie  = true;
            }
            else if arg == "-BStatic" {
                result.statik = true;
            }
            else if arg == "-Bdynamic" {
                result.statik = false;
            }
            else if arg == "--as-needed" {
                result.as_needed = true;
            }
            else if arg == "--no-as-needed" {
                result.as_needed = false;
            }
            else if arg == "-plugin" || arg.starts_with("-plugin-opt") {
                // ignored
                args.next();
            }
            else {
                result.files.push(arg)
            }
        }

        result
    }
}

#[derive(Debug)]
pub enum Emulation {
    X86_64,
    I386,
    AARCH64,
}
