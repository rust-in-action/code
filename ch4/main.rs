mod ch4_check_sats_1;
mod ch4_check_sats_2;
mod ch4_check_sats_3;
mod ch4_check_sats_clone_and_copy_traits;
mod ch4_check_sats_with_copy_trait;
mod ch4_rc_groundstation;
mod ch4_rc_refcell_groundstation;
mod ch4_sat_mailbox;
mod ch4_short_lived_strategy;

pub fn main() {
    println!("-- ch4_check_sats_1 --");
    ch4_check_sats_1::main();

    println!("-- ch4_check_sats_2 --");
    ch4_check_sats_2::main();

    println!("-- ch4_check_sats_3 --");
    ch4_check_sats_3::main();

    println!("-- ch4_check_sats_clone_and_copy_traits --");
    ch4_check_sats_clone_and_copy_traits::main();

    println!("-- ch4_check_sats_with_copy_trait --");
    ch4_check_sats_with_copy_trait::main();

    println!("-- ch4_rc_groundstation --");
    ch4_rc_groundstation::main();

    println!("-- ch4_rc_refcell_groundstation --");
    ch4_rc_refcell_groundstation::main();

    println!("-- ch4_sat_mailbox --");
    ch4_sat_mailbox::main();

    println!("-- ch4_short_lived_strategy --");
    ch4_short_lived_strategy::main();
}
