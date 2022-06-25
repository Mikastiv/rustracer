use rustracer::run;

fn main() {
    pollster::block_on(run());
}
