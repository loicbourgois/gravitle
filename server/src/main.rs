#![deny(warnings)]
//mod main_hashmap;
//mod server_1;
mod server_2;
mod part;
mod maths3d;
//#[cfg(test)]
//mod tests;
fn main() {
    let choice = 2;
    println!("CPUS: {}", num_cpus::get());
    println!("CPUS: {}", num_cpus::get_physical());
    match choice {
        // 0 => {
        //     main_hashmap::main();
        // }
        // 1 => {
        //     server_1::server::main();
        // }
        2 => {
            server_2::server::main();
        }
        _ => {}
    }
}
