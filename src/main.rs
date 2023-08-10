fn main() {
    let dir = tempfile::tempdir().
        expect("unable to create tmp dir");
    let path = dir.path();
    println!("{:#?}", path);
    println!("Hello, world!");
}
