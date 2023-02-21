mod complex;
fn main(){
    let mut z = complex::C::new(1, 1);
    z *= complex::C::new(0, 10);
    println!("{z}");

    z += complex::C::new(10, 10);
    println!("{z}");
}