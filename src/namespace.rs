#[derive(PartialEq, Debug)]
pub struct Namespace {
  name: String,
  namespace_type: NamespaceType,
}

impl Namespace {
  pub fn new(namespace_type: NamespaceType, name: &str) -> Namespace {
    Namespace {
      name: name.to_owned(),
      namespace_type,
    }
  }
}

#[derive(PartialEq, Debug)]
pub enum NamespaceType {
  Domain,
  User,
  Repository,
}

impl ToString for NamespaceType {
  fn to_string(&self) -> String {
    match self {
      NamespaceType::Domain => "NamespaceType::Domain".to_owned(),
      NamespaceType::User => "NamespaceType::User".to_owned(),
      NamespaceType::Repository => "NamespaceType::Repository".to_owned(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_namespace_new_domain() {
    let actual = Namespace::new(NamespaceType::Domain, "github.com");
    let expected = Namespace {
      name: "github.com".to_owned(),
      namespace_type: NamespaceType::Domain,
    };

    assert_eq!(actual, expected);
  }

  #[test]
  fn test_namespace_new_user() {
    let actual = Namespace::new(NamespaceType::User, "tett23");
    let expected = Namespace {
      name: "tett23".to_owned(),
      namespace_type: NamespaceType::User,
    };

    assert_eq!(actual, expected);
  }

  #[test]
  fn test_namespace_new_repository() {
    let actual = Namespace::new(NamespaceType::Repository, "ckusro-core");
    let expected = Namespace {
      name: "ckusro-core".to_owned(),
      namespace_type: NamespaceType::Repository,
    };

    assert_eq!(actual, expected);
  }
}
