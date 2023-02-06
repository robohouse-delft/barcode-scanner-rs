use barcode_scanner::BarcodeScanner;

fn do_main() -> Result<(), ()> {
	let args: Vec<_> = std::env::args().collect();
	if args.len() != 2 {
		let prog_name = args[0].rsplit_once('/').map(|(_parent, name)| name).unwrap_or(&args[0]);
		eprintln!("Usage: {prog_name} DEVICE_PATH");
		return Err(())
	}

	let device_path = &args[1];
	let mut scanner = BarcodeScanner::open(device_path)
		.map_err(|e| eprintln!("{e}"))?;

	println!("Ready to scan barcodes");
	loop {
		let barcode = scanner.read().map_err(|e| println!("Failed to read barcode: {e}"))?;
		println!("{barcode}");
	}
}
fn main() {
	if let Err(_barcode_scanner_error) = do_main() {
		std::process::exit(1);
	}
}
