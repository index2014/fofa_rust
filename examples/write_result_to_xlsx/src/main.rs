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