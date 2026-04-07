use compiler::lexer::lexer;
use compiler::parser::parser;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = if args.len() > 1 {
        &args[1]
    } else {
        "./test.asm"
    };

    let content =
        fs::read_to_string(file_path).expect("No se pudo leer el archivo. Verifica la ruta.");

    let tokens = lexer(content);

    let parsed = parser(tokens);

    let output_name = file_path.replace(".asm", "").replace("./", "");
    compiler::writer::write_hex(parsed.clone(), output_name.clone());

    println!("Archivo compilado con exito: {}.hex", output_name);
}
