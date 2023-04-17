
pub mod spiel{
    use rand::prelude::*;

    #[derive(Clone, Copy)]
    pub struct Spielbrett{
        aktuelle_anzahl_steine: i32
    }

    pub fn new_spielbrett(aktuelle_anzahl_steine: i32) -> Spielbrett {
        Spielbrett{aktuelle_anzahl_steine}
    }

    impl Spielbrett{
        pub fn mache_zug(&mut self, steine: i32 ) -> bool{
            if steine >= 1 && steine <= 10 {
                self.aktuelle_anzahl_steine += steine;
                return true;
            }

            if self.aktuelle_anzahl_steine + steine == 100{
                return false;
            }

            println!("Fehlerhafte Eingabe");
            false
        }

        pub fn starte_spiel(&mut self) {
            let mut rng = thread_rng();
            self.aktuelle_anzahl_steine = rng.gen_range(0..30);
        }

        pub fn get_aktuelle_anzahl_steine(&self) -> i32{
            self.aktuelle_anzahl_steine
        }

        pub fn hat_gewonnen(&self) -> bool{
            self.aktuelle_anzahl_steine == 100
        }
    }
}