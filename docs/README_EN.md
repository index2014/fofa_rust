*# FOFA RUST SDK使用说明

## Description

A `SDK` of [`FOFA API`](https://fofa.info/api) writen by  `rust` .

### Run as a standalone software

```bash
Usage: fofa-client [OPTIONS]
Options:
-k, --key               Fofa API key
-f, --fields            Query fields
-r, --range             Page range, e.g 2-5
-q, --query_string      Query keywords
-s, --size              Result quantity
-x, --xlsx              Generate xlsx file
-j, --json              Generate json file
-h, --help              Help text
```

### Example

```bash
fofa -k xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx -f ip,port -r 1-2 -q domain=apple.com -s 100 -x -j
```

query `apple.com` then save as xlsx and json

```bash
fofa -k xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx -f ip,port -p 1 -q domain=apple.com -s 100
```

query `apple.com` only show on tty

### tty output

```bash
......
17.171.88.107,443
17.36.202.8,80
17.36.202.9,443
54.233.248.43,443
17.33.201.133,443
17.33.194.218,443
17.33.193.45,80
17.33.202.88,80
17.33.194.218,443
.......
```
### Use as a `SDK`
#### Add dependencies

```tmol
[dependencies]
fofa = {version = "*"}
tokio ={ version = "*"}
```
#### Get one page data as Vec<Vec<String>>
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

    /*
    Outputs:
    https://mwp9-st14-ssl.ls.apple.com,17.167.200.84,443,apple.com,,,
    https://apay-partner-api-lt-bz.apple.com,17.32.208.79,443,apple.com,Apple,,
    apay-partner-api-lt-bz.apple.com,17.32.208.79,80,apple.com,Apple,,
    https://entitlements.podcasts.apple.com,17.8.152.105,443,apple.com,daiquiri/5,,
    https://entitlements.books.apple.com,17.8.152.135,443,apple.com,daiquiri/5,,
    https://entitlements.music.apple.com,17.8.152.105,443,apple.com,daiquiri/5,,
    entitlements.apps.apple.com,17.8.152.135,80,apple.com,daiquiri/5,,
    https://entitlements.podcastsconnect.apple.com,17.8.152.174,443,apple.com,daiquiri/5,,
    entitlements.podcastsconnect.apple.com,17.8.152.105,80,apple.com,daiquiri/5,,
    entitlements.podcasts.apple.com,17.8.152.135,80,apple.com,daiquiri/5,,
    */
```
#### Get special pages data
```rust
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
    let mut count = 0;
    let response = client.get_selected_pages(2,10).await.unwrap();  //Vec<Vec<String>>，you can cut it in client.get_lines
    for inner_vec in &response{
        for item in inner_vec{
            print!("{}, ",item);
        }
        println!();
    }
    Ok(())
}
/* Output_Sample:
    https://mwp15-st14-ssl.ls.apple.com,17.167.200.82,443,apple.com,,,
    images.apple.com,106.4.158.58,80,apple.com,Apple,,
    https://mwp9-st14-ssl.ls.apple.com,17.167.200.84,443,apple.com,,,
    https://apay-partner-api-lt-bz.apple.com,17.32.208.79,443,apple.com,Apple,,
    apay-partner-api-lt-bz.apple.com,17.32.208.79,80,apple.com,Apple,,
    https://entitlements.podcasts.apple.com,17.8.152.105,443,apple.com,daiquiri/5,,
    https://entitlements.books.apple.com,17.8.152.135,443,apple.com,daiquiri/5,,
```
#### Get data by special fields
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
    let mut count = 0;
    let response = client.search(1).await.unwrap();                 //return FofaApiResponse Struct
    for value in client.get_fields_in_range("ip,port,host".to_string(), &response) {    //change your special fields here, returns Vec<String>
        print!("{},", value);
        count += 1;
        if count%3==0{                 //you have 3 fields upside, so here every 3 values get one `\n` to make output beauty
            println!();
        }
    }
    Ok(())
}
/*  Output Sample:
    17.188.22.142,443,https://okapi-services-euw1.apple.com,
    17.188.22.76,443,https://okapi-services-use1.apple.com,
    17.188.22.42,443,https://okapi-services-usw2.apple.com,
    17.188.22.157,80,okapi-services-euw1.apple.com,
    17.188.22.220,443,https://okapi-services-apse1.apple.com,
    17.188.22.91,80,okapi-services-use1.apple.com,
    17.188.22.22,80,okapi-services-usw2.apple.com,
    17.188.22.205,80,okapi-services-apse1.apple.com,
    17.8.131.136,80,okapi-services.apple.com,
    17.8.131.8,443,https://okapi-services.apple.com,
*/
```
#### Get special lines in data (data cut)
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
    let mut count = 0;
    let response = client.search(1).await.unwrap();                 //return FofaApiResponse Struct
    let selected_lines = client.get_lines(&response, 2, 4);         //return Vec<Vec<String>>
    for row in &selected_lines {                                    //Iter
        for item in row {
            print!("{} ", item);
        }
        println!();
    }
    Ok(())
}
/* Output: You can see there is only 2 rows, like what we need
    https://okapi-services-usw2.apple.com 17.188.22.42 443 apple.com Apple  
    okapi-services-euw1.apple.com 17.188.22.157 80 apple.com Apple 
*/
```
#### Return some account information
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
    let response = client.search(1).await.unwrap();                 //returns FofaApiResponse Struct
    let consumed_fpoint = client.get_consumed_fpoint(&response);     //returns remaining F points
    let required_fpoints = client.get_required_fpoints(&response);   //returns required F points
    let size = client.get_size(&response);                           //returns total number of assets found
    let mode = client.get_mode(&response);                           //returns current query mode
    let query = client.get_query(&response);                         //returns current query string
    print!("Remaining F points: {}\nRequired F points: {}\nTotal assets: {}\nQuery mode: {}\nQuery string: {}\n", consumed_fpoint, required_fpoints, size, mode, query);
    Ok(())
}


