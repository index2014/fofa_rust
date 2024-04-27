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