fn main() {
    let bytes = std::fs::read("./europa-fakten-fischerei.pdf").unwrap();
    let out = pdf_extract::extract_text_from_mem(&bytes).unwrap();
    println!("{}", out);
}
