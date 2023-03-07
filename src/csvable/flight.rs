use super::ToCsv;

#[derive(Debug)]
pub struct Flight {
    starting_point: String,
    ending_point: String,
    flight_number: String,
}

impl Flight {
    pub fn build(starting_point: String, ending_point: String, flight_number: String, ) -> Flight {
        Flight { starting_point, ending_point, flight_number }
    }
}

impl ToCsv for Flight {
    fn to_csv(&self) -> String { format!("{}|{}|{}", &self.starting_point, &self.ending_point, &self.flight_number) }

    fn from_csv(line: &str) -> Option<Self> where Self: Sized {
        let mut v_vec = line.split("|");

        let starting_point = v_vec.next()?.to_string();
        let ending_point = v_vec.next()?.to_string();
        let flight_number = v_vec.next()?.to_string();

        Some(Flight { starting_point, ending_point, flight_number })
    }

    fn main_key(&self) -> String { self.flight_number.to_string() }
}