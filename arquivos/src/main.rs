mod arquivo;

use arquivo::{criar, obter_caminho_usuario};

fn main() {

    let caminho = obter_caminho_usuario().unwrap();

    existe(&caminho);

    criar(&caminho, &"hcode.txt"); 
}
