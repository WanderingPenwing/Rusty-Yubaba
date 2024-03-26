use eframe::egui;
use image::GenericImageView;
use std::error::Error;

pub fn load_icon() -> Result<egui::IconData, Box<dyn Error>> {
	let (icon_rgba, icon_width, icon_height) = {
		let icon = include_bytes!("../assets/yubaba.ico");
		let image = image::load_from_memory(icon)?;
		let rgba = image.clone().into_rgba8().to_vec();
		let (width, height) = image.dimensions();
		(rgba, width, height)
	};

	Ok(egui::IconData {
		rgba: icon_rgba,
		width: icon_width,
		height: icon_height,
	})
}


#[derive(Clone)]
pub struct FileEntry {
	pub name: String,
	pub extension: String,
	pub path: String,
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
pub struct ImageSettings {
	pub compress: bool,
	pub h_flip: bool,
	pub greyscale: bool,
	pub rotate: i16,
	pub smoothing: bool,
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
