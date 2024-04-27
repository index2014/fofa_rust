# FOFA Rust
An simple wrapper fo for [FOFA API]("https://fofa.info/api") rewrite in rust. written in `edition 2021`.

FOFA is a search engine for Internet-connected devices. `FOFA API` helps developers integrate FOFA data easily in their own projects.

# Documentaion
[中文版本](docs/README_CN.MD)

[Documentaion](docs/README_EN.MD)

# Usage
``` bash
fofa -k xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx -f ip,host,port -q domain=apple.com -r 1-5 -s 10 -x -j
```

# Use as SDK
``` rust
use fofa;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = fofa::FofaClient {
        key : "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string(),    //xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
        fields : "host,ip,port,domain,server,os".to_string(),
        query_string: "domain=apple.com".to_string(),
        size: 10,
    };
    let response = client.search(1).await.unwrap();    //return FofaApiResponse Struct
    let result = client.get_results(&response);         //return Vec<Vec<String>>
    for inner_vec in &result{
        for item in inner_vec{
            print!("{},",item);
        }
        println!();
    }
    Ok(())
}
```
# Dependencies
``` toml
reqwest = { version = "0.12", features = ["json"] }
tokio = { version = "1", features = ["full"] }
base64 = {version = "0.22.0"}
json ={ version = "0.12.4"}
serde = { version = "1.0.198", features = ["derive"] }
serde_json ={ version = "1.0.116"}
argparse ={ version = "0.2.2"}
xlsxwriter = {version = "0.6.0"}
calamine = "0.24.0"
```

# License
This software is licensed under [MIT License]("https://opensource.org/licenses/mit")

# TODO
[ ] Develop a GUI