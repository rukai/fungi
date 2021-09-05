use rayon::prelude::*;
use byteorder::{BigEndian, ReadBytesExt};

pub enum FileType {
    Dso(DsoFile),
    Obj(ObjFile),
    Ar(ArFile),
    ThinAr(ThinArFile),
    Text(TextFile),
    LLVMBitcode(LLVMBitcodeFile),
}

const ELF_TYPE_NONE: u16 = 0;
const ELF_TYPE_REL: u16 = 1;
const ELF_TYPE_EXEC: u16 = 2;
const ELF_TYPE_DYN: u16 = 3;

pub fn load_files(files: &[String]) -> Vec<FileType> {
    files.par_iter().map(|name| {
        let contents = std::fs::read(name).unwrap();
        if contents.len() >= 20 && &contents[..4] == "\x7FELF".as_bytes() {
            match (&contents[16..18]).read_u16::<BigEndian>().unwrap() {
                ELF_TYPE_NONE => unimplemented!("none elf type"),
                ELF_TYPE_REL => FileType::Obj(ObjFile {
                    contents
                }),
                ELF_TYPE_EXEC => unimplemented!("exec elf type"),
                ELF_TYPE_DYN => FileType::Dso(DsoFile {
                    contents
                }),
                _ => unimplemented!("Unknown ELF type")
            }
        }
        else if contents.len() >= 8 && &contents[..8] == "!<arch>\n".as_bytes() {
            FileType::Ar(ArFile {
                dsos: vec!(),
                objs: vec!(),
            })
        }
        else if contents.len() >= 8 && &contents[..8] == "!<thin>\n".as_bytes() {
            FileType::ThinAr(ThinArFile {
                dsos: vec!(),
                objs: vec!(),
            })
        }
        else if contents.len() >= 4 && (&contents[..8] == [0xDE, 0xC0, 0x17, 0x0B] || &contents[..8] == &[0x42, 0x43, 0xC0, 0xDE]) {
            FileType::LLVMBitcode(LLVMBitcodeFile {
                contents
            })
        }
        else if contents[..4].iter().all(|x| x.is_ascii() && !x.is_ascii_control()) {
            FileType::Text(TextFile {
                contents
            })
        }
        else {
            unimplemented!("Unknown file type");
        }
    }).collect()
}

#[derive(Debug)]
pub struct ObjFile {
    contents: Vec<u8>
}

#[derive(Debug)]
pub struct DsoFile {
    contents: Vec<u8>
}

pub struct ArFile {
    pub objs: Vec<ObjFile>,
    pub dsos: Vec<DsoFile>,
}

pub struct ThinArFile {
    pub objs: Vec<ObjFile>,
    pub dsos: Vec<DsoFile>,
}

#[derive(Debug)]
pub struct TextFile {
    contents: Vec<u8>
}

#[derive(Debug)]
pub struct LLVMBitcodeFile {
    contents: Vec<u8>
}
