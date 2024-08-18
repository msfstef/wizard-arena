use wizard_arena::run;

fn main() {
    println!("Hello, world!");
    pollster::block_on(run());
}
