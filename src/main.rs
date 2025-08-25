use eframe::egui;
     use printer_kinematics::{create_kinematics, Config, Kinematics, Scene};
     use std::fs;

     fn main() -> eframe::Result<()> {
         let config_str = fs::read_to_string("printer.toml").expect("Failed to read printer.toml");
         let config: Config = toml::from_str(&config_str).expect("Failed to parse printer.toml");
         let kinematics = create_kinematics(&config);

         let native_options = eframe::NativeOptions {
             viewport: egui::ViewportBuilder::default().with_inner_size(egui::vec2(800.0, 600.0)),
             ..Default::default()
         };

         eframe::run_native(
             "Printer Host",
             native_options,
             Box::new(|_cc| Ok(Box::new(MyApp::new(kinematics)))),
         )
     }

     struct MyApp {
         kinematics: Box<dyn Kinematics>,
         scene: Scene,
         x: f32,
         y: f32,
         z: f32,
         a: f32,
         b: f32,
     }

     impl MyApp {
         fn new(kinematics: Box<dyn Kinematics>) -> Self {
             Self {
                 kinematics,
                 scene: Scene::new(),
                 x: 0.0,
                 y: 0.0,
                 z: 0.0,
                 a: 0.0,
                 b: 0.0,
             }
         }
     }

     impl eframe::App for MyApp {
         fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
             egui::CentralPanel::default().show(ctx, |ui| {
                 ui.heading("Printer Kinematics Control");
                 ui.add(egui::Slider::new(&mut self.x, -100.0..=100.0).text("X"));
                 ui.add(egui::Slider::new(&mut self.y, -100.0..=100.0).text("Y"));
                 ui.add(egui::Slider::new(&mut self.z, -100.0..=100.0).text("Z"));
                 ui.add(egui::Slider::new(&mut self.a, -180.0..=180.0).text("A (degrees)"));
                 ui.add(egui::Slider::new(&mut self.b, -180.0..=180.0).text("B (degrees)"));

                 let positions = self.kinematics.to_motor_positions(self.x, self.y, self.z, self.a, self.b);
                 ui.label(format!("Motor Positions: {:?}", positions));

                 self.kinematics.visualize(&mut self.scene);
                 ui.label(format!("Scene Entities: {}", self.scene.entities.len()));
             });
         }
     }