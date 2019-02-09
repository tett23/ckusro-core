pub mod git_object;
use git_object::GitObject;
pub mod compressed_git_object;
use compressed_git_object::CompressedGitObject;

pub fn hoge() -> String {
    // let obj = GitObject::new(&"".as_bytes().to_vec());

    // println!("{}", obj);

    String::from("hoge")
}
