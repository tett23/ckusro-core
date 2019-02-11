use super::error::Error;

#[derive(PartialEq, Debug)]
pub struct PathFragment {
  pub domain: String,
  pub user: String,
  pub repository: String,
}

impl PathFragment {
  const USER_SEPARATOR: char = '@';
  const REPOSITORY_SEPARATOR: char = ':';

  pub fn parse_full_qualified_fragment(fragment: &str) -> Result<PathFragment, Error> {
    if !PathFragment::is_full_qualified_fragment(fragment) {
      return Err(Error::MalformedFragment(fragment.to_owned()));
    }

    match PathFragment::split_path_element(fragment) {
      (Some(domain), Some(user), Some(repository)) => Ok(PathFragment {
        domain: domain.to_owned(),
        user: user.to_owned(),
        repository: repository.to_owned(),
      }),
      _ => Err(Error::MalformedFragment(fragment.to_owned())),
    }
  }

  fn is_full_qualified_fragment(fragment: &str) -> bool {
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

  fn split_path_element(fragment: &str) -> (Option<&str>, Option<&str>, Option<&str>) {
    let (repository, rest) = PathFragment::split_repository_and_rest(fragment);
    let (domain, user) = match rest {
      Some(v) => PathFragment::split_domain_and_rest(v),
      None => (None, None),
    };

    let (domain, user) = match (domain, user) {
      (Some(d), Some(u)) => (Some(d), Some(u)),
      (Some(d), None) => (None, Some(d)),
      _ => (None, None),
    };

    (domain, user, repository)
  }

  fn split_domain_and_rest(fragment: &str) -> (Option<&str>, Option<&str>) {
    let vec: Vec<&str> = fragment.splitn(2, PathFragment::USER_SEPARATOR).collect();

    match vec.len() {
      1 => match vec.first() {
        Some(&"") => (None, None),
        Some(v) => (Some(v), None),
        None => (None, None),
      },
      _ => (Some(*vec.first().unwrap()), Some(*vec.last().unwrap())),
    }
  }

  fn split_repository_and_rest(fragment: &str) -> (Option<&str>, Option<&str>) {
    let vec: Vec<&str> = fragment
      .splitn(2, PathFragment::REPOSITORY_SEPARATOR)
      .collect();

    match vec.len() {
      1 => match vec.last() {
        Some(&"") => (None, None),
        Some(v) => (Some(v), None),
        None => (None, None),
      },
      _ => (Some(*vec.last().unwrap()), Some(*vec.first().unwrap())),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod path_fragment {
    use super::*;

    mod parse_full_qualified_fragment {
      use super::*;

      #[test]
      fn it_works() {
        let fragment = "github.com@tett23:ckusro-core";
        let actual = PathFragment::parse_full_qualified_fragment(fragment);
        let expected = Ok(PathFragment {
          domain: "github.com".to_owned(),
          user: "tett23".to_owned(),
          repository: "ckusro-core".to_owned(),
        });

        assert_eq!(actual, expected);
      }

      #[test]
      fn it_does_not_works() {
        let data = vec![
          "",
          "tett23:ckusro-core",
          "github.com@tett23",
          "ckusro-core:github.com@tett23",
        ];
        for datum in data {
          let actual = PathFragment::parse_full_qualified_fragment(datum);

          assert!(actual.is_err());
        }
      }
    }

    mod is_full_qualified_fragment {
      use super::*;

      #[test]
      fn it_works() {
        let data = vec![("github.com@tett23:ckusro-core", true)];

        for datum in data {
          let (fragment, expected) = datum;
          let actual = PathFragment::is_full_qualified_fragment(fragment);

          assert_eq!(actual, expected);
        }
      }

      #[test]
      fn when_the_argument_does_not_include_user_separator() {
        let data = vec![
          ("tett23:ckusro-core", false),
          ("ckusro-core", false),
          ("", false),
        ];

        for datum in data {
          let (fragment, expected) = datum;
          let actual = PathFragment::is_full_qualified_fragment(fragment);

          assert_eq!(actual, expected);
        }
      }
    }

    mod split_path_element {
      use super::*;

      #[test]
      fn it_works() {
        let data = vec![
          (
            "github.com@tett23:ckusro-core",
            (Some("github.com"), Some("tett23"), Some("ckusro-core")),
          ),
          (
            "tett23:ckusro-core",
            (None, Some("tett23"), Some("ckusro-core")),
          ),
          ("ckusro-core", (None, None, Some("ckusro-core"))),
          ("", (None, None, None)),
        ];

        for datum in data {
          let (fragment, expected) = datum;
          let actual = PathFragment::split_path_element(fragment);

          assert_eq!(actual, expected);
        }
      }
    }

    mod split_domain_and_rest {
      use super::*;

      #[test]
      fn it_works() {
        let data = vec![
          (
            "github.com@tett23:ckusro-core",
            (Some("github.com"), Some("tett23:ckusro-core")),
          ),
          ("github.com@tett23", (Some("github.com"), Some("tett23"))),
          ("github.com", (Some("github.com"), None)),
          ("", (None, None)),
        ];

        for datum in data {
          let (fragment, expected) = datum;
          let actual = PathFragment::split_domain_and_rest(fragment);

          assert_eq!(actual, expected);
        }
      }
    }

    mod split_repository_and_rest {
      use super::*;

      #[test]
      fn it_works() {
        let data = vec![
          (
            "github.com@tett23:ckusro-core",
            (Some("ckusro-core"), Some("github.com@tett23")),
          ),
          ("tett23:ckusro-core", (Some("ckusro-core"), Some("tett23"))),
          ("ckusro-core", (Some("ckusro-core"), None)),
          ("", (None, None)),
        ];

        for datum in data {
          let (fragment, expected) = datum;
          let actual = PathFragment::split_repository_and_rest(fragment);

          assert_eq!(actual, expected);
        }
      }
    }
  }
}
