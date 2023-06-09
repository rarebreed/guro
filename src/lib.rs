pub fn stack() {
    let x = 10;
    let y = 11;
    let const_ref_to_const_x = &x;
    println!("x: address = {:p}, value = {x}", &x);
    println!("y: address = {:p}, value = {y}", &y);
    println!(
        "const_ref_to_const_x: address = {const_ref_to_const_x:p}, value = {const_ref_to_const_x}"
    );

    let mut mut_ref_to_const_x = &x;
    println!("mut_ref_to_const_x: address = {mut_ref_to_const_x:p}, value = {mut_ref_to_const_x}");
    mut_ref_to_const_x = &y;
    println!("After reassignment:");
    println!("  mut_ref_to_const_x: address {mut_ref_to_const_x:p}, value = {mut_ref_to_const_x}");
    println!("  x: address = {:p}, value = {x}", &x);
    println!("  y: address = {:p}, value = {y}", &y);

    // Can't assign a value since it needs a reference
    //mut_ref_to_const_x = 10;  // error! expected &i32, got i32

    // Or maybe you think you can cheat and change x by assigning a new value to mut_ref_to_const_x?
    // *mut_ref_to_const_x = 100; // compiler error! can not assign to *mut_const_ref_to_x which is behing a & ref
}

pub fn stack2() {
    // But what if we create a mut ref?
    let mut z = 12;
    let mut mut_ref_to_mut_z = &mut z;
    println!("mut_ref_to_mut_z: address = {mut_ref_to_mut_z:p}");

    *mut_ref_to_mut_z = 100;
    // We can't println!(mut_ref_to_mut_z) anymore, but we can create a new variable that has the same address.
    // the borrow checker doesn't allow it.
    let a: &i32 = mut_ref_to_mut_z;
    println!("After assigning a to mut_ref_to_mut_z");
    println!("  a: address = {:p}, value = {a}", &a);
    println!("  mut_ref_to_mut_z: address = {:p}", mut_ref_to_mut_z);

    // But wait...what if I change where mut_ref_to_z is referencing??
    let mut b = 200;
    mut_ref_to_mut_z = &mut b;
    println!("After reassignment of z to 200:");
    println!("  a: address = {:p}, value = {a}", &a);
    println!("  mut_ref_to_mut_z: address = {:p}", mut_ref_to_mut_z);
    println!("  mut_ref_to_mut_z: value = {mut_ref_to_mut_z}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        stack2();
    }
}
