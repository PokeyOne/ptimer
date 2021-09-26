use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};
use std::env;
use std::io;
use std::io::Write;
use std::sync::mpsc;
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};

/// The return result for the process_args function. This contains the actual
/// time result, plus whether or not other arguments have been passed.
struct ProcessedArgs {
    time: HmsTime,
    stopped_timer: bool,
    play_sound: bool
}

/// A time construct for storing hours, minutes, and seconds as raw integers.
/// This results in less of a time, but more accurately an inverval.
///
/// The struct consists of just three elements: h, m, s. These refer to
/// hours, minutes, and seconds respectively.
struct HmsTime {
    h: u64,
    m: u64,
    s: u64
}

impl HmsTime {
    /// Create a new HmsTime with the given amount of seconds. These seconds
    /// will be rebalanced into the appropriate amount of hours, minutes,
    /// and seconds.
    fn from_seconds(seconds: u64) -> HmsTime {
        let result = HmsTime {
            h: 0,
            m: 0,
            s: seconds
        };

        result.rebalanced()
    }

    /// Using the stored hours, minutes, and seconds, calculates the total
    /// seconds that this HmsTime represents
    fn total_seconds(&self) -> u64 {
        self.s + (self.m + self.h * 60) * 60
    }

    /// Rebalance all the hours, minutes, and seconds values so that both
    /// seconds and minutes are less than or equal to 60. Does not check to
    /// to see if rebalancing is needed.
    fn rebalanced(&self) -> HmsTime {
        let total_seconds = self.total_seconds();
        let wrapped_seconds = total_seconds % 60;
        let total_minutes = (total_seconds - wrapped_seconds) / 60;
        let wrapped_minutes = total_minutes % 60;
        let total_hours = (total_minutes - wrapped_minutes) / 60;

        HmsTime {
            h: total_hours,
            m: wrapped_minutes,
            s: wrapped_seconds
        }
    }

    /// Format the time to something similar to "02:02:02"
    fn fmt(&self) -> String {
        format!("{:02}:{:02}:{:02}", self.h, self.m, self.s)
    }

    /// Play a small sound when the timer completes
    fn play_completion_sound(&self) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        for _ in [0, 1] {
            let source = SineWave::new(440) // A
                .take_duration(Duration::from_secs_f32(1.0))
                .amplify(0.20);
            sink.append(source);

            let source = SineWave::new(330) // E
                .take_duration(Duration::from_secs_f32(0.2))
                .amplify(0.20);
            sink.append(source);

            let source = SineWave::new(440) // A
                .take_duration(Duration::from_secs_f32(0.5))
                .amplify(0.20);
            sink.append(source);

            let source = SineWave::new(0) // break
                .take_duration(Duration::from_secs_f32(1.0))
                .amplify(0.20);
            sink.append(source);
        }

        sink.sleep_until_end();
    }
}

/// Process the arguments given through the command line
///
/// # Errors
/// The program will error if invalid argument syntax
fn process_args(args: Vec<String>) -> Result<ProcessedArgs, &'static str> {
    if args.len() < 2 {
        return Err("Must provide at least one argument");
    }

    let mut hours: u64 = 0;
    let mut minutes: u64 = 0;
    let mut seconds: u64 = 0;

    let mut play_sound = true;
    let mut stopped_timer = true;

    // Process all the arguments and add hours minutes and seconds as going
    // along. This is not implemented as a for loop because there are calls
    // to increment i that are not just the end of the loop.
    let mut i = 1;
    while i < args.len() {
        if args[i].eq("-h") {
            if i < args.len() - 1 {
                hours += args[i + 1]
                    .parse::<u64>()
                    .expect("Argument after -h must be a number");
                i += 1;
            } else {
                return Err("-h must be followed by another argument");
            }
        } else if args[i].eq("-m") {
            if i < args.len() - 1 {
                minutes += args[i + 1]
                    .parse::<u64>()
                    .expect("Argument after -m must be a number");
                i += 1;
            } else {
                return Err("-m must be followed by another argument");
            }
        } else if args[i].eq("-s") {
            if i < args.len() - 1 {
                seconds += args[i + 1]
                    .parse::<u64>()
                    .expect("Argument after -s must be a number");
                i += 1;
            } else {
                return Err("-s must be followed by another argument");
            }
        } else if args[i].eq("--silent") {
            play_sound = false;
        } else if args[i].eq("--exit-on-stop") {
            stopped_timer = false;
        } else {
            seconds += args[i]
                .parse::<u64>()
                .expect("Lone argument must be integer or flag");
        }

        i += 1;
    }

    let hms_time: HmsTime = HmsTime {
        h: hours,
        m: minutes,
        s: seconds
    };

    Ok(ProcessedArgs {
        time: hms_time.rebalanced(),
        stopped_timer: stopped_timer,
        play_sound: play_sound
    })
}

