use std::env;
use std::time::{Duration, Instant};
use std::thread::sleep;
use std::io;
use std::io::Write;

// TODO: use this type in places
struct HmsTime {
    h: u64,
    m: u64,
    s: u64
}
impl HmsTime {
    fn from_seconds(seconds: u64) -> HmsTime {
        let result = HmsTime {h: 0, m: 0, s: seconds};

        result.rebalanced()
    }

    fn total_seconds(&self) -> u64 {
        self.s + (self.m + self.h * 60) * 60
    }

    fn rebalanced(&self) -> HmsTime {
        let total_seconds = self.total_seconds();
        let wrapped_seconds = total_seconds % 60;
        let total_minutes = (total_seconds - wrapped_seconds) / 60;
        let wrapped_minutes = total_minutes % 60;
        let total_hours = (total_minutes - wrapped_minutes) / 60;

        HmsTime {h: total_hours, m: wrapped_minutes, s: wrapped_seconds}
    }

    fn fmt(&self) -> String {
        format!("{:02}:{:02}:{:02}", self.h, self.m, self.s)
    }
}

/// Process the arguments given through the command line
///
/// # Errors
/// The program will error if invalid argument syntax
fn process_args(args: Vec<String>) -> Result<HmsTime, &'static str> {
    if args.len() < 2 {
        return Err("Must provide at least one argument");
    }

    let mut hours: u64 = 0;
    let mut minutes: u64 = 0;
    let mut seconds: u64 = 0;

    let mut i = 1;
    while i < args.len() {
        if args[i].eq("-h") {
            if i < args.len() - 1 {
                hours += args[i+1]
                    .parse::<u64>()
                    .expect("Argument after -h must be a number");
                i += 1;
            } else {
                return Err("-h must be followed by another argument");
            }
        } else if args[i].eq("-m") {
            if i < args.len() - 1 {
                minutes += args[i+1]
                    .parse::<u64>()
                    .expect("Argument after -m must be a number");
                i += 1;
            } else {
                return Err("-m must be followed by another argument");
            }
        } else if args[i].eq("-s") {
            if i < args.len() - 1 {
                seconds += args[i+1]
                    .parse::<u64>()
                    .expect("Argument after -s must be a number");
                i += 1;
            } else {
                return Err("-s must be followed by another argument");
            }
        } else {
            seconds += args[i]
                .parse::<u64>()
                .expect("Lone argument must be integer or flag");
        }

        i += 1;
    }

    let hms_time: HmsTime = HmsTime { h: hours, m: minutes, s: seconds };
    Ok(hms_time.rebalanced())
}

fn run_timer_for(tv: HmsTime) {
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

fn main() {
    println!("Please standby while we process your request.");
    println!("An agent will be with you shortly.");

    // NOTE: This is its own scope to make it clear that args is invalid after
    //       ownership is transferred to process_args
    let time_value = {
        let args: Vec<String> = env::args().collect();

        match process_args(args) {
            Ok(val) => val,
            Err(msg) => {
                println!("{}", msg);
                panic!();
            }
        }
    };

    println!("Starting timer for {}h {}m {}s for a total of {}s", time_value.h, time_value.m, time_value.s, time_value.total_seconds());


    run_timer_for(time_value);
}
