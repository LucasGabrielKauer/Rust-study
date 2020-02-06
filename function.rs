//Se você não tem o ambiente configurado mas  quiser ver o código executando acesse o site :  site play.rust-lang.org
//Rust retorna a ultima expressão da função que não contenha ';'  então não se faz necessário o uso do mesmo
//Na ultima expressão não se faz necessário o uso do return, pois é interpretado de forma implicita

fn soma(a: u64 , b :u64) -> u64 {
    a + b
}

fn mult(a: u64 , b :u64) -> u64 {
    a * b
}

fn main(){

    println!("O resultado de 3 + 5 é : {}", soma(3,5));
    println!("O resultado de 3 + 5 é : {}", mult(3,5));
}