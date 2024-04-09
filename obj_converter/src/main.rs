/*
    Wavefront obj converter for Under Audit 2
    Either exports texcoords in UV or normals in UVW, normals take precedence

    N.B. vertices are exported with 16-bit signed components so scale your object accordingly.
*/

use wavefront::Obj;
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
        println!("Example usage: obj_converer.exe scene.obj\n");
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

    let obj = Obj::from_reader(&buffer[..]).unwrap();

    let to_fixed: f32 = 16383.0;
    let mut triangle_count: i32 = 0;
    let mut has_texcoords: bool = false;
    let mut has_normals: bool = false;

    let mut output_str = String::new();

    let header_str = "#include <stdint.h>\n\n";
    output_str.push_str(header_str);

    let array_str = format!("{}{}{}", "const int16_t ", name, "_model[] = {\n");
    output_str.push_str(&array_str);

    for [a, b, c] in obj.triangles() {
      
        let v0_x = (a.position()[0] * to_fixed) as i32;
        let v0_y = (a.position()[1] * to_fixed) as i32;
        let v0_z = (a.position()[2] * to_fixed) as i32;

        let v1_x = (b.position()[0] * to_fixed) as i32;
        let v1_y = (b.position()[1] * to_fixed) as i32;
        let v1_z = (b.position()[2] * to_fixed) as i32;

        let v2_x = (c.position()[0] * to_fixed) as i32;
        let v2_y = (c.position()[1] * to_fixed) as i32;
        let v2_z = (c.position()[2] * to_fixed) as i32;

        let mut v0_u: i32 = 0;
        let mut v0_v: i32 = 0;
        let mut v0_w: i32 = 0;

        let mut v1_u: i32 = 0;
        let mut v1_v: i32 = 0;
        let mut v1_w: i32 = 0;

        let mut v2_u: i32 = 0;
        let mut v2_v: i32 = 0;
        let mut v2_w: i32 = 0;

        let v0_uv = a.uv();
        if let Some(v0_uv) = v0_uv {
            v0_u = (v0_uv[0] * to_fixed) as i32;
            v0_v = (v0_uv[1] * to_fixed) as i32;

            has_texcoords = true;
        }

        let v1_uv = b.uv();
        if let Some(v1_uv) = v1_uv {
            v1_u = (v1_uv[0] * to_fixed) as i32;
            v1_v = (v1_uv[1] * to_fixed) as i32;
        }

        let v2_uv = c.uv();
        if let Some(v2_uv) = v2_uv {
            v2_u = (v2_uv[0] * to_fixed) as i32;
            v2_v = (v2_uv[1] * to_fixed) as i32;
        }

        let v0_normal = a.normal();
        if let Some(v0_normal) = v0_normal {
            v0_u = (v0_normal[0] * to_fixed) as i32;
            v0_v = (v0_normal[1] * to_fixed) as i32;
            v0_w = (v0_normal[2] * to_fixed) as i32;

            has_normals = true;
        }

        let v1_normal = b.normal();
        if let Some(v1_normal) = v1_normal {
            v1_u = (v1_normal[0] * to_fixed) as i32;
            v1_v = (v1_normal[1] * to_fixed) as i32;
            v1_w = (v1_normal[2] * to_fixed) as i32;          
        }

        let v2_normal = c.normal();
        if let Some(v2_normal) = v2_normal {
            v2_u = (v2_normal[0] * to_fixed) as i32;
            v2_v = (v2_normal[1] * to_fixed) as i32;
            v2_w = (v2_normal[2] * to_fixed) as i32;            
        }

        let vertex_str0 = format!("{:?}, {:?}, {:?}, {:?}, {:?}, {:?},\n", v0_x, v0_y, v0_z, v0_u, v0_v, v0_w);
        let vertex_str1 = format!("{:?}, {:?}, {:?}, {:?}, {:?}, {:?},\n", v1_x, v1_y, v1_z, v1_u, v1_v, v1_w);
        let vertex_str2 = format!("{:?}, {:?}, {:?}, {:?}, {:?}, {:?},\n", v2_x, v2_y, v2_z, v2_u, v2_v, v2_w);

        output_str.push_str(&vertex_str0);
        output_str.push_str(&vertex_str1);
        output_str.push_str(&vertex_str2);

        triangle_count += 1;
    }

    let end_str = "};\n";
    output_str.push_str(end_str);

    let const_str = format!("\n{}{}{}{:?}{}", "const uint16_t ", name, "_triangles = ", triangle_count, ";");
    output_str.push_str(&const_str);
    
    let has_texcoords_str = format!("\nconst bool {}_has_texcoords = {};", name, has_texcoords);
    output_str.push_str(&has_texcoords_str);

    let has_normals_str = format!("\nconst bool {}_has_normals = {};", name, has_normals);
    output_str.push_str(&has_normals_str);

    match write_file.write(output_str.as_bytes()) {
        Ok(_) => { println!("Exported as file {}", output_filename); } ,
        Err(_err) => { println!("Error writing to file {}", output_filename) } ,
    };    
}