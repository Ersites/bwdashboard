use bwdashboard::app::MyApp;

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "bwdashboard",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc.egui_ctx.clone())))),
    )
}
