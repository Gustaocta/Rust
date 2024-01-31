enum Fruta {
    Maca,
    Banana,
    Morango,
    Acai
}

enum Coordenada {
    DoisD(i32, i32),
    TresD(i32, i32, i32)
}

struct Pessoa {
    nome: String,
    idade: i8,
    altura: f32
}

struct Retangulo {

    altura: u32,
    largura: u32,

}

trait FormaGeometrica {
    fn calcular_area(&self) -> u32;

    fn new(largula: u32, altura: u32) -> Self;

}    


impl FormaGeometrica for Retangulo {
    
    fn calcular_area(&self) -> u32 {
        self.largura * self.altura
    } 

    fn new(largura: u32, altura: u32) -> Self {
        Self {altura, largura}
    }

}

fn main() {
    enumeracao(Fruta::Morango);
    enumeracao(Fruta::Maca);
    enumeracao(Fruta::Banana);
    enumeracao(Fruta::Acai);
    enumeracao_com_dados();

    estrutura();

}

fn estrutura() {

    let glaucio = Pessoa {
        nome: String::from("Glaucio"), 
        idade: 25, 
        altura: 1.69
     };

     println!("Nome: {}", glaucio.nome);
     println!("Idade: {}", glaucio.idade);
     println!("Altura: {}", glaucio.altura);

     let retangulo1 = Retangulo::new(10,20);

    let area1 = retangulo1.calcular_area();    

    let retangulo2 = Retangulo {
        largura: 33,
        altura: 5
     };

    let area2 = retangulo2.calcular_area();

    println!("Rec 1: {}", area1);
    println!("Rec 2: {}", area2);

}

fn enumeracao(fruta: Fruta)  {

    match fruta {
        Fruta::Maca => println!("É uma Maçã."),
        Fruta::Banana => println!("É uma Banana."),
        Fruta::Morango => println!("É um Morango."),
        Fruta::Acai => println!("É um Açaí."),
    }

}

fn enumeracao_com_dados() {

    let ponto2d = Coordenada::DoisD(5, 10);

    let ponto3d = Coordenada::TresD(3, 8, 15);

    match ponto2d {
        Coordenada::DoisD(x, y ) => println!("Coordenada 2d: {}, {}", x, y),
        Coordenada::TresD(_, _, _) => println!("Coordenada 3d")
    }

    match ponto3d {
        Coordenada::DoisD(_, _) => println!("Coordenada 2d"),
        Coordenada::TresD(x, y, z) => println!("Coordenada 3d: {}, {}, {}", x, y, z)
    }

}
