use eframe::egui;
use egui::{Visuals};
use std::path::PathBuf;
use std::env;
use std::thread;
use std::sync::Arc;
use std::collections::HashMap;
use std::io::{Cursor};
use image::codecs::gif::{GifDecoder};
use image::{AnimationDecoder, Frame};

mod format;
use format::Format;

mod python;

mod core;

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

#[derive(Clone)]
struct FileEntry {
	name: String,
	extension: String,
	path: String,
}

impl Default for FileEntry {
	fn default() -> Self {
		Self {
			name: "none".to_string(),
			extension: "".to_string(),
			path: "".to_string(),
		}
	}
}

#[derive(Clone)]
struct ImageSettings {
	compress: bool,
	h_flip: bool,
	greyscale: bool,
	rotate: i16,
	smoothing: bool,
}

impl Default for ImageSettings {
	fn default() -> Self {
		Self {
			compress: false,
			h_flip: false,
			greyscale: false,
			rotate: 0,
			smoothing: false,
		}
	}
}


struct Yubaba {
	input_files: Vec<FileEntry>,
	output_folder: Option<String>,
	file_type: Format,
	selected_extension: String,
	processing_handle: Option<thread::JoinHandle<()>>,
	image_settings: Option<ImageSettings>,
	loading_gif: Vec<Frame>,
}

impl Default for Yubaba {
	fn default() -> Self {
		let gif_data = include_bytes!("../assets/totoro.gif");
        let gif_cursor = Cursor::new(Vec::from(gif_data));

        let decoder = GifDecoder::new(gif_cursor).unwrap();
        let frames = decoder.into_frames();
        let frames = frames.collect_frames().expect("error decoding gif");
		
		Self {
			input_files: vec![],
			output_folder: None,
			file_type: Format::None,
			selected_extension: "".to_string(),
			processing_handle: None,
			image_settings: None,
			loading_gif: frames,
		}
	}
}

impl eframe::App for Yubaba {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		ctx.set_visuals(Visuals::dark());
		
		egui::TopBottomPanel::bottom("credits").show(ctx, |ui| {
			ui.label("Yubaba, inspired by “Spirited Away”, is a file conversion tool made by Nicolas, Paul, Thomas, and Eliott during the Cod’Icam Hackathon, March 2024, built with Rust and Python.");	
		});
		
		egui::SidePanel::left("file_tree_panel").show(ctx, |ui| {
			if ui.add(egui::Button::new(&format!("📁 Open Files {}", self.file_type.display()))).clicked() {
				if let Some(paths) = rfd::FileDialog::new()
					.add_filter(self.file_type.to_filter(), &self.file_type.get_extensions())
					.pick_files()
				{
					self.open_files(paths);
				}
			}

			ui.separator();

			let mut index_to_remove : Option<usize> = None;
			
			for (index, file) in self.input_files.iter().enumerate() {
				ui.horizontal(|ui| {
					if ui.add(egui::Button::new("X")).clicked() {
						index_to_remove = Some(index)
					}
					ui.label(&format!("{}.{}", file.name, file.extension));
				});
			}

			if let Some(index) = index_to_remove {
				self.input_files.remove(index);
				if self.input_files.is_empty() {
					self.reset();
				}
			}
		});

		egui::CentralPanel::default().show(ctx, |ui| {

			if let Some(handle) = &self.processing_handle {
				ui.label("Processing...");
				//ui.add(frame_to_egui_image(&self.loading_gif[0]));
				if handle.is_finished() {
					self.processing_handle = None;
				}
				return
			}
			
			egui::Grid::new("my_grid")
				.num_columns(2)
				.spacing([40.0, 4.0])
				.striped(true)
				.show(ui, |ui| {
					ui.label("📦 Output Folder");

					let button_text = if let Some(folder) = &self.output_folder {
						folder
					} else {
						"No Folder Selected"
					};
					
					if ui.add(egui::Button::new(button_text)).clicked() {
						if let Some(path) = rfd::FileDialog::new().pick_folder()
						{
							self.select_folder(path);
						}
					}
					ui.end_row();
					
					ui.label("📋 Output Format");
					
					egui::ComboBox::from_label("")
						.selected_text(self.selected_extension.clone())
			            .show_ui(ui, |ui| {
			                ui.style_mut().wrap = Some(false);
			                ui.set_min_width(60.0);
			                for extension in self.file_type.get_extensions() {
			                	if ui.add(egui::SelectableLabel::new(self.selected_extension == extension, extension.clone())).clicked() {
			                	    self.selected_extension = extension.to_string();
			                	}
			                }
			        	});

					ui.end_row();

					if self.file_type != Format::Image {
						return
					}
					if let Some(settings) = &mut self.image_settings {
						ui.label("⛶ Compress files");
						ui.checkbox(&mut settings.compress, "");
						ui.end_row();

						ui.label("↔ Flip");
						ui.checkbox(&mut settings.h_flip, "");
						ui.end_row();
						
						ui.label("↺ Rotation");
						ui.add(egui::Slider::new(&mut settings.rotate, -180..=180).suffix("°"));
						ui.end_row();

						ui.label("# Greyscale");
						ui.checkbox(&mut settings.greyscale, "");
						ui.end_row();

						ui.label("☁ Smoothing");
						ui.checkbox(&mut settings.smoothing, "");
						ui.end_row();
					} else {
						self.image_settings = Some(ImageSettings::default())
					}
				});

			ui.separator();
			if ui.add(egui::Button::new("Convert")).clicked() {
				println!("convert");
				if self.selected_extension == "" {
					return
				}
				if let Some(output_folder_borrow) = &self.output_folder {
					let input_files = self.input_files.clone();
					let selected_extension = self.selected_extension.clone();
					let file_type = self.file_type.clone();
					let output_folder = output_folder_borrow.clone();
					let image_settings = self.image_settings.clone();
					self.processing_handle = Some(thread::spawn(move || {
						match file_type {
							Format::Image => {
								let settings = if let Some(img_settings) = image_settings {
									img_settings
								} else {
									ImageSettings::default()
								};
								
								let quality = if settings.compress {15} else {85};
								
								let mut effects = HashMap::new();
								if settings.h_flip {
									effects.insert("symetrie".to_string(),"".to_string());
								}
								if settings.greyscale {
									effects.insert("niveaux_de_gris".to_string(),"".to_string());
								}
								if settings.smoothing {
									effects.insert("lissage".to_string(),"".to_string());
								}
								if settings.rotate != 0 {
									effects.insert("rotation".to_string(), format!("{}", settings.rotate));
								}
								let _ = python::convert_image(input_files, effects, output_folder, selected_extension, quality);
							}
							Format::Audio => {
								let _ = python::convert_audio(input_files, output_folder, selected_extension);
							}
							Format::Video => {
								let _ = python::convert_video(input_files, output_folder, selected_extension);
							}
							_ => {
								return
							}
						}
						println!("finished");
					}));
				}
				
				self.input_files = vec![];
				self.file_type = Format::None;
				self.selected_extension = "".to_string();
			}
		});
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
