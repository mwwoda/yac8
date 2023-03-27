use std::env;

pub fn load_from_cli() -> Vec<u8> {
    load_rom_from_path(parse_file_path().as_str())
}

pub fn parse_file_path() -> String {
    let mut args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Please provide valid filepath.")
    }
    args.remove(1)
}

pub fn load_rom_from_path(path: &str) -> Vec<u8> {
    std::fs::read(path).unwrap_or_else(|err| panic!("Error encountered while loading file from path {} : {}", path, err))
}