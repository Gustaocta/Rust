fn main() {
    nome_da_funcao();

    let a = com_return();
    println!("O valor retornado em A é: {}", a);

    let b = sem_return();
    println!("O valor retornado em B é: {}", b);

    let c = maior_valor(10, 20);
    println!("O maior valor entre {} e {} é: {}", 10, 20, c);

    let d = 5;

    let resultado_incremento = incrementa(d);
    println!("O número original: {}, o número incrementado {}", d, resultado_incremento);
}

fn nome_da_funcao() {
    println!("Olá, Hcode!");
}

fn com_return() -> i8 {
   return 3;
}

fn sem_return() -> i8 {
    3
}

fn maior_valor(a:i32, b:i32) -> i32 {

    {
        if a>b {
           a
        } else {
            b
        }
    }
}

fn incrementa(mut a: i32) -> i32 {
    a += 1;
    a
}
