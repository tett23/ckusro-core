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

  pub fn parse_fragment(fragment: &str) -> Result<PathFragment, Error> {
    let (domain, user, repository) = PathFragment::split_fragment_string(fragment)?;

    Ok(PathFragment {
      domain: domain.to_owned(),
      user: user.to_owned(),
      repository: repository.to_owned(),
    })
  }

  fn split_fragment_string(fragment: &str) -> Result<(&str, &str, &str), Error> {
    if !PathFragment::is_valid_format(fragment) {
      return Err(Error::MalformedFragment(fragment.to_owned()));
    }

    let (domain, rest) = PathFragment::split_domain_and_rest(fragment)?;
    let (user, repository) = PathFragment::split_user_and_repository(rest)?;

    Ok((domain, user, repository))
  }

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

  fn split_domain_and_rest(fragment: &str) -> Result<(&str, &str), Error> {
    let vec: Vec<&str> = fragment.splitn(2, PathFragment::USER_SEPARATOR).collect();

    match vec.len() {
      2 => Ok(2),
      _ => Err(Error::MalformedDomainFragment(fragment.to_owned())),
    }?;

    let domain = match vec.first() {
      Some(&"") => Err(Error::MalformedDomainFragment(fragment.to_owned())),
      Some(v) => Ok(v),
      None => Err(Error::MalformedDomainFragment(fragment.to_owned())),
    }?;

    let rest = match vec.last() {
      Some(&"") => Err(Error::MalformedDomainFragment(fragment.to_owned())),
      Some(v) => Ok(v),
      None => Err(Error::MalformedDomainFragment(fragment.to_owned())),
    }?;

    Ok((domain, rest))
  }

  fn split_user_and_repository(fragment: &str) -> Result<(&str, &str), Error> {
    let vec: Vec<&str> = fragment
      .splitn(2, PathFragment::REPOSITORY_SEPARATOR)
      .collect();

    match vec.len() {
      2 => Ok(2),
      _ => Err(Error::MalformedDomainFragment(fragment.to_owned())),
    }?;

    let user = match vec.first() {
      Some(&"") => Err(Error::MalformedDomainFragment(fragment.to_owned())),
      Some(v) => Ok(v),
      None => Err(Error::MalformedDomainFragment(fragment.to_owned())),
    }?;

    let repository = match vec.last() {
      Some(&"") => Err(Error::MalformedDomainFragment(fragment.to_owned())),
      Some(v) => Ok(v),
      None => Err(Error::MalformedDomainFragment(fragment.to_owned())),
    }?;

    Ok((user, repository))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod path_fragment {
    use super::*;

    mod parse_fragment {
      use super::*;

      #[test]
      fn it_works() {
        let fragment = "github.com@tett23:ckusro-core";
        let actual = PathFragment::parse_fragment(fragment);
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
          let actual = PathFragment::parse_fragment(datum);

          assert!(actual.is_err());
        }
      }
    }

    mod split_fragment_string {
      use super::*;

      #[test]
      fn it_works() {
        let fragment = "github.com@tett23:ckusro-core";
        let actual = PathFragment::split_fragment_string(fragment);
        let expected = Ok(("github.com", "tett23", "ckusro-core"));

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
          let actual = PathFragment::split_fragment_string(datum);

          assert!(actual.is_err());
        }
      }
    }

    mod is_valid_format {
      use super::*;

      #[test]
      fn it_works() {
        let fragment = "github.com@tett23:ckusro-core";
        let actual = PathFragment::is_valid_format(fragment);

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

    mod split_domain_and_rest {
      use super::*;

      #[test]
      fn it_works() {
        let fragment = "github.com@tett23:ckusro-core";
        let actual = PathFragment::split_domain_and_rest(fragment);
        let expected = Ok(("github.com", "tett23:ckusro-core"));

        assert_eq!(actual, expected);
      }

      #[test]
      fn it_does_not_works() {
        let data = vec!["", "github.com", "tett23:ckusro-core"];
        for datum in data {
          let actual = PathFragment::split_domain_and_rest(datum);

          assert!(actual.is_err());
        }
      }
    }

    mod split_user_and_repository {
      use super::*;

      #[test]
      fn it_works() {
        let fragment = "tett23:ckusro-core";
        let actual = PathFragment::split_user_and_repository(fragment);
        let expected = Ok(("tett23", "ckusro-core"));

        assert_eq!(actual, expected);
      }

      #[test]
      fn it_does_not_works() {
        let data = vec!["", "tett23"];
        for datum in data {
          let actual = PathFragment::split_domain_and_rest(datum);

          assert!(actual.is_err());
        }
      }
    }
  }
}
