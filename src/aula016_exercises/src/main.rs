fn main() {
    // Integer addition
    // assert!(1u32 + 2 == 3); //error por causa que estou comparando um i32 com um u32
    assert!(1_u32 + 2_u32 == 3_u32);

    // // Integer subtraction //error
    // assert!(1i32 - 2 == 3);
    // assert!(1u8 - 2 == -1);//o tipo "u" não aceita números negativos
    // Integer subtraction
    assert!(1 - 2 == -1);
    assert!(1_i8 - 2_i8 == -1_i8);

    assert!(3 * 50 == 150);

    // assert!(9.6 / 3.2 == 3.0); // error ! make it work //aqui temos o problema em relação à precisão do ponteiro dos números floats (por padrão são f64)
    assert!(9.6_f32 / 3.2_f32 == 3.0_f32);

    assert!(24 % 5 == 4);
    println!("success")
}
