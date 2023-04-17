use std::io;

pub mod Spieler{
    use std::io;

    pub struct spieler{name: String}

    pub fn new_spieler(name: String) -> spieler {
        spieler{name}
    }

    impl spieler{
        pub fn get_name(&mut self) -> &str {
            &self.name
        }

        pub fn steine_setzen(&mut self) -> i32{
            println!("Am zug: Spieler {}", self.get_name());
            println!("Ihre Wahl (1-10): ");
            let mut eingabe = String::new();
            io::stdin().read_line(&mut eingabe).expect("Error");
            loop {
                let eingabe_num: i32 = match eingabe.trim().parse::<i32>() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                return eingabe_num;
            }


        }
    }
}