# barcode-scanner

Scan 1D barcodes using a hand scanner for Rust.

The `barcode-scanner` crate provides a Linux interface to barcode USB hand scanners. It works with the `Device` struct from the `evdev` crate.

Currently supported features:
* One [`BarcodeScanner`] struct for all USB hand scanners that operate as a keyboard.
* Prevent other clients from receiving events from the selected device by grabbing it.
* Read 1D barcode consisting of numbers and letters.
* Omit special characters in a barcode.

## Example
This example grabs a hand scanner and prints a barcode that is read.

```rust
	use barcode_scanner::BarcodeScanner;

	let scanner = BarcodeScanner::new("usb-0000:00:14.0-3/input0")?;
	loop {
    	let barcode = scanner.read()?;
    	println!("{}", barcode);
	}
```

License: BSD-2-Clause OR Apache-2.0
