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