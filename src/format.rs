const IMAGE_EXTENSION : [&str; 4] = ["jpeg", "png", "jpg", "webp"];
const AUDIO_EXTENSION : [&str; 3] = ["mp3", "wav", "ogg"];
const VIDEO_EXTENSION : [&str; 4] = ["mp4", "mkv", "mov", "avi"];
const ALL_EXTENSION : [&str; 11] = ["jpeg", "png", "jpg", "webp", "mp3", "wav", "ogg", "mp4", "mkv", "mov", "avi"];

#[derive(PartialEq)]
pub enum Format {
	Image,
	Audio,
	Video,
	None,
}

impl Format {
	pub fn from_extension(extension : String) -> Self {
		if IMAGE_EXTENSION.contains(&extension.as_str()) {
			return Self::Image
		}
		if AUDIO_EXTENSION.contains(&extension.as_str()) {
			return Self::Audio
		}
		if VIDEO_EXTENSION.contains(&extension.as_str()) {
			return Self::Video
		}
		return Self::None
	}
	
	pub fn get_extensions(&self) -> Vec<String> {
		match self {
			Self::Image => IMAGE_EXTENSION.to_vec().into_iter().map(|e| e.to_string()).collect(),
			Self::Audio => AUDIO_EXTENSION.to_vec().into_iter().map(|e| e.to_string()).collect(),
			Self::Video => VIDEO_EXTENSION.to_vec().into_iter().map(|e| e.to_string()).collect(),
			Self::None => ALL_EXTENSION.to_vec().into_iter().map(|e| e.to_string()).collect(),
		}
	}
	
	pub fn to_filter(&self) -> String {
		match self {
			Self::Image => "image".to_string(),
			Self::Audio => "audio".to_string(),
			Self::Video => "video".to_string(),
			Self::None => "multimedia".to_string(),
		}
	}
	
	pub fn display(&self) -> String {
		match self {
			Self::Image => "🎨 ".to_string(),
			Self::Audio => "🎧 ".to_string(),
			Self::Video => "🎥 ".to_string(),
			Self::None => "  ".to_string(),
		}
	}
}