/// Given an HmsTime, this function will freeze the program and terminal for
/// the specified length of time. It will keep the time in the terminal updated
/// live as it runs by using the special return character.
fn run_timer_for(tv: &HmsTime) {
    let totes: u64 = tv.total_seconds();
    let init_time = Instant::now();

    while init_time.elapsed().as_secs() < totes {
        sleep(Duration::from_millis(500));

        let hms = HmsTime::from_seconds(totes - init_time.elapsed().as_secs());
        print!("\r{}", hms.fmt());
        io::stdout().flush().unwrap();
    }
    println!("");

    println!("done");
}

/// Shows a timer counting up to the user. This is intended to be the time
/// since the timer has ended. This state can be exitted by typing q and enter
/// or just entering a blank line.
fn show_completed_timer() {
    let wait_start_time = Instant::now();

    let (tx, rx) = mpsc::channel();

    // Here we spawn another thread that will essentially just monitor for
    // user input of commands to the program. Currently it just monitors for
    // the q character or nothing.
    // We don't assign a value because we never need to join with this thread.
    let _ = thread::spawn(move || {
        loop {
            // Block until can read a line
            let mut line = String::new();
            io::stdin()
                .read_line(&mut line)
                .expect("Could not read line");

            // Trim the newline and any spaces
            line = line.trim().to_string();

            // Look for quit command
            if line.eq("q") || line.eq("") {
                // Ignore errors because if can't send then we are closing this
                // thread regardless and the error is almost guaranteed to be
                // rx not existing. The user can also almost always just C-c
                match tx.send(true) {
                    Ok(_) => {}
                    Err(_) => {}
                }
                // This will kill this loop
                break;
            }
        }
    });

    println!(
        "Your time has completed. Below is the time since completion. \
              Enter one of the following commands."
    );
    println!("  (q) - Quit (default)");
    loop {
        // Sleep so we aren't updating the screen at unnecessary intervals
        sleep(Duration::from_millis(500));

        // If the spawned thread sends a message, then we should stop the loop
        let should_close: bool = match rx.try_recv() {
            Ok(_) => true,
            Err(_) => false
        };
        if should_close {
            break;
        }

        // Print the time on the same line
        let hms = HmsTime::from_seconds(wait_start_time.elapsed().as_secs());
        print!("\r{}  ", hms.fmt());
        io::stdout().flush().unwrap();
    }
    println!("");
}

fn main() {
    println!("Please standby while we process your request.");
    println!("An agent will be with you shortly.");

    // NOTE: This is its own scope to make it clear that args is invalid after
    //       ownership is transferred to process_args
    let (time_value, show_stopped_timer, play_sound) = {
        let args: Vec<String> = env::args().collect();

        let proargs: ProcessedArgs = match process_args(args) {
            Ok(val) => val,
            Err(msg) => {
                println!("{}", msg);
                panic!();
            }
        };

        (proargs.time, proargs.stopped_timer, proargs.play_sound)
    };

    println!(
        "Starting timer for {}h {}m {}s for a total of {}s",
        time_value.h,
        time_value.m,
        time_value.s,
        time_value.total_seconds()
    );

    run_timer_for(&time_value);

    if play_sound {
        time_value.play_completion_sound();
    }

    if show_stopped_timer {
        show_completed_timer();
    }
}
