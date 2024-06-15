pub enum AccessLevel {
    Guest, 
    Normal,
    Admin,
}

pub struct User {
    name:String,
    acessLevel: AccessLevel,
}

impl User {
  pub fn new(name: String, level: AccessLevel) -> User {
    User{
        name,
        acessLevel:level,
    }
  }
  pub fn send_name(&self) -> Option<&str> {
    match self.acessLevel {
        AccessLevel::Guest => None,
        _=>Some(&self.name)
    }
  }
}

pub fn check_user_name(user: &User) -> (bool, &str) {
    match user.send_name(){
        None => (false,"ERROR: User is guest" ),
        _ => (true, &user.name),
    }
}
