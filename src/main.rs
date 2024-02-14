use std::sync::mpsc;
use std::thread;
use std::fs::File;
use std::fs::OpenOptions;
use anyhow::Result;
use clap::Parser;

use std::os::unix::fs::MetadataExt;

use std::io::Write;
use std::io::{Seek, SeekFrom};
use std::time::Instant;
use std::time::Duration;
use size_format::SizeFormatterBinary;

#[derive(Clone)]
struct WiperInfo {
    device: String,
    length: u64,
    block_size: u64,
}

struct WiperState {
    id: usize,
    info: WiperInfo,
    offset: u64,
    bytes_per_sec: u64
}

struct Wiper {
    id: usize,
    file: File,
    info: WiperInfo
}

#[derive(Debug)]
struct WiperProgress {
    id: usize,
    offset: u64,
    bytes_per_sec: u64
}

impl Wiper {
    pub fn new(id: usize, device: &str) -> Result<(Wiper, WiperInfo)> {
        let mut file = OpenOptions::new().read(true).write(true).open(device)?;
        let md = file.metadata()?;
        let block_size = md.blksize();
        let length = file.seek(SeekFrom::End(0))?;
        file.rewind()?;

        let info = WiperInfo{ device: device.to_string(), length, block_size };
        Ok((Wiper{ id, file, info: info.clone() }, info))
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn wipe(&mut self, tx: mpsc::Sender<WiperProgress>) {
        let buf = vec![ 0; self.info.block_size as usize ];
        let mut offset: u64 = 0;
        let mut last_report_timestamp = Instant::now();
        let mut last_report_offset: u64 = 0;
        while offset < self.info.length {
            self.file.write_all(&buf).expect("write error");
            offset += self.info.block_size;

            // Send updates every second
            let now = Instant::now();
            if now.duration_since(last_report_timestamp) > Duration::from_secs(1) {
                let bytes_per_sec = offset - last_report_offset;
                let progress = WiperProgress{ id: self.id, offset, bytes_per_sec };
                tx.send(progress).unwrap();
                last_report_timestamp = now;
                last_report_offset = offset;
            }
        }
        let progress = WiperProgress{ id: self.id, offset, bytes_per_sec: 0 };
        tx.send(progress).unwrap();
    }
}

/// Overwrites file/device contents with zeroes
#[derive(Parser)]
struct Cli {
    /// Devices to wipe
    device: Vec<String>
}

fn main() -> Result<()> {
    let args = Cli::parse();
    if args.device.is_empty() {
        println!("missing devices to wipe");
        return Ok(())
    }

    let (tx, rx) = mpsc::channel();

    let mut states = Vec::<WiperState>::new();
    let devices = args.device;
    for (id, dev) in devices.iter().enumerate() {
        let (mut wiper, info) = Wiper::new(id, dev)?;
        println!("{}: {}, {}B, using {}B blocks", wiper.get_id(), dev, SizeFormatterBinary::new(info.length), SizeFormatterBinary::new(info.block_size));
        states.push(WiperState{ id, info, offset: 0, bytes_per_sec: 0 });

        let tx = tx.clone();
        thread::spawn(move || {
            wiper.wipe(tx);
        });
    }
    println!();

    loop {
        // Try to fetch as many updates as possible
        loop {
            let result = rx.try_recv();
            match result {
                Ok(r) =>  {
                    let state = states.iter_mut().find(|s| s.id == r.id).expect("unknown id received");
                    state.offset = r.offset;
                    state.bytes_per_sec = r.bytes_per_sec;
                },
                Err(_) => { break; }
            };
        }
        let mut all_finished = true;
        for s in &states {
            let pct = (s.offset as f32 / s.info.length as f32) * 100.0;
            println!("{}: {}, {}B / {}B, {}B/sec, {:.2}% completed", s.id, s.info.device, SizeFormatterBinary::new(s.offset), SizeFormatterBinary::new(s.info.length), SizeFormatterBinary::new(s.bytes_per_sec), pct);
            if s.offset < s.info.length {
                all_finished = false;
            }
        }
        if all_finished {
            println!(">> All devices wiped, exiting");
            break;
        }
        println!();
        thread::sleep(Duration::from_secs(1));
    }
    Ok(())
}
