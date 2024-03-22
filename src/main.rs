use pyo3::{prelude::*, types::{IntoPyDict, PyModule}};
use eframe::egui;

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


struct Yubaba {}

impl Default for Yubaba {
	fn default() -> Self {
		Self {}
	}
}

impl eframe::App for Yubaba {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.label("Hello there");
		});
	}
}

fn find_file_name(input_path : String) -> PyResult<Vec<String>> {
	Python::with_gil(|py| {
    	let code = include_str!("python/convert.py");
    	let maybe_module = PyModule::from_code(py, code, "convert.py", "convert");

    	match maybe_module {
    		Ok(module) => {
    			println!("got activator");
    			let function = module.getattr("find_file_name")?;
    			let args = (&input_path,);
		    	let result : Vec<String> = function.call1(args)?.extract()?;
		    	println!("result : {:?}", result);
				Ok(result)
    		}
    		Err(error) => {
    			println!("no activator : {}", error);
    			Err(error)
    		}
    	}
    })
}
