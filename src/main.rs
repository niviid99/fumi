mod fmgr;
mod ui;

fn main() {
    let natopts = eframe::NativeOptions::default();
    let err = eframe::run_native(
        "fumi",
        natopts,
        Box::new(|_cc| Ok(Box::new(ui::Main::default()))),
    );

    if err.is_err() {
        println!("eframe::run_native crushed with errors");
    }
}
