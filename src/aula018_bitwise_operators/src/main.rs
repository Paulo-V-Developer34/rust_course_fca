fn main() {
    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011 & 0b0101); //operação AND //os números que estão aqui não números binários //observe que não faz diferença eu utilizar "u32" ou não
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101); //essa é a operação XOR, que defini valores diferentes como true
    println!("1 << 5 is {}", 1u32 << 5); //aqui estamos movendo o número 1 dois bits para a esquerda
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2); //estamos fazendo o mesmo, porém para a direita
    println!("success")
}
