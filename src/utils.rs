
pub mod utils {
  use std::{env, path::{PathBuf}, fs::{create_dir, create_dir_all}};
  use slint::SharedString;

  pub fn winico(input: SharedString, output: SharedString, is_flatten: bool) {
    let current_dir = env::current_dir().unwrap();
    println!("current dir: {:?}", current_dir);

    let source_path;
    if input == "" {
      let default_png_path = current_dir.join("icon.png");
      source_path = default_png_path;
    } else {
      let input_path = PathBuf::from(input.as_str());
      source_path = input_path;
    };
    println!("source path: {:?}", &source_path);

    let target_path;
    if output == "" {
      if is_flatten {
        if !current_dir.join("build").exists() {
          create_dir(current_dir.join("build")).unwrap();
        };
        target_path = current_dir.join("icon.ico");
      } else {
        if !current_dir.join("build").join("win").exists() {
          create_dir_all(current_dir.join("build").join("win")).unwrap();
        };
        target_path = current_dir.join("build").join("win").join("icon.ico");
      };
    } else {
      let output_path = PathBuf::from(output.as_str());
      if is_flatten {
        if !output_path.join("build").exists() {
          create_dir(output_path.join("build")).unwrap();
        };
        target_path = output_path.join("build").join("icon.ico");
      } else {
        if !output_path.join("build").join("win").exists() {
          create_dir_all(output_path.join("build").join("win")).unwrap();
        };
        target_path = output_path.join("build").join("win").join("icon.ico")
      };
    };
    println!("target path: {:?}", &target_path);

    let png_file = image::open(source_path).unwrap().to_rgba8();
    let resized = image::imageops::resize(&png_file, 256, 256, image::imageops::FilterType::Triangle);
    resized.save(target_path).unwrap();
  }
}
