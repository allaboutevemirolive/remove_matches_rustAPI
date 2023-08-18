pub fn generate() {
    for i in 1..=100 {
        if i % 3 == 0 {
            println!("{}", i);
        }
    }
}

fn main (){
    generate()
}
