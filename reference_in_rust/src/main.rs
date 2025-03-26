fn main() {
    let mut i = 0;
    let i_ref = &mut i;
    *i_ref += 1;
    i += 1;
    let i_ref = &mut i;
    println!("{}", i_ref);
}
