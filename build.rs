use std::process::Command;
use std::path::Path;

fn main() {
    // only run if elm source changes
    println!("cargo:rerun-if-changed=src-elm/");

    let input_path = Path::new("main.elm");
    //let out_dir = env::var("OUT_DIR").unwrap();
    //let dest_path = Path::new(&out_dir).join("main.html");
    let dest_path = Path::new("../assets/index.html");
    let out = Command::new("elm-make")
        .current_dir(Path::new("src-elm"))
        .arg(input_path)
        .arg("--output")
        .arg(dest_path)
        .arg("--yes")
        .output().unwrap();
    if !out.status.success() {
        println!("{}", String::from_utf8_lossy(&out.stderr));
        std::process::exit(1);
    }
}
