use pyo3::{prelude::*, types::PyModule};
use native_dialog::{FileDialog, MessageDialog, MessageType};
use eframe::egui;
use std::path::PathBuf;

fn main() -> Result<(), eframe::Error> {
    let _test = find_file_name("etc/test/turtle.webp".to_string());

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


struct Yubaba {
	input_files: Vec<FileEntry>,
	output_folder: Option<String>,
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

impl Default for Yubaba {
	fn default() -> Self {
		Self {
			input_files: vec![],
			output_folder: None,
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
			ui.label("Hello there");
			if ui.add(egui::Button::new("📦 Output Folder")).clicked() {
				if let Some(path) = rfd::FileDialog::new().pick_folder()
				{
					self.select_folder(path);
				}
			}

			if let Some(folder) = &self.output_folder {
				ui.label(folder);
			} else {
				ui.label("no folder selected");
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
				self.select_folder(PathBuf::from(file_data[0].clone()));
				new_file.name = file_data[1].clone();
				new_file.extension = file_data[2].clone();
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
