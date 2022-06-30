
pub mod utils {
  use std::env;

  pub fn get_exe_path () -> String {
    let current_path = match env::current_exe() {
      Ok(exe_path) => {
        exe_path.display().to_string()
      }
      Err(_) => panic!("Path does note exists"),
    };
    current_path
  }
}
