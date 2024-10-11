fn main() {
    let _x = 251_i16 + 8; //aqui tanto faz usar u ou i, mas é melhor usar u
                          // let _y = 251_u8 + 8; //esse valor não pode pois é muito baixo
    println!("{}", i16::MAX); //eu pensava que simplesmente i16 não era o suficiente, mas o maximo dele é 32767
    println!("success")
}
