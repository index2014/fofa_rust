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