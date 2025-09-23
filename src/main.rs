use public_ip;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncSeekExt, AsyncWriteExt};
use std::net::IpAddr;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::{Duration, Instant, UNIX_EPOCH};
use std::thread::sleep;
use std::{
    process::Command,
};

const SLEEP_DUR: u16 = 900;
const FILEPATH: &'static str = "/home/sam/my_ip/ip.list";
const COMMAND_PATH: &'static str = "/home/sam/rust/pio/git_script.sh";

async fn check_if_file_exist(path: &str) -> tokio::io::Result<File> {
    let path_buf = PathBuf::from_str(path).unwrap();
    
    if !path_buf.exists() {
        let f = File::create_new(&path_buf).await?;
        return Ok(f);
    }else {
        let f = OpenOptions::new()
            .write(true)
            .open(path_buf).await?;
        return Ok(f);
    }

    
}



#[tokio::main()]
async fn main() {
    let mut past_ip: IpAddr = IpAddr::from_str("0.0.0.0").unwrap();
    println!("started");
    loop {
        println!("loop start");
        let ip = public_ip::addr().await.unwrap();
        println!("current ip: {}, past ip: {}",ip.to_string(), past_ip.to_string());
        if ip != past_ip {
            println!("loop work");
            let ct = UNIX_EPOCH.duration_since(UNIX_EPOCH).unwrap();

            let string = format!("\n{:#?}({})",ct.as_secs(), ip);
            // let path = PathBuf::from_str(FILEPATH).unwrap();
            // let mut f = File::create(path).await.unwrap();
            let mut f = check_if_file_exist(FILEPATH).await.unwrap();
            let _size = f.seek(std::io::SeekFrom::End(0));
            match f.write_all(string.as_bytes()).await {
                Ok(..) => {},
                Err(e) => {eprintln!("err: {:?}", e)}
            }
            f.flush().await.unwrap();
            f.sync_data().await.unwrap();
                

            let mut _status = Command::new(COMMAND_PATH).spawn().unwrap();
            if !_status.wait().is_ok() {
                println!("err");
            }else {_status.kill().unwrap()}
            drop(f);

            past_ip = ip;
        }else {
            println!("loop sleep");
            let dur = Duration::from_secs(SLEEP_DUR as u64);
            sleep(dur);

        }

        println!("loop end");
    }
    
}
