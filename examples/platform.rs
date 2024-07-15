use monolith::platform::unix::Clock;

fn main() {
    let clock = Clock::monotonic();

    println!("Resolution: {}", clock.resolution());

    loop {
        clock.update();

        println!("ticks: {}", clock.ticks());
        println!("nanoseconds: {}", clock.nanoseconds());
        println!("microseconds: {}", clock.microseconds());
        println!("milliseconds: {}", clock.milliseconds());
        println!("seconds: {}", clock.seconds());

        std::thread::sleep(std::time::Duration::from_millis(600));
    }
}
