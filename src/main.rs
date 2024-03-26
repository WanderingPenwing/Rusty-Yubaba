use eframe::egui;
use egui::{Visuals};
use std::path::PathBuf;
use std::env;
use std::thread;
use std::sync::Arc;
use std::time;

mod format;
use format::Format;
mod core;
use core::{FileEntry, ImageSettings};

mod python;
mod totoro;
mod ui;

const MAX_FPS: f32 = 30.0;
const TIME_SCALE : f32 = 0.3;

//🎥 🎧 🎨
fn main() -> Result<(), eframe::Error> {
	if let Ok(test) = python::find_file_name("etc/test/turtle.webp".to_string()) {
		println!("python call successful : etc/test/turtle.webp => {:?}", test);
	}

	for arg in env::args().skip(1) {
		println!("Opening file: {}", arg);
	}

	let icon_data = core::load_icon().unwrap_or_default();

	let options = eframe::NativeOptions {
   		viewport: egui::ViewportBuilder::default()
   			.with_inner_size([1200.0, 800.0])
   			.with_icon(Arc::new(icon_data)),
   		..Default::default()
   	};

   	eframe::run_native(
		"Yubaba",
		options,
		Box::new(move |_cc| Box::from(Yubaba::default())),
	)
}


struct Yubaba {
	input_files: Vec<FileEntry>,
	output_folder: Option<String>,
	file_type: Format,
	selected_extension: String,
	processing_handle: Option<thread::JoinHandle<()>>,
	image_settings: Option<ImageSettings>,
	frame_time: time::Instant,
	current_frame: f32,
}

impl Default for Yubaba {
	fn default() -> Self {
		Self {
			input_files: vec![],
			output_folder: None,
			file_type: Format::None,
			selected_extension: "".to_string(),
			processing_handle: None,
			image_settings: None,
			frame_time: time::Instant::now(),
			current_frame: 0.0,
		}
	}
}

impl eframe::App for Yubaba {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		thread::sleep(time::Duration::from_secs_f32(
			((1.0 / MAX_FPS) - self.frame_time.elapsed().as_secs_f32()).max(0.0),
		));
		self.frame_time = time::Instant::now();
		self.current_frame += TIME_SCALE;
		if self.current_frame > totoro::GIF.len() as f32 {
			self.current_frame = 0.0;
		}
		
		egui_extras::install_image_loaders(ctx);
		ctx.set_visuals(Visuals::dark());
		
		egui::TopBottomPanel::bottom("credits").show(ctx, |ui| {
			ui.label("Yubaba, inspired by “Spirited Away”, is a file conversion tool made by Nicolas, Paul, Thomas, and Eliott during the Cod’Icam Hackathon, March 2024, built with Rust and Python.");	
		});
		
		self.show_file_panel(ctx);

		self.show_config_panel(ctx);
		
		if !self.processing_handle.is_none() {
			egui::Context::request_repaint(ctx);
		}
	}
}



impl Yubaba {
	fn open_files(&mut self, paths: Vec<PathBuf>) {
		for path in paths {
			let path_str = path.to_string_lossy().to_string();
			println!("opened : {}", path.display());
			let mut new_file = FileEntry::default();
			if let Ok(file_data) = python::find_file_name(path_str.clone()) {
				if self.output_folder.is_none() {
					self.select_folder(PathBuf::from(file_data[0].clone()));
				}
				new_file.name = file_data[1].clone();
				new_file.extension = file_data[2].clone();
				if self.file_type == Format::None {
					self.file_type = Format::from_extension(file_data[2].clone());
					println!("new file type : {}", self.file_type.display());
				}
			}
			new_file.path = path_str;
			
			self.input_files.push(new_file);
		}
	}
	
	fn select_folder(&mut self, path: PathBuf) {
		println!("folder : {}", path.display());
		self.output_folder = Some(path.to_string_lossy().to_string());
	}

	fn reset(&mut self) {
		self.file_type = Format::None;
		self.selected_extension = "".to_string();
		self.image_settings = None;
	}
}
