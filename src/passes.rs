use crate::cli::Args;
use crate::files::{FileType, ObjFile, DsoFile, load_files};

pub struct Passes {
    args: Args,
    objs: Vec<ObjFile>,
    dsos: Vec<DsoFile>,
}

impl Passes {
    pub fn new(args: Args) -> Passes {
        let mut objs = vec!();
        let mut dsos = vec!();
        for file in load_files(&args.files) {
            match file {
                FileType::Obj(obj) => objs.push(obj),
                FileType::Dso(dso) => dsos.push(dso),
                FileType::Ar(ar) => {
                    dsos.extend(ar.dsos);
                    objs.extend(ar.objs);
                }
                FileType::ThinAr(ar) => {
                    dsos.extend(ar.dsos);
                    objs.extend(ar.objs);
                }
                FileType::Text(text) => todo!("Linker script unimplemented: {:?}", text),
                FileType::LLVMBitcode(bitcode) => unimplemented!("LLVM bitcode is not supported: {:?}", bitcode),
            }
        }

        Passes {
            args,
            objs,
            dsos,
        }
    }

    pub fn blah(&mut self) {

    }

    pub fn blah_blah(&mut self) {
        panic!("{:?} {} {}", self.args, self.objs.len(), self.dsos.len());
    }
}
