use super::ToCsv;

#[derive(Debug)]
pub struct User {
    name: String,
    surname: String,
    username: String,
    phone_number: u32,
}

impl User {
    pub fn build(name: String, surname: String, username: String, phone_number: u32) -> User {
        User { name, surname, username, phone_number }
    }

    pub fn change(&mut self, name: String) { self.name = name; }
}

impl ToCsv for User {
    fn to_csv(&self) -> String { format!("{}|{}|{}|{}", &self.name, &self.surname, &self.username, &self.phone_number) }

    fn from_csv(line: &str) -> Option<User> {
        let mut v_vec = line.split("|");

        let name = v_vec.next()?.to_string();
        let surname = v_vec.next()?.to_string();
        let username = v_vec.next()?.to_string();
        let phone_number: u32 = match v_vec.next()?.parse() {
            Ok(num) => num,
            Err(_e) => {
                eprintln!("phone number can't be parsed, using default 0");
                0
            }
        };

        Some(User { name, surname, username, phone_number })
    }

    fn main_key(&self) -> String { self.username.to_string() }
}