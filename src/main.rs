use count_zeroes::CountZeroes;
use std::io::Seek;
use std::io::Write;

const REFRESH_INTERVAL: std::time::Duration = std::time::Duration::from_secs(1);
const BYTE_UNITS: &'static [&'static str] = &["kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

struct DisplayBytes {
    unit: Option<&'static str>,
    value: f64,
}

impl std::fmt::Display for DisplayBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(unit) = self.unit {
            write!(f, "{:.2} {}", self.value, unit)
        } else {
            write!(f, "{:.0} B", self.value)
        }
    }
}

impl DisplayBytes {
    fn new(value: u64) -> Self {
        let value = value as f64;

        if let Some((value, unit)) = BYTE_UNITS
            .iter()
            .enumerate()
            .map(|(i, u)| (value / 1000_f64.powf(i as f64 + 1.0), u))
            .take_while(|(i, _)| *i > 1.0)
            .last()
        {
            Self {
                unit: Some(unit),
                value,
            }
        } else {
            Self { unit: None, value }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut f = std::fs::File::open(
        std::env::args()
            .skip(1)
            .next()
            .expect("missing file path in argument"),
    )?;
    let len = f.seek(std::io::SeekFrom::End(0))?;
    f.seek(std::io::SeekFrom::Start(0))?;
    let stdout = std::io::stdout();
    let mut stdout_lock = stdout.lock();
    let mut t1 = std::time::Instant::now();
    let t2 = std::time::Instant::now();
    let progress_len = DisplayBytes::new(len);

    macro_rules! display_progress {
        ($zeroes:expr, $count:expr) => {{
            let _ = write!(
                stdout_lock,
                "\u{001b}[2K\r{}/{} zeroes: {} ({:.2}%) speed: {}/s",
                DisplayBytes::new($count),
                progress_len,
                DisplayBytes::new($zeroes),
                $zeroes as f64 / $count as f64 * 100.0,
                DisplayBytes::new($count.checked_div(t2.elapsed().as_secs()).unwrap_or(0)),
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
