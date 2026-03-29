use std::fs::write;

pub fn write_hex(tokens: Vec<u16>, file_name: String) {
    let path: String = format!("./{}.hex", file_name);
    
    let mut contents = String::from("v2.0 raw\n");
    contents.push_str(
        &tokens
            .iter()
            .map(|token| format!("{:04X}", token))
            .collect::<Vec<String>>()
            .join("\n")
    );
        
    write(&path, contents).unwrap_or_else(|_| panic!("Error al escribir el archivo: {}", path));
}
