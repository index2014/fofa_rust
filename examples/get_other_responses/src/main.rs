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