fn log(arr: &[i32]) {
    println!("{:?}", arr);
}

fn main() {
    let a = [0; 10];
    log(&a);

    // println!("{}", a);
}
