fn main() {
    let x: u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x)); //Método "to_string" para transformá-lo em uma string //type_of não existe naturalmente, por isso o criamos em uma outra função
    println!("success")
}

fn type_of<T>(_: &T) -> String {
    //O "T" representa o tipo //O "_" representa Um valo genérico
    format!("{}", std::any::type_name::<T>()) //i32 //o type_name é o "verdadeiro type_of" do Rust
}
