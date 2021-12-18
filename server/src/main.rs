#![deny(warnings)]
mod main_hashmap;
mod main_hashmap2;
mod part;
#[cfg(test)]
mod tests;
fn main() {
    let choice = 1;
    println!("CPUS: {}", num_cpus::get());
    println!("CPUS: {}", num_cpus::get_physical());
    match choice {
        0 => {
            main_hashmap::main();
        }
        1 => {
            main_hashmap2::main();
        }
        _ => {}
    }
    //
}
