pub struct PathFragment {
  domain: String,
  user: String,
  repository: String,
}

impl PathFragment {
  fn is_valid_format(fragment: &str) -> bool {
    let user_separator_pos = fragment.find('@');
    if user_separator_pos.is_none() {
      return false;
    }

    let repository_separator_pos = fragment.find(':');
    if repository_separator_pos.is_none() {
      return false;
    }

    user_separator_pos < repository_separator_pos
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod path_fragment {
    use super::*;

    mod is_valid_format {
      use super::*;

      #[test]
      fn it_works() {
        let fragment = "github.com@tett23:ckusro-core";
        let actual = PathFragment::is_valid_format(fragment);
        // let expected = ("github.com", "tett23", "ckusro-core");

        assert!(actual);
      }

      #[test]
      fn when_the_argument_does_not_include_user_separator() {
        let fragment = "tett23:ckusro-core";
        let actual = PathFragment::is_valid_format(fragment);

        assert!(!actual);
      }

      #[test]
      fn when_the_argument_does_not_include_repository_separator() {
        let fragment = "github.com@tett23";
        let actual = PathFragment::is_valid_format(fragment);

        assert!(!actual);
      }

      #[test]
      fn when_the_argument_is_malformed_format() {
        let fragment = "ckusro-core:github.com@tett23";
        let actual = PathFragment::is_valid_format(fragment);

        assert!(!actual);
      }
    }
  }
}
