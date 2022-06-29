#![windows_subsystem = "windows"]

use slint::SharedString;
use native_dialog::FileDialog;

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
        main.on_handleConfirm(move |input: SharedString, output: SharedString, flag: bool| {
            println!("handle confirm{} {} {}", input, output, flag);
        });
    }

    main.run();
}
