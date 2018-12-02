extern crate webpacker;

fn main() {
    println!("Validating dependencies…");
    assert!(webpacker::valid_project_dir());
    
    println!("Compiling assets…");
    let _ = webpacker::compile();
}
