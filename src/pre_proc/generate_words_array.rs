use std::fs;

pub fn generate() {
    let file_path = "english.txt";
    let contents = fs::read_to_string(file_path).expect("Should've openned the file.");

    let mut file_content = "pub static WORDS: [&'static str; 2048] = [".to_owned();
    for word in contents.split_whitespace() {
        file_content += "\"";
        file_content += word;
        file_content += "\",";
    }
    file_content.pop();
    file_content += "];";
    fs::write("src/words.rs", file_content).unwrap();
    // println!("Text: \n{contents}");
}
