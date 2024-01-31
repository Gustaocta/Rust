use std::fs::File;
use std::io::{self, Read};
fn main() {
    //funcao_com_panic(0);

    let resultado =std::panic::catch_unwind(|| {

        let a = funcao_com_panic(20);

        Ok::<i32, &str>(a)

    });

    match resultado {
        Ok(valor) => {
            println!("Resultado usando result: {}", valor.unwrap());
        }, 
        Err(_) => {
            println!("A função resultou em panic");
        }
    } 

    println!("*********************************************");

    let resultado = ler_arquivo(r"C:\Users\gustavo.otacilio\Rust\erros\src\main.rs");

    match resultado {
        Ok(conteudo) => {
            println!("O conteudo do arquivo {}", conteudo);
        },
        Err(erro) => {
            println!("Erro ao ler arquivo: {}", erro);
        }
    }

    println!("*********************************************");

    let a: f64 = 2.0;

    let resultado_divisao = dividir(100 as f64, a);

    match resultado_divisao {
        Some(valor) => {
            println!("O resultado da divisão é: {}", valor);
        },
        None => {
            println!("Não foi possível fazer a divisão!");
        }
     }

}

fn funcao_com_panic(valor:i32) -> i32 {
    if valor == 0 {
        panic!("O valor não pode ser zero");
    }

    valor
}

fn ler_arquivo(caminho: &str) -> Result<String, io::Error >  {

    let mut arquivo = File::open(caminho)?;

    let mut conteudo = String::new();

    arquivo.read_to_string(&mut conteudo);

    Ok(conteudo)

}

fn dividir(dividendo: f64, divisor: f64) -> Option<f64> {

    if divisor == 0.0 {
        None
    } else {
        Some(dividendo / divisor)
    }

}