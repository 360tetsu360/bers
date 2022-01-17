use bers::motd::Motd;

#[test]
fn motd() {
    let motd_str = "MCPE;§b§lEU §7§l» WINTERFEST \u{e101};121;1.0;20566;100001;1821793688326766702;Hive Games;Survival";
    let motd = Motd::from(motd_str);
    let encoded = motd.to_string();
    assert_eq!(encoded, motd_str.to_owned());
}
