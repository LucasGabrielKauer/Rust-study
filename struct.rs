//Se você não tem o ambiente configurado mas  quiser ver o código executando acesse o site :  site play.rust-lang.org
//Implementação de struct (modelo de dados)
//Implementação impl (para metodos da struct)

struct Person{

    name : String,
    age : i32
}

impl Person {
    fn sayhi(&self) -> String {
        format!("Meu nome é {}, eu tenho {} anos", self.name, self.age)
    }
}

fn main(){

    let person = Person {name : "Lucas".to_string(), age : 24};
    let person2 = Person {name : "Rose".to_string(), age : 30};
    println!("{}", person.sayhi());
    println!("{}", person2.sayhi());

}