use crate::name::people::people_do;
use super::multip;

mod people;
mod eat;

pub fn get_name() {
    println!("我是 get_name");
    people_do();
    // let a = 3;
    // let b = 5;
    let (a, b) = (3, 5);
    let numb = multip::res_multip(a, b);
    println!("我是数学家{}乘{}等于{}", a, b, numb)
}