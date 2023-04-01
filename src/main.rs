use std::{time::{Duration}, path::PathBuf, fs, sync::{Arc, Mutex}, thread};
use buttplug::{core::connector::new_json_ws_client_connector, client::{ButtplugClient, ScalarValueCommand}};
use notify_debouncer_mini::{notify::*,new_debouncer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let list = Arc::new(Mutex::new(vec![0u8]));

    let watchpath: PathBuf = [ &std::env::var("APPDATA").unwrap(), "Factorio", "script-output" ].iter().collect();
    let mut outputfile = watchpath.clone();
    outputfile.push("haptics-current");

    {
        let list = Arc::clone(&list);

        thread::spawn(move || {
            let (tx, rx) = std::sync::mpsc::channel();
        
            let mut debouncer = new_debouncer(Duration::from_millis(400), None, tx).unwrap();
        
            debouncer.watcher().watch(&watchpath, RecursiveMode::NonRecursive).unwrap();
        
            for events in rx {
                let mut read_haptics = false;

                if let Ok(events) = events {
                    for e in events {
                        if e.path.file_name().map(|s| s == "haptics-current") == Some(true) {
                            read_haptics = true;
                        }
                    }
                }

                if read_haptics {
                    let contents = fs::read_to_string(&outputfile).unwrap();
                    let mut list = list.lock().unwrap();
                    list.clear();

                    for part in contents.split(',') {
                        let n: u8 = part.parse().unwrap();
                        list.push(n);
                    }

                    // eprintln!("{:?}", list);
                }
            }

        });
    }

    let connector = new_json_ws_client_connector("ws://127.0.0.1:12345");

    // eprintln!("{:?}", list.lock().unwrap());

    let client = ButtplugClient::new("Factorio Haptics");
    client.connect(connector).await?;
    
    println!("Connected!");

    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
    client.start_scanning().await?;
    interval.tick().await;
    interval.tick().await;
    client.stop_scanning().await?;

    let mut list_i = 0;
    while client.connected() {
        interval.tick().await;

        let vib_amount: f64 = {
            let list = list.lock().unwrap();
            if list_i >= list.len() {
                list_i = 0;
            }
            let n = list[list_i];
            // eprintln!("{} in {:?} [{}]", n, list, list_i);
            list_i += 1;

            n as f64 / 100.0
        };

        let vib_amount = ScalarValueCommand::ScalarValue(vib_amount);
        for device in client.devices() {
            device.vibrate(&vib_amount).await?;
        }
    }

    println!("Disconnected, exiting.");
    Ok(())
}
