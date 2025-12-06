use crate::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex};
use std::thread;

//#region input output data

#[derive(Debug)]
struct Job {
    input: PathBuf,
    output: PathBuf,
}

#[derive(Debug)]
enum JobResult {
    Ok,
    Err(String),
}

#[derive(Clone)]
struct StopSignal {
    signal: Arc<AtomicBool>,
}
impl StopSignal {
    pub fn stop(&self) {
        self.signal.store(true, Ordering::SeqCst)
    }
    pub fn should_stop(&self) -> bool {
        self.signal.load(Ordering::SeqCst)
    }
}

//#endregion

#[allow(dead_code)]
fn timestamp() -> String {
    let now = std::time::SystemTime::now();
    let duration = now.duration_since(std::time::UNIX_EPOCH).unwrap();
    let ms = duration.as_millis();
    let s = ms / 1000;
    let m = s / 60 % 60;
    return F!("{:02}:{:02}.{:03}", m, s % 60, ms % 1000);
}

pub struct Worker {
    total: i32,
    ok: i32,
    skip: i32,
    fail: i32,
    handles: Vec<thread::JoinHandle<()>>,
    jobtx: Option<Sender<Job>>,
    resrx: Option<Receiver<JobResult>>,
}

impl Worker {
    pub fn is_main() -> bool {
        thread::current().name() == Some("main")
    }

    pub fn init() -> Self {
        let (jobtx, jobrx) = channel::<Job>();
        let (restx, resrx) = channel::<JobResult>();
        let jobrx = Arc::new(Mutex::new(jobrx)); // to share it across threads
        let mut this =
            Self { jobtx: Some(jobtx), resrx: Some(resrx), ok: 0, skip: 0, fail: 0, total: 0, handles: vec![] };
        let stop_signal = StopSignal { signal: Arc::new(AtomicBool::new(false)) };

        let ncpus = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
        for id in 0..ncpus {
            let jobrx = Arc::clone(&jobrx);
            let restx = restx.clone();
            let stop_signal = stop_signal.clone();
            let handle = thread::spawn(move || Self::thread(id, jobrx, restx, stop_signal));
            this.handles.push(handle);
        }

        return this;
    }

    pub fn skip_job(&mut self) {
        self.total += 1;
        self.skip += 1;
    }
    pub fn add_job(&mut self, input: PathBuf, output: PathBuf) {
        self.total += 1;
        match &self.jobtx {
            None => eprintln!("sender is closed"),
            Some(tx) => _ = tx.send(Job { input, output }),
        }
    }

    fn thread(_id: usize, rx: Arc<Mutex<Receiver<Job>>>, tx: Sender<JobResult>, stop_signal: StopSignal) {
        let stop_on_error = *stop_on_error!();
        let mut dead = false;
        while !(dead || stop_signal.should_stop()) {
            let job = {
                let receiver = rx.lock().unwrap();
                receiver.recv() // block until a job is available
            };
            dead = match job {
                Err(_) => true, // caused by drop(sender)
                Ok(job) => match MdlxData::read(&job.input).and_then(|a| a.write(&job.output)) {
                    Ok(_) => tx.send(JobResult::Ok).is_err(),
                    Err(e) => {
                        yes!(stop_on_error, stop_signal.stop());
                        tx.send(JobResult::Err(e.to_string())).is_err()
                    },
                },
            };
        }
    }

    fn handle(&mut self) {
        let stop_on_error = *stop_on_error!();
        if let Some(rx) = &self.resrx {
            while let Ok(result) = rx.recv() {
                match result {
                    JobResult::Ok => self.ok += 1,
                    JobResult::Err(e) => {
                        self.fail += 1;
                        elog!("{}", e);
                        yes!(stop_on_error, break);
                    },
                }
            }
        }
    }

    pub fn join(mut self) -> Result<(), MyError> {
        self.jobtx = None; // close the sender
        self.handle(); // ?: do not return error, just log and keep going
        self.resrx = None; // close the receiver

        for h in self.handles {
            if let Err(e) = h.join() { // ?: do not return error, just log and keep going
                elog!("Failed to join thread: {:?}", e);
            }
        }

        let (ok, skip, error) = (self.ok, self.skip, self.fail);
        if self.total > 1 {
            print!("Converted {ok} files");
            yes!(skip > 0, print!(", {skip} skipped"));
            yes!(error > 0, print!(", {error} errors"));
            println!(".");
        };

        return Ok(());
    }
}
