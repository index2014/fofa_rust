*# FOFA RUST SDK使用说明

## 简介

基于 [`FOFA API`](https://fofa.info/api) 编写的 `rust` 版 `SDK`, 方便 `rust` 开发者快速将 `FOFA` 集成到自己的项目中，接下来会做一个简单的gui。

### 作为独立程序使用

```bash
Usage: fofa-client [OPTIONS]
Options:
-k, --key               Fofa API      key
-f, --fields            查询字段
-r, --range             页码区间, e.g 2-5
-q, --query_string      查询关键词
-s, --size              结果数量
-x, --xlsx              生成 xlsx 文件
-j, --json              生成 json 文件
-h, --help              帮助文本
```

### 举例

```bash
fofa -k xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx -f ip,port -r 1-2 -q domain=apple.com -s 100 -x -j
```

查询apple.com并保存为xlsx和json

```bash
fofa -k xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx -f ip,port -p 1 -q domain=apple.com -s 100
```

查询apple.com不保存为xlsx

### 控制台输出

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
### 开发用SDK
#### 添加依赖

```tmol
[dependencies]
fofa = {version = "*"}
tokio ={ version = "*"}
```
#### 返回特定一页数据 Vec<Vec<String>>
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
#### 返回一定页数区间内的数据
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
    let response = client.get_selected_pages(2,10).await.unwrap();  //Vec<Vec<String>>，也可以拿去使用client.get_lines切片
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
#### 返回特定字段的数据
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
    for value in client.get_fields_in_range("ip,port,host".to_string(), &response) {    //这里修改你需要的字段，这里返回的是Vec<String>
        print!("{},", value);
        count += 1;
        if count%3==0{                 //3是字段数，如果不想改就把selected_fields定义成单独的一个String，数字替换成selected_fields.spilit(",")
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
#### 返回一段数据中的特定行数(切割数据)
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
    let selected_lines = client.get_lines(&response, 2, 4);         //返回的是Vec<Vec<String>>
    for row in &selected_lines {                                    //遍历输出
        for item in row {
            print!("{} ", item);
        }
        println!();
    }
    Ok(())
}
/* Output 只输出两行，这里也可以是任何fofaapi返回的results的拼接，或者裁切
    https://okapi-services-usw2.apple.com 17.188.22.42 443 apple.com Apple  
    okapi-services-euw1.apple.com 17.188.22.157 80 apple.com Apple 
*/
```
#### 返回一些账号信息
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
    let response = client.search(1).await.unwrap();                 //return FofaApiResponse Struct
    let consumed_fpoint = client.get_consumed_fpoint(&response);     //return consumed_fpoint 剩余F点
    let required_fpoints = client.get_required_fpoints(&response);   //return required fpoint 需求的F点
    let size = client.get_size(&response);                           //return size 搜索到的资产总数量
    let mode = client.get_mode(&response);                           //return mod 当前查询的模式
    let query = client.get_query(&response);                         //return query_string 当前查询的查询语句
    print!("剩余F点：{}\n需求F点：{}\n资产总数：{}\n查询模式：{}\n查询语句：{}\n", consumed_fpoint, required_fpoints, size, mode, query);
    Ok(())
}


/*  Outputs:
    剩余F点：0
    需求F点：0
    资产总数：17177
    查询模式：extended
    查询语句：domain="apple.com"
*/
```
#### 写数据到json
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
    // let response = client.get_result(raw_response);              //Vec<Vec<String>>，只输出一页的情况
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
#### 写数据到xlsx
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
    let response = client.get_selected_pages(start, end).await.unwrap();  //Vec<Vec<String>>，也可以拿去使用client.get_lines切片，或者write_data_to_*写入
    let location = "./".to_string();
    client.write_data_to_excel(location, end - start+1, response);              //页码也可以手动输入，但是太小了会panic
}
/* Output:
    文件会输出在指定目录下，目录必须之前就存在
*/
```
## 协议
[`MIT License`](https://opensource.org/licenses/mit)

# TODO
- [x] 完成json输出
- [ ] 完成一个GUI