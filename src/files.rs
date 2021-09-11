use rayon::prelude::*;
use byteorder::{LittleEndian, ReadBytesExt};
use anyhow::anyhow;

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
        let data = std::fs::read(name).map_err(|e| anyhow!("failed to read {}: {}", name, e)).unwrap();
        if data.len() >= 20 && data.starts_with(b"\x7FELF") {
            match (&data[16..18]).read_u16::<LittleEndian>().unwrap() {
                ELF_TYPE_NONE => unimplemented!("none elf type"),
                ELF_TYPE_REL => FileType::Obj(ObjFile {
                    data
                }),
                ELF_TYPE_EXEC => unimplemented!("exec elf type"),
                ELF_TYPE_DYN => FileType::Dso(DsoFile {
                    data
                }),
                elf_type => unimplemented!("Unknown ELF type, {}", elf_type)
            }
        }
        else if data.len() >= 8 && data.starts_with("!<arch>\n".as_bytes()) {
            parse_fat_ar(&data)
        }
        else if data.len() >= 8 && data.starts_with("!<thin>\n".as_bytes()) {
            parse_thin_ar(&data, name)
        }
        else if data.len() >= 4 && (data.starts_with(&[0xDE, 0xC0, 0x17, 0x0B]) || data.starts_with(&[0x42, 0x43, 0xC0, 0xDE])) {
            FileType::LLVMBitcode(LLVMBitcodeFile {
                data
            })
        }
        else if data[..4].iter().all(|x| x.is_ascii() && !x.is_ascii_control()) {
            FileType::Text(TextFile {
                data
            })
        }
        else {
            unimplemented!("Unknown file type");
        }
    }).collect()
}

fn parse_thin_ar(data: &[u8], file_name: &str) -> FileType {
    let mut paths = vec!();

    const AR_HEADER_LENGTH: usize = 60;
    let mut header_index = 8;
    let mut string_table_index = usize::MAX;
    while header_index + 2 < data.len() {
        // Each header is aligned to a 2 byte boundary.
        if header_index % 2 == 1 {
            header_index += 1;
        }

        let header = &data[header_index..AR_HEADER_LENGTH];
        let header_name = &header[..16];
        let header_size = &header[48..58];

        let body_index = header_index + AR_HEADER_LENGTH;
        // TODO: Surely I can skip the utf8 check here?
        // TODO: atol actually ignores any non numeric characters, is that important here?
        let body_size: usize = std::str::from_utf8(header_size).unwrap().parse().unwrap();

        // Read a string table.
        if header_name.starts_with("// ".as_bytes()) {
            string_table_index = body_index;
            header_index = body_index + body_size;
            continue;
        }

        // Skip a symbol table.
        if header_name.starts_with("/ ".as_bytes()) {
            header_index = body_index + body_size;
            continue;
        }

        if header_name[0] != '/' as u8 {
            panic!("Filename is not stored as a long filename: {:?}", header_name)
        }

        // TODO: Surely I can skip the utf8 check here?
        let string_table_offset: usize = std::str::from_utf8(&header_name[1..]).unwrap().parse().unwrap();

        // TODO: Surely I can skip the utf8 check here?
        let name_start = string_table_index + string_table_offset;
        let name_end: usize = data[name_start..].windows(2).position(|bytes2| bytes2 == b"/\n").unwrap(); // TODO: apparently there are faster ways to do this.
        let name = std::str::from_utf8(&data[name_start..name_end]).unwrap();
        let path = if data[name_start] == b'/' {
            name.to_string()
        } else {
            format!("{}/{}", file_name, name)
        };
        paths.push(path);
    }
    println!("{:?}", paths);

    FileType::ThinAr(ThinArFile {
        dsos: vec!(),
        objs: vec!(),
        paths,
    })
}

fn parse_fat_ar(_data: &[u8]) -> FileType {
    FileType::Ar(ArFile {
        dsos: vec!(),
        objs: vec!(),
    })
}

#[derive(Debug)]
pub struct ObjFile {
    data: Vec<u8>
}

#[derive(Debug)]
pub struct DsoFile {
    data: Vec<u8>
}

pub struct ArFile {
    pub objs: Vec<ObjFile>,
    pub dsos: Vec<DsoFile>,
}

pub struct ThinArFile {
    pub objs: Vec<ObjFile>,
    pub dsos: Vec<DsoFile>,
    pub paths: Vec<String>,
}

#[derive(Debug)]
pub struct TextFile {
    data: Vec<u8>
}

#[derive(Debug)]
pub struct LLVMBitcodeFile {
    data: Vec<u8>
}
