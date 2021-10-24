use std::fs;
use std::path::Path;

fn copy_res_dir(target_path: String) {
    let res_path = Path::new("resources");
    let dir = fs::read_dir(res_path).unwrap();
    for file in dir {
        if let Ok(entry) = file {
            let path = format!("./{}/resources", target_path);
            fs::create_dir_all(&path).unwrap();
            fs::copy(
                entry.path(),
                format!(
                    "./{}/resources/{}",
                    target_path,
                    entry.file_name().to_str().unwrap()
                ),
            )
            .unwrap();
        }
    }
}

fn main() {
    let profile = std::env::var("PROFILE").unwrap();
    copy_res_dir(format!("./target/{}", profile));
}
