use git2::Oid;

pub struct RootNode {
  name: String,
  items: Vec<HostNode>,
}

pub struct HostNode {
  name: String,
  items: Vec<UserNode>,
}

pub struct UserNode {
  name: String,
  items: Vec<RepositoryNode>,
}

pub struct RepositoryNode {
  name: String,
  oid: Oid,
}

impl RootNode {
  pub fn find_host(&self, host: String) -> Option<&HostNode> {
    self.items.iter().find(|item| item.name == host)
  }

  pub fn find_user(&self, host: String, user: String) -> Option<&UserNode> {
    let host_item = self.items.iter().find(|item| item.name == host);
    if host_item.is_none() {
      return None;
    }

    host_item.unwrap().find(user)
  }

  pub fn find_repository(
    &self,
    host: String,
    user: String,
    repository: String,
  ) -> Option<&RepositoryNode> {
    let host_item = self.items.iter().find(|item| item.name == host);
    if host_item.is_none() {
      return None;
    }

    let user_item = host_item.unwrap().find(user);
    if user_item.is_none() {
      return None;
    }

    user_item.unwrap().find(repository)
  }

  pub fn push(&mut self, host: HostNode) {
    self.items.push(host)
  }
}

impl HostNode {
  pub fn find(&self, name: String) -> Option<&UserNode> {
    self.items.iter().find(|item| item.name == name)
  }

  pub fn push(&mut self, user: UserNode) {
    self.items.push(user)
  }
}

impl UserNode {
  pub fn find(&self, name: String) -> Option<&RepositoryNode> {
    self.items.iter().find(|item| item.name == name)
  }

  pub fn push(&mut self, repository: RepositoryNode) {
    self.items.push(repository)
  }
}
