use tokio::runtime::Runtime;

type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

const PROXY_FILE_NAME: &str = "proxies.txt";
const GOOD_PROXIES_FILE: &str = "good_proxies.txt";
const URL_TO_CHECK: &str = "https://api.ipify.org?format=json";

fn main() -> BoxResult<()> {
    let rt = Runtime::new().unwrap();

    let proxies = list_proxies()?;

    rt.block_on(async {
        let mut proxies_details = Vec::with_capacity(proxies.len());
        let proxies_len = proxies.len();

        for (ii, proxy) in proxies.into_iter().enumerate() {
            println!("Proxy: {} of {}", ii + 1, proxies_len);
            let details = get_proxy_details(proxy).await;

            if let Ok(detail) = details {
                println!(
                    "Took {} ms for {}",
                    detail.latancy.as_millis(),
                    detail.proxy.username
                );
                proxies_details.push(detail);
            } else {
                eprintln!("Skipping due to error on proxy: Line {}", ii + 1)
            }
        }

        proxies_details.sort_by(|a, b| a.latancy.cmp(&b.latancy));

        let good_proxies_raw = proxies_details
            .into_iter()
            .map(|aa| aa.proxy.raw)
            .collect::<Vec<_>>()
            .join("\r\n");

        use std::io::prelude::Write;

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(GOOD_PROXIES_FILE)
            .unwrap();

        file.write_all(good_proxies_raw.as_bytes()).unwrap();
    });

    Ok(())
}

#[derive(Debug)]
struct Proxy {
    url: String,
    port: String,
    username: String,
    password: String,
    raw: String,
}

fn list_proxies() -> BoxResult<Vec<Proxy>> {
    use std::io;
    use std::io::prelude::*;

    let file = std::fs::File::open(PROXY_FILE_NAME)?;
    let lines = io::BufReader::new(file).lines();

    let mut proxies = vec![];

    for line in lines {
        if let Ok(ll) = line {
            let raw = ll.to_string();
            let split: Vec<_> = ll.split(":").collect();
            if split.len() == 4 {
                let proxy = Proxy {
                    url: split[0].to_string(),
                    port: split[1].to_string(),
                    username: split[2].to_string(),
                    password: split[3].to_string(),
                    raw,
                };
                proxies.push(proxy);
            }
        }
    }

    Ok(proxies)
}

#[derive(Debug)]
struct ProxyWithLatancy {
    proxy: Proxy,
    latancy: std::time::Duration,
}

async fn get_proxy_details(proxy: Proxy) -> BoxResult<ProxyWithLatancy> {
    let start = std::time::Instant::now();

    let rproxy = reqwest::Proxy::http(&format!(
        "http://{}:{}@{}:{}",
        proxy.username, proxy.password, proxy.url, proxy.port
    ))?;
    let client = reqwest::Client::builder()
        .proxy(rproxy)
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    for _ in 0..10 {
        let resp = client.get(URL_TO_CHECK).send().await;

        if let Ok(resp) = resp {
            let text = resp.text().await?;
            if text.contains("ip") {
                continue;
            } else {
                return Err("Invalid resp from api".into());
            }
        } else {
            return Err("Error on sending request".into());
        }
    }

    Ok(ProxyWithLatancy {
        proxy,
        latancy: start.elapsed(),
    })
}
