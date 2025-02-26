use std::process::Command;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    let path = env::current_dir();

    println!("{0}",path.display());

    println!("{0}", args[1]);
    
    /*let output = Command::new("/bin/cat")
        .arg("file.txt")
        .output()
        .expect("failed to execute process");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success());*/
}
