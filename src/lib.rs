use std::io::BufRead;
use std::io::Read;

const DEFAULT_BUFFER_SIZE: usize = 64 * 1024;

pub trait CountZeroes<P> {
    fn count_zeroes(&mut self, progress: P) -> Result<(u64, u64), std::io::Error>;
}

impl<R: Read, P: FnMut(u64, u64)> CountZeroes<P> for std::io::BufReader<R> {
    fn count_zeroes(&mut self, mut progress: P) -> Result<(u64, u64), std::io::Error> {
        let mut zeroes: u64 = 0;
        let mut count: u64 = 0;

        while let Ok(buffer) = self.fill_buf() {
            let len = buffer.len();

            if len == 0 {
                break;
            }

            count += len as u64;
            zeroes += buffer.iter().filter(|&&x| x == 0).count() as u64;

            progress(zeroes, count);

            self.consume(len);
        }

        Ok((zeroes, count))
    }
}

impl<P: FnMut(u64, u64)> CountZeroes<P> for std::fs::File {
    fn count_zeroes(&mut self, progress: P) -> Result<(u64, u64), std::io::Error> {
        let mut reader = std::io::BufReader::with_capacity(DEFAULT_BUFFER_SIZE, self);

        reader.count_zeroes(progress)
    }
}

impl<R: Read> CountZeroes<()> for std::io::BufReader<R> {
    fn count_zeroes(&mut self, _progress: ()) -> Result<(u64, u64), std::io::Error> {
        self.count_zeroes(|_zeroes: u64, _count: u64| ())
    }
}

impl CountZeroes<()> for std::fs::File {
    fn count_zeroes(&mut self, progress: ()) -> Result<(u64, u64), std::io::Error> {
        let mut reader = std::io::BufReader::with_capacity(DEFAULT_BUFFER_SIZE, self);

        reader.count_zeroes(progress)
    }
}
