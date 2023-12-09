pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub mod user;
pub mod book;
mod address;
mod name;


pub fn user_info() {
    println!("我是userInfo")
}

pub fn user_name() {
    println!("我是user_name")
}

pub fn get_address() {
    address::get();
}

pub fn get_name() {
    name::get_name();
}
