use std::io::Read;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = std::io::BufReader::with_capacity(
        64 * 1024,
        std::fs::File::open(
            std::env::args()
                .skip(1)
                .next()
                .expect("missing file path in argument"),
        )?,
    );

    let stdout = std::io::stdout();
    let mut stdout_lock = stdout.lock();
    let mut zeroes: u64 = 0;
    let mut total: u64 = 0;
    let mut t1 = std::time::Instant::now();
    const REFRESH_INTERVAL: std::time::Duration = std::time::Duration::from_secs(1);

    macro_rules! display_progress {
        () => {{
            let _ = write!(
                stdout_lock,
                "\r{}/{} ({:.2}%) ",
                zeroes,
                total,
                zeroes as f64 / total as f64 * 100.0,
            );
            let _ = stdout_lock.flush();
        }};
    }

    for byte in f.bytes() {
        let byte = byte?;

        total += 1;
        if byte == 0 {
            zeroes += 1;
        }

        if t1.elapsed() >= REFRESH_INTERVAL {
            display_progress!();
            t1 = std::time::Instant::now();
        }
    }

    display_progress!();
    let _ = writeln!(stdout_lock);

    Ok(())
}
