fn main() {
    let _x = 1_024 + 0xff + 0o77 + 0b1111_1111; //aqui estamos somando números em diferentes sistemas numéricos //decimal, hexadecimal, octal, binário //valores correspondentes: 1024, 255, 63, 255 = 1597
    assert_eq!(_x, 1597);
    println!("{}", _x);
    println!("success")
}
