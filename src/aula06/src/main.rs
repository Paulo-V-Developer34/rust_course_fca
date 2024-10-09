fn main() {
    let (x, y); //não preciso declarar que eles são mutáveis visto que eu ainda não lhes atribuí valores

    //o ".." serve meramente para preencher espaço, ele não armazenará nada
    (x, ..) = (3, 4); //isso é um array
    [.., y] = [1, 2]; //isso é uma tupla
    assert_eq!([x, y], [3, 2]);
    println!("success")
}
