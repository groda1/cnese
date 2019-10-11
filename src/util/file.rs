
pub fn derp(path: &String) -> Vec<u8> {
    std::fs::read(path).expect("Unable to open file!")
}