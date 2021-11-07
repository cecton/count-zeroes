use count_zeroes::CountZeroes;
use std::io::Write;

const REFRESH_INTERVAL: std::time::Duration = std::time::Duration::from_secs(1);
const BYTE_UNITS: &'static [&'static str] = &["kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

macro_rules! format_bytes {
    ($value:expr, $min:expr) => {
        BYTE_UNITS
            .iter()
            .enumerate()
            .map(|(i, u)| ($value as f64 / 1000_f64.powf(i as f64 + 1.0), u))
            .take_while(|(i, _)| *i > $min as f64)
            .map(|(i, u)| format!("{:.2} {}", i, u))
            .last()
            .unwrap_or(format!("{} B", $value))
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut f = std::fs::File::open(
        std::env::args()
            .skip(1)
            .next()
            .expect("missing file path in argument"),
    )?;
    let stdout = std::io::stdout();
    let mut stdout_lock = stdout.lock();
    let mut t1 = std::time::Instant::now();
    let t2 = std::time::Instant::now();

    macro_rules! display_progress {
        ($zeroes:expr, $count:expr) => {{
            let _ = write!(
                stdout_lock,
                "\u{001b}[2K\r{}/{} ({:.2}%) {}/s",
                format_bytes!($zeroes, 1000),
                format_bytes!($count, 1000),
                $zeroes as f64 / $count as f64 * 100.0,
                format_bytes!($count.checked_div(t2.elapsed().as_secs()).unwrap_or(0), 1),
            );
            let _ = stdout_lock.flush();
        }};
    }

    let (zeroes, count) = f.count_zeroes(|zeroes: u64, count: u64| {
        if t1.elapsed() >= REFRESH_INTERVAL {
            display_progress!(zeroes, count);
            t1 = std::time::Instant::now();
        }
    })?;

    display_progress!(zeroes, count);
    let _ = writeln!(stdout_lock);

    Ok(())
}
