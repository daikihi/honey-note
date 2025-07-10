use std::fs;
use std::path::Path;

fn main() {
    let js_dir = Path::new("../server/src/assets/javascript");
    fs::create_dir_all(js_dir).unwrap();
    let js_path = js_dir.join("honey-note.js");
    fs::write(js_path, "// generated JS code\nconsole.log('Hello!');").unwrap();
}