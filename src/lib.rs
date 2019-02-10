pub mod bundled_repository;
pub mod compressed_git_object;
pub mod error;
pub mod git_object;
pub mod namespace;
pub mod namespace_ref;

pub fn hoge() -> String {
    // let obj = GitObject::new(&"".as_bytes().to_vec());

    // println!("{}", obj);

    String::from("hoge")
}
