pub fn read_file(path: &String) -> Vec<u8> {
    std::fs::read(path).expect(format!("Unable to open file: {}", path).as_str())
}

