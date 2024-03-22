use pyo3::{prelude::*, types::PyModule};
use eframe::egui;
use std::path::PathBuf;

const IMAGE_EXTENSION : [&str; 3] = ["png", "jpg", "webp"];

//🎥 🎧 🎨
fn main() -> Result<(), eframe::Error> {
    if let Ok(test) = find_file_name("etc/test/turtle.webp".to_string()) {
    	println!("python call successful : etc/test/turtle.webp => {:?}", test);
    }

    let options = eframe::NativeOptions {
   		viewport: egui::ViewportBuilder::default()
   			.with_inner_size([1200.0, 800.0]),
   		..Default::default()
   	};

   	eframe::run_native(
		"Yubaba",
		options,
		Box::new(move |_cc| Box::from(Yubaba::default())),
	)
}

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

struct Yubaba {
	input_files: Vec<FileEntry>,
	output_folder: Option<String>,
	available_extensions: Vec<String>,
	selected_extension: String,
	compress: bool,
}

impl Default for Yubaba {
	fn default() -> Self {
		Self {
			input_files: vec![],
			output_folder: None,
			available_extensions: vec![],
			selected_extension: "".to_string(),
			compress: false,
		}
	}
}

impl eframe::App for Yubaba {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::SidePanel::left("file_tree_panel").show(ctx, |ui| {
			if ui.add(egui::Button::new("📁 Open Files")).clicked() {
				if let Some(paths) = rfd::FileDialog::new().pick_files()
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
			}
		});

		egui::CentralPanel::default().show(ctx, |ui| {
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
			                for extension in &self.available_extensions {
			                	if ui.add(egui::SelectableLabel::new(&self.selected_extension == extension, extension)).clicked() {
			                	    self.selected_extension = extension.to_string();
			                	}
			                }
			        	});

					ui.end_row();

					ui.label("⛶ Compress files");
					ui.checkbox(&mut self.compress, "");

					ui.end_row();
				});

			ui.separator();
			if ui.add(egui::Button::new("Convert")).clicked() {
				println!("convert");
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
			if let Ok(file_data) = find_file_name(path_str.clone()) {
				if self.output_folder.is_none() {
					self.select_folder(PathBuf::from(file_data[0].clone()));
				}
				new_file.name = file_data[1].clone();
				new_file.extension = file_data[2].clone();
				if IMAGE_EXTENSION.contains(&file_data[2].as_str()) && self.available_extensions.is_empty() {
					self.available_extensions = IMAGE_EXTENSION.to_vec().into_iter().map(|e| e.to_string()).collect();
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
}




fn find_file_name(input_path : String) -> PyResult<Vec<String>> {
	Python::with_gil(|py| {
    	let code = include_str!("python/convert.py");
    	let maybe_module = PyModule::from_code(py, code, "convert.py", "convert");

    	match maybe_module {
    		Ok(module) => {
    			let function = module.getattr("find_file_name")?;
    			let args = (&input_path,);
		    	let result : Vec<String> = function.call1(args)?.extract()?;
				Ok(result)
    		}
    		Err(error) => {
    			println!("no activator : {}", error);
    			Err(error)
    		}
    	}
    })
}
