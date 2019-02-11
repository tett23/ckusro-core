use super::error::Error;

pub struct PathFragment {
  domain: String,
  user: String,
  repository: String,
}

impl PathFragment {
  const USER_SEPARATOR: char = '@';
  const REPOSITORY_SEPARATOR: char = ':';

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
