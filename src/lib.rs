use crate::repo::clone::GitCloner;

mod repo;

pub fn sheep_test() {
    let cloner = GitCloner::new();
    println!("cloner {:?}", cloner);
    println!("cloner {:?}", cloner);
}