use eframe::egui::{self, widgets};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };
    eframe::run_native("dlib-loader", options, Box::new(|_cc| Box::<MyApp>::default()))
}

struct MyApp {
    lib_path: String,
    func_name: String,
    output: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            lib_path: "target/release/lib_example.dll".to_string(),
            func_name: "add".to_string(),
            output: "".to_string(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::Label::new("Dynamic Library Path"));
            ui.text_edit_singleline(&mut self.lib_path);
            ui.add(egui::Label::new("Call Func"));
            ui.text_edit_singleline(&mut self.func_name);
            if ui.button("Call dynamic function").clicked() {
                match call_dynamic(self.lib_path.clone(), self.func_name.clone()) {
                    Ok(result) => {
                        self.output = format!("Result: {}", result);
                    }
                    Err(err) => {
                        self.output = format!("Error: {}", err);
                    }
                }
            }
            ui.add(widgets::Separator::default());
            ui.add(egui::Label::new("Output"));
            ui.text_edit_multiline(&mut self.output);
        });

        std::thread::sleep(std::time::Duration::from_millis(10));
        ctx.request_repaint()
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        println!("Goodbye!");
    }
}


fn call_dynamic(file_path: String, func_name: String) -> Result<u32, Box<dyn std::error::Error>> {
    unsafe {
        let lib = libloading::Library::new(file_path)?;
        let func: libloading::Symbol<unsafe extern fn(usize, usize) -> usize> = lib.get(func_name.as_bytes())?;
        Ok(func(2, 2) as u32)
    }
}