
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::str;
use std::mem;
use std::io::{self, Write};


// a cla`ss for rendering things to the terminal in a background thread
pub struct BackgroundTerminal {
    foreground: Vec<u8>,
    shared: Arc<Mutex<(bool, Vec<u8>)>>,
    handle: Option<thread::JoinHandle<()>>,
}

impl BackgroundTerminal {
    pub fn new(
        limit: Option<usize>,
        sleep: Option<Duration>,
    ) -> Self {
        // assume 10ms sleep interval by default
        let sleep = sleep.unwrap_or_else(|| Duration::from_millis(10));

        let shared = Arc::new(Mutex::new((false, vec![])));
        let handle = thread::spawn({
            let shared = Arc::clone(&shared);
            move || {
                let mut background = vec![];
                let mut lines = 1;
                loop {
                    // swap background/shared buffers if updated
                    let done = {
                        let mut shared = shared.lock().unwrap();
                        if shared.1.len() > 0 {
                            background.clear();
                            mem::swap(&mut shared.1, &mut background);
                        }
                        shared.0
                    };

                    // how many lines do we want to show
                    let lines_ = str::from_utf8(&background).unwrap()
                        .lines()
                        .count();

                    // optionally limit the number of lines we print
                    let (skip, lines_) = match limit {
                        Some(limit) if limit < lines_ => {
                            (lines_ - limit, limit)
                        }
                        _ => {
                            (0, lines_)
                        }
                    };

                    // give ourselves a canvas
                    //
                    // NOTE in theory we should limit how many newlines we
                    // print by the terminal size, but that's more complicate
                    // than I'm interested in right now
                    while lines_ > lines {
                        println!();
                        lines += 1;
                    }

                    for (i, line) in
                        str::from_utf8(&background).unwrap()
                            .lines()
                            .skip(skip)
                            .enumerate()
                    {
                        // reset cursor, note we move from the bottom to
                        // let the terminal limit our cursor movement
                        print!("\r");
                        if lines-1-i > 0 {
                            // \x1b[nA => move cursor up n lines
                            print!("\x1b[{}A", lines-1-i);
                        }
                        // \x1b[K   => clear line
                        // \x1b[?7l => disable linewrap
                        // \x1b[?7h => enable linewrap
                        print!("\x1b[K\x1b[?7l{}\x1b[?7h", line);
                        if lines-1-i > 0 {
                            // \x1b[nB => move cursor down n lines
                            print!("\x1b[{}B", lines-1-i);
                        }
                    }

                    // flush output
                    io::stdout().flush().unwrap();

                    if done {
                        break;
                    }

                    thread::sleep(sleep);
                }
            }
        });

        Self{
            shared: shared,
            foreground: vec![],
            handle: Some(handle),
        }
    }

    // swap foreground/shared buffers, updating the terminal and preparing
    // for a new write
    pub fn swap(&mut self) {
        mem::swap(&mut self.shared.lock().unwrap().1, &mut self.foreground);
        self.foreground.clear();
    }
}

// allow writing to terminal
impl Write for BackgroundTerminal {
    fn write(&mut self, data: &[u8]) -> Result<usize, io::Error> {
        self.foreground.write(data)
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        self.foreground.flush()
    }
}


// cleanup background thread when dropped
impl Drop for BackgroundTerminal {
    fn drop(&mut self) {
        self.shared.lock().unwrap().0 = true;
        self.handle.take().unwrap().join().unwrap();
        // sneaky newline
        println!();
    }
}
