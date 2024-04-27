mod fofa;
use argparse::{ArgumentParser, StoreTrue, Store};
use std::collections::HashMap;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut range = String::new();
    let mut xlsx_flag = false;
    let mut json_flag = false;
    if env::args().len() == 1 {
        println!("Usage: fofa-client [OPTIONS]");
        println!("Options:");
        println!("-k, --key               Fofa API      key");
        println!("-f, --fields            查询字段");
        println!("-r, --range             页码区间, e.g 2-5");
        println!("-q, --query_string      查询关键词");
        println!("-s, --size              结果数量");
        println!("-x, --xlsx              生成 xlsx 文件");
        println!("-j, --json              生成 json 文件");
        println!("-h, --help              帮助文本");
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "No arguments provided")) as Box<dyn std::error::Error>);
    }

    let mut client = fofa::FofaClient {
        key: String::new(),
        fields: String::new(),
        query_string: String::new(),
        size: 100,
    };

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Fofa");
        ap.refer(&mut client.key).add_option(&["-k", "--key"], Store, "Fofa API key");
        ap.refer(&mut client.fields).add_option(&["-f", "--fields"], Store, "查询字段");
        ap.refer(&mut range).add_option(&["-r", "--range"], Store, "页码区间");
        ap.refer(&mut client.query_string).add_option(&["-q", "--query_string"], Store, "查询语句");
        ap.refer(&mut client.size).add_option(&["-s", "--size"], Store, "结果数量");
        ap.refer(&mut xlsx_flag).add_option(&["-x", "--xlsx"], StoreTrue, "输出xlsx");
        ap.refer(&mut json_flag).add_option(&["-j", "--json"], StoreTrue, "输出json");
        ap.parse_args_or_exit();
    }

    let (start, end) =  split_string(&range[..]);
    let result = client.get_selected_pages(start, end).await.unwrap();
    if xlsx_flag{
    println!("{}",end-start+2);
        let _ = client.write_data_to_excel("".to_string(), end-start+1, &result);
    }
    if json_flag{
        let _ = client.write_data_to_json("".to_string(), &result);
    }
    for inner_vec in &result{
        for item in inner_vec{
            print!("{},",item);
        }
        println!();
    }
    Ok(())
}

fn split_string(input: &str) -> (u16, u16) {
    let parts: Vec<&str> = input.split('-').collect();
    let a = parts[0].parse::<u16>().unwrap();
    let b = parts[1].parse::<u16>().unwrap();
    (a, b)
}