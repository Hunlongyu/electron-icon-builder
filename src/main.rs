#![windows_subsystem = "windows"]
use slint::SharedString;
use native_dialog::FileDialog;
use std::time::Instant;

mod utils;
use crate::utils::utils::{ check_input, check_output, winico, linuxpng, macicns };

slint::include_modules!();

fn main() {
    let main = Main::new();

    {
        let main_weak = main.as_weak();
        main.on_handleInput(move || {
            let dialog = FileDialog::new().add_filter("png image", &["png"]).show_open_single_file().unwrap();
            let path = match dialog {
                Some(path) => path,
                None => return,
            };
            let input_path = SharedString::from(path.as_path().display().to_string());
            let mainw = main_weak.unwrap();
            mainw.set_inputPath(input_path);
        });
    }

    {
        let main_weak = main.as_weak();
        main.on_handleOutput(move || {
            let dialog = FileDialog::new().show_open_single_dir().unwrap();
            let path = match dialog {
                Some(path) => path,
                None => return,
            };
            let output_path = SharedString::from(path.as_path().display().to_string());
            let mainw = main_weak.unwrap();
            mainw.set_outputPath(output_path);
        });
    }

    {
        let main_weak = main.as_weak();
        main.on_handleReset(move || {
            let mainw = main_weak.unwrap();
            mainw.set_flatten(false);
            let ip = SharedString::from("");
            let op = SharedString::from("");
            mainw.set_inputPath(ip);
            mainw.set_outputPath(op);
        });
    }


    {
        let main_weak = main.as_weak();
        main.on_handleConfirm(move |input: SharedString, output: SharedString, flag: bool| {
            let mainw = main_weak.unwrap();
            let source = match check_input(input) {
                Ok(path) => path,
                Err(_) => {
                    let str = SharedString::from("PNG file not found or Square image less than 512 in width and height, please check and try again");
                    mainw.set_popupText(str);
                    mainw.invoke_show_popup();
                    return;
                },
            };
            let target = check_output(output);
            let now = Instant::now();
            winico(&source, &target, flag);
            macicns(&source, &target, flag);
            linuxpng(&source, &target, flag);
            let end = Instant::now();
            let time = format!("Done! It takes {:?}", end - now);
            let success = SharedString::from(time);
            mainw.set_popupText(success);
            mainw.invoke_show_popup();
        });
    }

    main.run();
}
