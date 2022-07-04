
pub mod utils {
  use std::{env, path::{PathBuf}, fs::{create_dir, remove_dir_all, File}};
  use slint::SharedString;
  use icns::{Image, IconFamily};
  use image::io::Reader as ImageReader;

  pub fn check_input(input: SharedString) -> Result<PathBuf, bool> {
    let current_dir = env::current_dir().unwrap();
    let source_path;
    if input == "" {
      let default_png_path = current_dir.join("icon.png");
      if !default_png_path.exists() {
        return Err(false);
      }
      source_path = default_png_path;
    } else {
      let input_path = PathBuf::from(input.as_str());
      if !input_path.exists() {
        return Err(false);
      }
      source_path = input_path;
    };
    return Ok(source_path);
  }

  pub fn check_output(output: SharedString) -> PathBuf {
    let current_dir = env::current_dir().unwrap();
    let target_path;
    if output == "" {
      if !current_dir.join("build").exists() {
        create_dir(current_dir.join("build")).unwrap();
      } else {
        remove_dir_all(current_dir.join("build")).unwrap();
        create_dir(current_dir.join("build")).unwrap();
      };
      target_path = current_dir.join("build");
    } else {
      let output_path = PathBuf::from(output.as_str());
      if !output_path.join("build").exists() {
        create_dir(output_path.join("build")).unwrap();
      } else {
        remove_dir_all(output_path.join("build")).unwrap();
        create_dir(output_path.join("build")).unwrap();
      };
      target_path = output_path.join("build");
    };
    return target_path;
  }

  pub async fn winico(input: &PathBuf, output: &PathBuf, is_flatten: bool) {
    let target_path;
    if is_flatten {
      target_path = output.join("icon.ico");
    } else {
      create_dir(output.join("win")).unwrap();
      target_path = output.join("win").join("icon.ico")
    };
    let png_file = image::open(input).unwrap().to_rgba8();
    let resized = image::imageops::resize(&png_file, 256, 256, image::imageops::FilterType::Triangle);
    resized.save(target_path).unwrap();
  }

  pub async fn linuxpng(input: &PathBuf, output: &PathBuf, is_flatten: bool) {
    let sizes: Vec<u32> = vec![16, 32, 48, 64, 128, 512];
    let png_file = image::open(input).unwrap().to_rgba8();
    let target_path;
    if is_flatten {
      target_path = PathBuf::from(output);
    } else {
      create_dir(output.join("png")).unwrap();
      target_path = output.join("png");
    };
    for n in sizes {
      let name = format!("{}x{}.png",  n, n);
      let path = target_path.join(name);
      let resized = image::imageops::resize(&png_file, n, n, image::imageops::FilterType::Triangle);
      resized.save(path).unwrap();
    };
  }

  pub async fn macicns(input: &PathBuf, output: &PathBuf, is_flatten: bool) {
    let target_path;
    if is_flatten {
      target_path = output.join("icon.icns");
    } else {
      create_dir(output.join("mac")).unwrap();
      target_path = output.join("mac").join("icon.icns")
    };
    let path = File::create(&target_path).unwrap();
    let mut icns = IconFamily::new();
    let bytes = ImageReader::open(input).unwrap().decode().unwrap().resize(512, 512, image::imageops::FilterType::Triangle).to_rgba8().to_vec();
    let icon = Image::from_data(icns::PixelFormat::RGBA, 512, 512, bytes).unwrap();
    icns.add_icon(&icon).unwrap();
    icns.write(path).unwrap();
  }
}
