use pyo3::{prelude::*, types::PyModule};
use std::collections::HashMap;
use crate::FileEntry;


pub fn find_file_name(input_path : String) -> PyResult<Vec<String>> {
	Python::with_gil(|py| {
		let code = include_str!("python/convert.py");
		let maybe_module = PyModule::from_code(py, code, "convert.py", "convert"); // je génère un module python a partir de code

		match maybe_module {
			Ok(module) => {														// s'il a réussi à le générer
				let function = module.getattr("find_file_name")?;				  // on cherche la fonction
				let args = (&input_path,);										 // on prépare les arguments
				let result : Vec<String> = function.call1(args)?.extract()?;	   // on appelle la fonction avec les arguments
				Ok(result)														 // on balance le résultat
			}
			Err(error) => {
				println!("no module : {}", error);
				Err(error)
			}
		}
	})
}

pub fn convert_audio(inputs : Vec<FileEntry>, output_folder : String, output_format : String) -> PyResult<()> {
	let input_paths : Vec<String> = inputs.into_iter().map(|f| f.path.clone()).collect();
	Python::with_gil(|py| {
		let code = include_str!("python/sound.py");
		let maybe_module = PyModule::from_code(py, code, "sound.py", "sound"); // je génère un module python a partir de code

		match maybe_module {
			Ok(module) => {														// s'il a réussi à le générer
				let function = module.getattr("audio_convert")?;				  // on cherche la fonction
				let args = (&input_paths.into_py(py), &output_folder, &output_format);										 // on prépare les arguments
				let _result : Vec<String> = function.call1(args)?.extract()?;	   // on appelle la fonction avec les arguments
				Ok(())														 // on balance le résultat
			}
			Err(error) => {
				println!("no audio module : {}", error);
				Err(error)
			}
		}
	})
}

pub fn convert_video(inputs : Vec<FileEntry>, output_folder : String, output_format : String) -> PyResult<()> {
	let input_paths : Vec<String> = inputs.into_iter().map(|f| f.path.clone()).collect();
	Python::with_gil(|py| {
		let code = include_str!("python/video.py");
		let maybe_module = PyModule::from_code(py, code, "video.py", "video"); // je génère un module python a partir de code

		match maybe_module {
			Ok(module) => {														// s'il a réussi à le générer
				let function = module.getattr("video_convert")?;				  // on cherche la fonction
				let args = (&input_paths.into_py(py), &output_folder, &output_format);										 // on prépare les arguments
				let _result : Vec<String> = function.call1(args)?.extract()?;	   // on appelle la fonction avec les arguments
				Ok(())														 // on balance le résultat
			}
			Err(error) => {
				println!("no video module : {}", error);
				Err(error)
			}
		}
	})
}

pub fn convert_image(inputs : Vec<FileEntry>, effects : HashMap<String, String>, output_folder : String,  output_format : String, quality : i16) -> PyResult<()> {
	let input_paths : Vec<String> = inputs.into_iter().map(|f| f.path.clone()).collect();
		Python::with_gil(|py| {
			let code = include_str!("python/image.py");
			let maybe_module = PyModule::from_code(py, code, "image.py", "image"); // je génère un module python a partir de code
	
			match maybe_module {
				Ok(module) => {														// s'il a réussi à le générer
					let function = module.getattr("traiter_comprimer_image")?;				  // on cherche la fonction
					let args = (&input_paths.into_py(py), &effects.into_py(py), &output_folder, &output_format, &quality.into_py(py));										 // on prépare les arguments
					let _result : Vec<String> = function.call1(args)?.extract()?;	   // on appelle la fonction avec les arguments
					Ok(())														 // on balance le résultat
				}
				Err(error) => {
					println!("no image module : {}", error);
					Err(error)
				}
			}
		})
}
