fn main() {
    assert!(0.1 as f32 + 0.2 as f32 == 0.3 as f32); //diferentemente do "assert_eq! (equals)" este nós devemos explicitar qual é o tipo de comparação a ser feita // em Rust e em outras linguagens de baixo nível temos que 0.1 + 0.2 = 0.300000000004 ou algo assim, para isso basta diminuirmos a quantidades de bits a serem usados
    println!("success")
}
