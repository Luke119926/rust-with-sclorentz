use std::io; // imports

fn main() {
    println!("Hello, world!");

    let var = "hello world";
    println!("{var}");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input) // criar um input e salvar na variavel 'input'
        .expect("failed to read line"); // caso não seja possivel ler o input
    
    let _input: u32 = input // u32 se define como somente númerico
        .trim() // remove do input, elementos de whitespace como "/n" ou espaço
        .parse() // converte a string para um número
        .expect("please type a number!"); // o que será imprimido caso o valor de input não corresponder ao tipo solicitado

    println!("{input}");
}