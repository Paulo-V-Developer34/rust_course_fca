fn main() {
    let mut sum: i32 = 0;

    for i in -3..2 {
        //esse é o for loop do Rust, o ".." significa de x até y //nesse caso o "2" não estará incluído
        sum += i;
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        //há diferença entre '' e "" ;-;
        //podemos fazer o mesmo com letras //nesse caso o "z" está incluído por causa do "="
        println!("{}", c);
    }

    for c in 'a'..='z' {
        //um valor na memória pode ser igual mas representar coisas diferentes de acordo com seu tipo, para saber mais observe a tabela ASCII
        //observe que a única coisa que mudamos aqui é o "as u8" e os valores são representados por números agora
        println!("{}", c as u8);
    }

    println!("success")
}
