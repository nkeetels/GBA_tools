/*
    Raw converter converter for Under Audit 2
    Creates byte array from input file
*/

use std::io::Read;
use std::io::Write;
use std::fs::File;
use std::env;

fn main() {
    let mut input_filename = String::new();

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        input_filename.push_str(&args[1]);
    } else {
        println!("Example usage: raw_converer.exe data.bin\n");
        std::process::exit(0);
    }

    let mut read_file = match File::open(&input_filename) {
        Ok(file) => file, 
        Err(_err) => { println!("File {} not found!", input_filename); 
        return }
    };

    let mut buffer = Vec::new();
    match read_file.read_to_end(&mut buffer) {
        Ok(_) => println!("Parsing file {} of size {} bytes", input_filename, buffer.len()),
        Err(error) => println!("Error reading file: {}", error),
    };

    let name = input_filename.split('.').next().unwrap_or("");
    let output_filename = format!("{}.c", name);

    let mut write_file = match File::create(&output_filename) {
        Ok(file) => file, Err(_err) => { println!("Could not create {}!", output_filename); return } ,
    };    

    let mut output_str = String::new();

    let header_str = "#include <stdint.h>\n\n";
    output_str.push_str(header_str);

    let buffer_size = buffer.len();
    let buffer_size_str = format!("\nconst uint16_t {}_size = {};\n", name, buffer_size);
    output_str.push_str(&buffer_size_str);    

    let array_str = format!("{}{}{}", "const uint8_t ", name, "_data[] = {\n");
    output_str.push_str(&array_str);

    let mut pos = 0;
    while pos < buffer_size {
        output_str.push_str(&format!("{:#04X}, ", buffer[pos]));

        if pos & 15 == 0 {
            output_str.push_str("\n");
        }

        pos += 1;
    }    

    let end_str = "};\n";
    output_str.push_str(end_str);

    match write_file.write(output_str.as_bytes()) {
        Ok(_) => { println!("Exported as file {}", output_filename); } ,
        Err(_err) => { println!("Error writing to file {}", output_filename) } ,
    };    
}