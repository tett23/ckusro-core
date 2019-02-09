mod git_object;
use git_object::GitObject;
mod compressed_git_object;
use compressed_git_object::CompressedGitObject;

pub fn hoge() -> String {
    let obj = GitObject::new("".as_bytes());

    println!("{}", obj);

    return String::from("hoge");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = hoge();

        assert_eq!(&a, "hoge");
    }
}
