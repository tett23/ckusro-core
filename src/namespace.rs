use super::error::Error;

trait GetRaw<'a> {
  fn raw(&self) -> &'a Namespace;
}

struct DomainNamespace<'a> {
  namespace: &'a Namespace,
}

impl<'a> DomainNamespace<'a> {
  pub fn new(namespace: &'a Namespace) -> Result<DomainNamespace, Error> {
    match &namespace.namespace_type {
      NamespaceType::Domain => Ok(DomainNamespace {
        namespace: namespace,
      }),
      ns => Err(Error::NamespaceMismatch {
        t1: "DomainNamespace".to_owned(),
        t2: ns.to_string(),
      }),
    }
  }
}

impl<'a> GetRaw<'a> for DomainNamespace<'a> {
  fn raw(&self) -> &'a Namespace {
    self.namespace
  }
}

struct UserNamespace<'a> {
  namespace: &'a Namespace,
}

impl<'a> UserNamespace<'a> {
  pub fn new(namespace: &'a Namespace) -> Result<UserNamespace, Error> {
    match &namespace.namespace_type {
      NamespaceType::User => Ok(UserNamespace {
        namespace: namespace,
      }),
      ns => Err(Error::NamespaceMismatch {
        t1: "UserNamespace".to_owned(),
        t2: ns.to_string(),
      }),
    }
  }
}

impl<'a> GetRaw<'a> for UserNamespace<'a> {
  fn raw(&self) -> &'a Namespace {
    self.namespace
  }
}

struct RepositoryNamespace<'a> {
  namespace: &'a Namespace,
}

impl<'a> RepositoryNamespace<'a> {
  pub fn new(namespace: &'a Namespace) -> Result<RepositoryNamespace, Error> {
    match &namespace.namespace_type {
      NamespaceType::Repository => Ok(RepositoryNamespace {
        namespace: namespace,
      }),
      ns => Err(Error::NamespaceMismatch {
        t1: "RepositoryNamespace".to_owned(),
        t2: ns.to_string(),
      }),
    }
  }
}

impl<'a> GetRaw<'a> for RepositoryNamespace<'a> {
  fn raw(&self) -> &'a Namespace {
    self.namespace
  }
}

#[derive(PartialEq, Debug)]
pub struct Namespace {
  name: String,
  pub namespace_type: NamespaceType,
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

impl Copy for NamespaceType {}

impl Clone for NamespaceType {
  fn clone(&self) -> Self {
    *self
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod namespace {
    use super::*;

    #[test]
    fn test_new_domain() {
      let actual = Namespace::new(NamespaceType::Domain, "github.com");
      let expected = Namespace {
        name: "github.com".to_owned(),
        namespace_type: NamespaceType::Domain,
      };

      assert_eq!(actual, expected);
    }

    #[test]
    fn test_new_user() {
      let actual = Namespace::new(NamespaceType::User, "tett23");
      let expected = Namespace {
        name: "tett23".to_owned(),
        namespace_type: NamespaceType::User,
      };

      assert_eq!(actual, expected);
    }

    #[test]
    fn test_new_repository() {
      let actual = Namespace::new(NamespaceType::Repository, "ckusro-core");
      let expected = Namespace {
        name: "ckusro-core".to_owned(),
        namespace_type: NamespaceType::Repository,
      };

      assert_eq!(actual, expected);
    }
  }

  mod domain_namespace {
    use super::*;

    #[test]
    fn test_new() {
      let namespace = Namespace::new(NamespaceType::Domain, "github.com");
      let actual = DomainNamespace::new(&namespace);

      assert!(actual.is_ok());
    }

    #[test]
    fn test_new_when_passed_invalid_namespace_type() {
      let namespace = Namespace::new(NamespaceType::User, "tett23");
      let actual = DomainNamespace::new(&namespace);

      assert!(actual.is_err());
    }

    #[test]
    fn test_raw() {
      let namespace = Namespace::new(NamespaceType::Domain, "github.com");
      let domain = DomainNamespace {
        namespace: &namespace,
      };
      let actual = domain.raw();
      let expected = &namespace;

      assert_eq!(actual, expected);
    }
  }

  mod user_namespace {
    use super::*;

    #[test]
    fn test_new() {
      let namespace = Namespace::new(NamespaceType::User, "tett23");
      let actual = UserNamespace::new(&namespace);

      assert!(actual.is_ok());
    }

    #[test]
    fn test_new_when_passed_invalid_namespace_type() {
      let namespace = Namespace::new(NamespaceType::Domain, "github.com");
      let actual = UserNamespace::new(&namespace);

      assert!(actual.is_err());
    }

    #[test]
    fn test_raw() {
      let namespace = Namespace::new(NamespaceType::User, "tett23");
      let user = UserNamespace {
        namespace: &namespace,
      };
      let actual = user.raw();
      let expected = &namespace;

      assert_eq!(actual, expected);
    }
  }

  mod repository_namespace {
    use super::*;

    #[test]
    fn test_new() {
      let namespace = Namespace::new(NamespaceType::Repository, "ckusro-core");
      let actual = RepositoryNamespace::new(&namespace);

      assert!(actual.is_ok());
    }

    #[test]
    fn test_new_when_passed_invalid_namespace_type() {
      let namespace = Namespace::new(NamespaceType::User, "tett23");
      let actual = RepositoryNamespace::new(&namespace);

      assert!(actual.is_err());
    }

    #[test]
    fn test_raw() {
      let namespace = Namespace::new(NamespaceType::Repository, "ckusro-core");
      let repository = RepositoryNamespace {
        namespace: &namespace,
      };
      let actual = repository.raw();
      let expected = &namespace;

      assert_eq!(actual, expected);
    }
  }
}
