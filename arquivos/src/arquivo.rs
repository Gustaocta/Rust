use std::env;
use std::fs::File;
use std::fs::metadata;
use std::io::prelude::*;
//use std::io::Write;

pub fn obter_caminho_usuario() -> Option<String> {

    if let Some(caminho_home) = env::var_os("HOME") {
        Some(caminho_home.into_string().unwrap())
    } else {
        None
    }

}

pub fn criar(caminho: &str, nome_arquivo: &str) {

    println!("Criando arquivo no caminho: {}", caminho);
    println!("Criando arquivo com o nome: {}", nome_arquivo);

    let caminho_completo = format!(r"{}\{}", caminho, nome_arquivo);

    match File::create(&caminho_completo) {
        Ok(_) => {
            println!("Arquivo criado com sucesso no caminho: {}", caminho_completo);
        },
        Err(e) => {
            println!("Erro ao criar o arquivo: {}", e);
        }
    }

}

pub fn existe(caminho_completo: &str) -> Result<(), &'static str> {

    if metadata(caminho_completo).is_ok() {
        Ok(())
    } else {
        Err("O arquivo não existe.")
    }

}