mod emailer;

use std::io::Read;
use std::os::unix::raw::time_t;
use std::time::SystemTime;
use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
#[derive(Serialize, Deserialize)]
struct IpAddress {
    ipaddressid: u8,
    ipaddress: String,
    isactive: bool,
    datecreated: String
}

#[derive(Serialize, Deserialize)]
struct IpRunLog {
    iprunlogid: u8,
    rundate: String,
    ipupdated: bool
}
impl From<String> for IpAddress {
    fn from(value: String) -> Self {
        IpAddress {
            ipaddressid: 0,
            ipaddress: value,
            isactive: true,
            datecreated: Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
        }
    }
}

impl From<bool> for IpRunLog {
    fn from(value: bool) -> Self {
        IpRunLog {
            iprunlogid: 0,
            rundate: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            ipupdated: value
        }
    }
}

#[derive(Serialize, Deserialize)]
struct ApiResultWrapper<T> {
    Value: T
}
#[tokio::main]
async fn main() -> Result<()>{
    let run_log;

    let result = get_active_ip().await.expect("TODO: panic message");
    let real_public_ip = get_real_public_ip().await.expect("TODO: PANICKING");
    println!("{}", result.Value.get(0).unwrap().ipaddress);
    println!("{}", real_public_ip);

    if real_public_ip == result.Value.get(0).unwrap().ipaddress {
        println!("{}", String::from("ips match, success"));
        run_log = IpRunLog::from(false);
    } else {
        println!("{}", String::from("ipmismatch, update database to new publicip"));
        run_log = IpRunLog::from(true);
        deactivate_ip_records().await.expect("Panik");
        let new_ip = IpAddress::from(real_public_ip);
        insert_new_ip_record(&new_ip).await.expect("PANIK");
        emailer::send_ips_changed_alert().await.expect("PANIK");
    }
    insert_new_ip_runlog(&run_log).await.expect("PANIK");
    Ok(())
}

async fn get_active_ip() -> std::result::Result<ApiResultWrapper<Vec<IpAddress>>, Box<dyn std::error::Error>> {
    let mut body = reqwest::get("http://localhost:8080/ip/active")
        .await?
        .text()
        .await?;
    println!("body: {:?}", body);

    let result: ApiResultWrapper<Vec<IpAddress>> = serde_json::from_str(&body)?;

    Ok(result)
}

async fn get_real_public_ip() -> std::result::Result<String, Box<dyn std::error::Error>> {
    let  body = reqwest::get("http://ipinfo.io/ip")
        .await?
        .text()
        .await?;

    Ok(body)
}

async fn deactivate_ip_records() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client.put("http://localhost:8080/ip/deactivateall")
        .body("")
        .send()
        .await?;
    Ok(())
}

async fn insert_new_ip_record(ipaddress: &IpAddress) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let serialized_body = serde_json::to_string(&ipaddress).unwrap();
    let client = reqwest::Client::new();
    let res = client.post("http://localhost:8080/ip/new")
        .body(serialized_body)
        .send()
        .await?;

    Ok(())
}

async fn insert_new_ip_runlog(ip_run_log: &IpRunLog) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let serialized_body = serde_json::to_string(&ip_run_log).unwrap();
    let client = reqwest::Client::new();
    let res = client.post("http://localhost:8080/ip/runlog/new")
        .body(serialized_body)
        .send()
        .await?;

    Ok(())
}
