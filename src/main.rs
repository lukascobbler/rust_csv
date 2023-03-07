mod csvable;
mod csv_loader;

use std::collections::HashMap;
use csvable::user::User;
use csvable::flight::Flight;
use csvable::ToCsv;

use csv_loader::CsvLoader;

fn main() {
	let user_handler: CsvLoader<User> = CsvLoader::new("./users.csv");
	let flight_handler: CsvLoader<Flight> = CsvLoader::new("./flights.csv");

	let mut user_database = HashMap::<String, User>::new();
	let mut flight_database = HashMap::<String, Flight>::new();

	let user1 = User::build("Me".to_string(), "I".to_string(), "mei".to_string(), 1234);
	let user2 = User::build("You".to_string(), "Y".to_string(), "yoy".to_string(), 4567);
	let user3 = User::build("We".to_string(), "W".to_string(), "wew".to_string(), 7891);

	let flight1 = Flight::build("NYC".to_string(), "BRK".to_string(), "TE12".to_string());
	let flight2 = Flight::build("BRK".to_string(), "NYC".to_string(), "TE22".to_string());

	user_database.insert(user1.main_key(), user1);
	user_database.insert(user2.main_key(), user2);
	user_database.insert(user3.main_key(), user3);

	user_database.get_mut("yoy").unwrap().change("Yours".to_string());

	flight_database.insert(flight1.main_key(), flight1);
	flight_database.insert(flight2.main_key(), flight2);

	user_handler.save(&user_database);
	flight_handler.save(&flight_database);

	println!("{:?}", user_handler.load());
	println!("{:?}", flight_handler.load());
}