/*  Outputs:
    Remaining F points: 0
    Required F points: 0
    Total assets: 17177
    Query mode: extended
    Query string: domain="apple.com"
*/
```
#### Write to json file
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
    // let raw_response = client.search(1).await.unwrap();
    // let response = client.get_result(raw_response);              //Vec<Vec<String>>，only one page
    let response = client.get_selected_pages(2,10).await.unwrap();  //Vec<Vec<String>>，也可以拿去使用client.get_lines切片，或者write_data_to_*写入
    let location = "./".to_string();
    client.write_data_to_json(location, response);              
    Ok(())
}
/* Output:
    文件会输出在指定目录下，目录必须之前就存在
    Samples:
    [
  [
    "https://fsi-platform-partner.apple.com",
    "17.33.192.58",
    "443",
    "apple.com",
    "Apple",
    ""
  ],
  [
    "rsa-fsi-rno.apple.com",
    "17.179.240.221",
    "80",
    "apple.com",
    "Apple",
    ""
  ],
  .....
```
#### Write to xlsx
``` rust
use fofa;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = fofa::FofaClient {
        key : "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string(),    //xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
        fields : "host,ip,port,domain,server,os".to_string(),
        query_string: "domain=apple.com".to_string(),
        size: 10,
    };
    // let raw_response = client.search(1).await.unwrap();
    // let response = client.get_result(raw_response);              //Vec<Vec<String>>，只输出一页的情况
    let start = 2;
    let end = 10;
    let response = client.get_selected_pages(start, end).await.unwrap();  //Vec<Vec<String>> special pages in range
    let location = "./".to_string();
    client.write_data_to_excel(location, end - start+1, response);              //
}
/* Output:
    file will be in `location` directory, if directory is not exist, there will get panic.
*/
```
## Licence
[`MIT License`](https://opensource.org/licenses/mit)

# TODO
- [x] complete json outputs
- [ ] complete a GUI(on working)