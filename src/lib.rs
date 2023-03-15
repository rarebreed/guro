pub fn stack() {
    let x = 42;
    let y = 43;
    let var1 = &x;
    println!("x = {x}, y = {y}, var1 = {var1}");

    let mut var2 = &x;
    println!("var2 = {var2}, address of var2 = {var2:p}");
    var2 = &y;
    println!("After reassignment: var2 is now {var2:p}, var1 = {var1}, x = {x}");

    println!(
        "address of x = {:p}, address of var1 = {var1:p}, address of var2 = {:p}",
        &x, &var2
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        stack();
    }
}
