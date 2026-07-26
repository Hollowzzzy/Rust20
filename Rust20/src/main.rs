use eframe::egui::{self, ColorImage, TextureHandle};
use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};
use std::io::Cursor;

const DICE_IMAGES: [&[u8]; 20] = [
    include_bytes!("../assets/dice_images/Dice_1.png"),
    include_bytes!("../assets/dice_images/Dice_2.png"),
    include_bytes!("../assets/dice_images/Dice_3.png"),
    include_bytes!("../assets/dice_images/Dice_4.png"),
    include_bytes!("../assets/dice_images/Dice_5.png"),
    include_bytes!("../assets/dice_images/Dice_6.png"),
    include_bytes!("../assets/dice_images/Dice_7.png"),
    include_bytes!("../assets/dice_images/Dice_8.png"),
    include_bytes!("../assets/dice_images/Dice_9.png"),
    include_bytes!("../assets/dice_images/Dice_10.png"),
    include_bytes!("../assets/dice_images/Dice_11.png"),
    include_bytes!("../assets/dice_images/Dice_12.png"),
    include_bytes!("../assets/dice_images/Dice_13.png"),
    include_bytes!("../assets/dice_images/Dice_14.png"),
    include_bytes!("../assets/dice_images/Dice_15.png"),
    include_bytes!("../assets/dice_images/Dice_16.png"),
    include_bytes!("../assets/dice_images/Dice_17.png"),
    include_bytes!("../assets/dice_images/Dice_18.png"),
    include_bytes!("../assets/dice_images/Dice_19.png"),
    include_bytes!("../assets/dice_images/Dice_20.png"),
];

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
    let image = image::load_from_memory(include_bytes!("../assets/dice_images/R20.png"))
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
    dice_images: Vec<TextureHandle>,
    show_ui: bool,
    play_sound: bool,
    stream: OutputStream,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let dice_images = DICE_IMAGES
            .iter()
            .enumerate()
            .map(|(i, bytes)| {
                cc.egui_ctx.load_texture(
                    format!("dice_{}", i + 1),
                    load_image(bytes),
                    Default::default(),
                )
            })
            .collect();

        let stream =
            OutputStreamBuilder::open_default_stream().expect("Failed to open audio device");

        Self {
            roll_number: 1,
            dice_images,
            show_ui: true,
            play_sound: true,
            stream,
        }
    }

    fn play_sound(&self) {
        if self.play_sound {
            let bytes = match self.roll_number {
                1 => NAT1_SOUND,
                20 => NAT20_SOUND,
                _ => ROLL_SOUND,
            };

            let cursor = Cursor::new(bytes);

            let decoder = Decoder::try_from(cursor).expect("Failed to decode sound");

            let sink = Sink::connect_new(self.stream.mixer());

            sink.append(decoder);
            sink.detach();
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.show_ui = !self.show_ui;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::top_down(egui::Align::Center).with_cross_align(egui::Align::Center),
                |ui| {
                    let content_height = if self.show_ui { 390.0 } else { 256.0 };

                    ui.add_space((ui.available_height() - content_height).max(0.0) / 2.0);

                    ui.add(
                        egui::Image::new(&self.dice_images[self.roll_number - 1])
                            .fit_to_exact_size(egui::vec2(256.0, 256.0)),
                    );

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
