fn main() {
    let (mut x, y) = (5, 2); //aqui eu precisei declarar que x é mutável //isso é uma tupla
    x += 5;

    assert_eq!(x, 10);
    assert_eq!(y, 2);
}
