use crate::Yubaba;
use crate::Format;
use crate::python;
use crate::ImageSettings;
use crate::egui;
use crate::totoro;
use std::collections::HashMap;
use std::thread;

impl Yubaba {
	pub fn show_file_panel(&mut self, ctx: &egui::Context) {
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
	}

	pub fn show_config_panel(&mut self, ctx: &egui::Context) {
		egui::CentralPanel::default().show(ctx, |ui| {
			if let Some(handle) = &self.processing_handle {
				ui.label("Processing...");
				ui.add(egui::Image::new(totoro::GIF[self.current_frame.floor() as usize].clone()).max_size(egui::Vec2 {x: 120.0, y: 120.0}));
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
