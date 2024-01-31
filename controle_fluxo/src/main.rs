fn main() {

    se();

    declaracao_if();

    loop_infinito();

    loop_finito();

    for_interator();

    deu_match();

}

fn se(){

    let numero = 0;
    if numero > 0 {
        println!("O número {} é positivo.", numero);
    } else if numero == 0{
        println!("O número é 0.");
    } else {
        println!("O número {} é negativo.", numero);
    }

}

fn declaracao_if () {
    let condicao = true;

    let resultado = if condicao {
        "A condição é verdadeira!"
    } else {
        "A condição é falsa!"
    };

    println!("{}", resultado);

}

fn loop_infinito() {

    let mut contador = 0;

    loop {
        println!(" Contatdor: {}", contador);

        contador += 1;

        if contador == 5 {
            break;
        }
    }

}

fn loop_finito() {

    let mut contador = 0;

    while contador < 5 {

        println!(" Contatdor: {}", contador);

        contador += 1;

    }

}

fn for_interator() {

    for i in 0..=10 {
        println!("O index é: {}", i);

    }

}

fn deu_match() {

    let estacao_atual = "primavera";

    match estacao_atual {
        "primavera" => {
            println!("É primavera, meu amor!");
        },
        "verão" => {
            println!("É verão, muito calor!");
        },
        "outono" => {
            println!("É outono, as folhas estão caindo!");
        },
        "inverno" => {
            println!("É inverno, está frio lá fora!");
        },
        _ => {
            println!("Estação desconhecida!");
        }
    }

}
