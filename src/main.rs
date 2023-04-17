mod Spielbrett;
mod Spieler;
fn main() {
    let mut spielbrett = crate::Spielbrett::spiel::new_spielbrett(0);
    spielbrett.starte_spiel();
    let mut spieler_1 = Spieler::Spieler::new_spieler(String::from("Sandro"));
    let mut spieler_2 = Spieler::Spieler::new_spieler(String::from("Pascal"));

    loop {
        spielzug(spielbrett);
        loop {
            if spielbrett.mache_zug(spieler_1.steine_setzen()){
                break
            }
        }
        if(spielbrett.hat_gewonnen()){
            println!("Spieler {} hat gewonnen.", spieler_1.get_name());
            break;
        }

        spielzug(spielbrett);
        loop {
            if spielbrett.mache_zug(spieler_2.steine_setzen()){
                break
            }
        }
        if(spielbrett.hat_gewonnen()){
            println!("Spieler {} hat gewonnen.", spieler_1.get_name());
            break;
        }
    }
}

fn spielzug(spielbrett: crate::Spielbrett::spiel::Spielbrett){
    println!("Steine auf dem Brett");
    println!("{}", spielbrett.get_aktuelle_anzahl_steine())
}




