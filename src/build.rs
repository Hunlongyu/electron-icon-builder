#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
  let mut res = winres::WindowsResource::new();
  res.set_icon("src/ui/logo.ico");
  res.compile().unwrap();
  slint_build::compile("src/ui/main.slint").unwrap();
}

#[cfg(unix)]
fn main() {
  slint_build::compile("ui/main.slint").unwrap();
}
