use std::io::BufRead;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut f = std::io::BufReader::with_capacity(
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
    let t2 = std::time::Instant::now();
    const REFRESH_INTERVAL: std::time::Duration = std::time::Duration::from_secs(1);

    macro_rules! display_progress {
        () => {{
            let _ = write!(
                stdout_lock,
                "\r{}/{} ({:.2}%) {}B/s ",
                zeroes,
                total,
                zeroes as f64 / total as f64 * 100.0,
                total.checked_div(t2.elapsed().as_secs()).unwrap_or(0),
            );
            let _ = stdout_lock.flush();
        }};
    }

    while let Ok(buffer) = f.fill_buf() {
        let len = buffer.len();

        if len == 0 {
            break;
        }

        let (prefix, aligned, suffix) = unsafe { buffer.align_to::<u128>() };

        total += len as u64;
        if prefix.iter().all(|&x| x == 0)
            && suffix.iter().all(|&x| x == 0)
            && aligned.iter().all(|&x| x == 0)
        {
            zeroes += len as u64;
        }

        if t1.elapsed() >= REFRESH_INTERVAL {
            display_progress!();
            t1 = std::time::Instant::now();
        }

        f.consume(len);
    }

    display_progress!();
    let _ = writeln!(stdout_lock);

    Ok(())
}
