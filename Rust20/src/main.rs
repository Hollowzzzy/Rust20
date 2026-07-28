use eframe::egui::{self, ColorImage, TextureHandle};
use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};
use std::io::Cursor;

const DICE_IMAGE: &[u8] = include_bytes!("../assets/dice_image/Dice.png");

const ROLL_SOUND: &[u8] = include_bytes!("../assets/sounds/roll.wav");
const NAT1_SOUND: &[u8] = include_bytes!("../assets/sounds/nat1.wav");
const NAT20_SOUND: &[u8] = include_bytes!("../assets/sounds/nat20.wav");

fn main() -> Result<(), eframe::Error> {
    let icon = load_icon();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Rust20")
            .with_icon(icon),
        ..Default::default()
    };

    eframe::run_native("Rust20", options, Box::new(|cc| Ok(Box::new(App::new(cc)))))
}

fn load_icon() -> egui::IconData {
    let image = image::load_from_memory(include_bytes!("../assets/dice_image/Dice.png"))
        .expect("Failed to load icon")
        .into_rgba8();

    let (width, height) = image.dimensions();

    egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    }
}

struct App {
    roll_number: usize,
    dice_image: TextureHandle,
    show_ui: bool,
    play_sound: bool,
    stream: Option<OutputStream>,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let dice_image =
            cc.egui_ctx
                .load_texture("dice", load_image(DICE_IMAGE), Default::default());

        let stream = match OutputStreamBuilder::open_default_stream() {
            Ok(stream) => Some(stream),
            Err(err) => {
                eprintln!("Failed to initialize audio: {err}");
                None
            }
        };

        Self {
            roll_number: 1,
            dice_image,
            show_ui: true,
            play_sound: true,
            stream,
        }
    }

    fn play_sound(&self) {
        if !self.play_sound {
            return;
        }

        let Some(stream) = &self.stream else {
            return;
        };

        let bytes = match self.roll_number {
            1 => NAT1_SOUND,
            20 => NAT20_SOUND,
            _ => ROLL_SOUND,
        };

        let decoder = match Decoder::try_from(Cursor::new(bytes)) {
            Ok(decoder) => decoder,
            Err(err) => {
                eprintln!("Failed to decode sound: {err}");
                return;
            }
        };

        let sink = Sink::connect_new(stream.mixer());

        sink.append(decoder);
        sink.detach();
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.show_ui = !self.show_ui;
        }
        if ctx.input(|i| i.key_pressed(egui::Key::E)) {
            self.roll_number = rand::random_range(1..=20);
            self.play_sound();
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(egui::Color32::from_rgb(67, 67, 67)))
            .show(ctx, |ui| {
                ui.with_layout(
                    egui::Layout::top_down(egui::Align::Center)
                        .with_cross_align(egui::Align::Center),
                    |ui| {
                        let content_height = if self.show_ui { 390.0 } else { 256.0 };

                        ui.add_space((ui.available_height() - content_height).max(0.0) / 2.0);

                        let size = egui::vec2(256.0, 256.0);
                        let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());

                        ui.painter().image(
                            self.dice_image.id(),
                            rect,
                            egui::Rect::from_min_max(
                                egui::Pos2::new(0.0, 0.0),
                                egui::Pos2::new(1.0, 1.0),
                            ),
                            egui::Color32::WHITE,
                        );

                        let painter = ui.painter();
                        let center = rect.center();

                        let font_size = 30.0;
                        let font = egui::FontId::proportional(font_size);
                        let text = self.roll_number.to_string();

                        let text_color = match self.roll_number {
                            1 => egui::Color32::RED,
                            20 => egui::Color32::GREEN,
                            _ => egui::Color32::WHITE,
                        };

                        let outline = (font_size / 64.0).max(1.0);

                        for x in -1..=1 {
                            for y in -1..=1 {
                                if x == 0 && y == 0 {
                                    continue;
                                }

                                painter.text(
                                    center + egui::vec2(x as f32 * outline, y as f32 * outline),
                                    egui::Align2::CENTER_CENTER,
                                    &text,
                                    font.clone(),
                                    egui::Color32::BLACK,
                                );
                            }
                        }

                        painter.text(center, egui::Align2::CENTER_CENTER, &text, font, text_color);

                        if self.show_ui {
                            ui.add_space(10.0);

                            ui.label(format!("You rolled a {}", self.roll_number));

                            ui.add_space(10.0);

                            if ui.button("Roll Dice").clicked() {
                                self.roll_number = rand::random_range(1..=20);
                                self.play_sound();
                            }

                            ui.add_space(10.0);

                            ui.checkbox(&mut self.play_sound, "Enable Sound");

                            if self.stream.is_none() {
                                ui.colored_label(egui::Color32::YELLOW, "Audio unavailable");
                            }
                        }
                    },
                );
            });
    }
}

fn load_image(bytes: &[u8]) -> ColorImage {
    let image = image::load_from_memory(bytes)
        .expect("Failed to decode image")
        .to_rgba8();

    let size = [image.width() as usize, image.height() as usize];

    ColorImage::from_rgba_unmultiplied(size, image.as_raw())
}
