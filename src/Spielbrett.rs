mod spiel{
    pub struct Spielbrett{
        aktuelle_anzahl_steine: i32
    }

    pub fn new_spielbrett() -> Spielbrett {
        Spielbrett
}

    impl Spielbrett{
        pub fn mache_zug(&mut self, steine: i32 ) -> bool{
            if steine >= 1 && steine <= 10 {
                self.aktuelle_anzahl_steine += steine;
                true
            }
            false
        }

        pub fn starte_spiel(&mut self) {
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