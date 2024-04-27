use reqwest;
use std::str;
use base64::{Engine as _, engine::{general_purpose}};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use xlsxwriter::*;
use std::collections::HashMap;
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct FofaApiSearch {    //接受fofa返回的response
    error: bool,
    consumed_fpoint: u16,
    required_fpoints: u16,
    size: u32,
    mode: String,
    query: String,
    results: Vec<Vec<String>>,
    
}
impl FofaApiSearch {
    fn new() -> FofaApiSearch {
        FofaApiSearch {
            error: true,
            consumed_fpoint: 0,
            required_fpoints: 0,
            size: 0,
            mode: "".to_string(),
            query: "".to_string(),
            results: Vec::new(),
        }
    }
}


pub struct FofaClient {     //发送给fofa的request
    pub key: String,
    pub fields: String,
    pub query_string: String,
    pub size: u32,
    pub outfile: bool,
    workbook: xlsxwriter::Workbook,
    json: String,
}
impl FofaClient{
    async fn search(&self, page: u16) -> Result<FofaApiSearch, reqwest::Error> {
        let mut base64_encoded = String::new();
        general_purpose::STANDARD.encode_string(self.query_string.clone(), &mut base64_encoded);
        let url = format!(
            "https://fofa.info/api/v1/search/all?&key={}&qbase64={}&fields={}&page={}&size={}",
            self.key, base64_encoded, self.fields, page, self.size
        );
        let response = reqwest::get(url).await?.text().await?;
        let fofaresponse: FofaApiSearch = serde_json::from_str(&response[..]).unwrap();
        if cfg!(debug_assertions){
            println!("API调用成功");
        }
        Ok(fofaresponse)
    }
    fn get_consumed_fpoint(&self, raw_response:FofaApiSearch) -> u16 {
        if cfg!(debug_assertions){
            println!("剩余fpoint获取成功");
        }
        raw_response.consumed_fpoint.clone()
    }
    fn get_required_fpoints(&self, raw_response:FofaApiSearch) -> u16 {
        if cfg!(debug_assertions){
            println!("需要fpoint获取成功");
        }
        raw_response.required_fpoints.clone()
    }
    fn get_size(&self, raw_response:FofaApiSearch) -> u32 {
        if cfg!(debug_assertions){
            println!("Total数据量获取成功");
        }
        raw_response.size.clone()
    }
    fn get_mode(&self, raw_response:FofaApiSearch) -> String {
        if cfg!(debug_assertions){
            println!("接口模式获取成功");
        }
        raw_response.mode.clone()
    }
    fn get_query(&self, raw_response:FofaApiSearch) -> String {
        if cfg!(debug_assertions){
            println!("Querry String获取成功");
        }
        raw_response.query.clone()
    }
    fn get_results(&self, raw_response:FofaApiSearch) -> Vec<Vec<String>> {
        if cfg!(debug_assertions){
            println!("成功获取到Vec<Vec<String>>");
        }
        raw_response.results.clone()
    }
    fn get_lines(&self, raw_response:FofaApiSearch, start: usize, end: usize) -> Vec<Vec<String>> {
        let results = raw_response.results.clone();
        let selected_lines = results[start..end].to_vec();
        if cfg!(debug_assertions){
            println!("正在获取{}到{}行的数据",start,end);
        }
        selected_lines
    }
    fn get_selected_fields(&self, selected_fields: String, raw_response:FofaApiSearch) -> Vec<String> {
        let fields: Vec<&str> = self.fields.split(',').collect();
        let mut selected_values = Vec::new();
        for result in raw_response.results {
            let mut hashmap = HashMap::new();
            for (field, value) in fields.iter().zip(result.iter()) {
                hashmap.insert(field.to_string(), value.clone());
            }
            for selected_field in selected_fields.split(',') {
                if let Some(value) = hashmap.get(selected_field) {
                    selected_values.push(value.clone());
                }
            }
        }
        if cfg!(debug_assertions){
            println!("成功获取{}字段的数据",selected_fields);
        }
        selected_values
    }
    async fn get_selected_pages(&self, start: u16, end: u16) -> Result<Vec<Vec<String>>, reqwest::Error> {
        let mut all_responses = Vec::new();
        for i in start..=end {
            let raw_response = self.search(i).await?;
            let wait_to_write = self.get_results(raw_response);
            all_responses.extend(wait_to_write);
            if cfg!(debug_assertions) {
                println!("正在获取第{}页数据", i);
            }
        }
        Ok(all_responses)
    }
    fn write_data_to_excel(&mut self, pages :u16, raw_response:FofaApiSearch) -> Result<(), XlsxError> {
        let data = raw_response.results.clone();
        let rows = self.size * pages as u32;
        let cols = self.fields.split(',').count() as u32;
        let mut worksheet = self.workbook.add_worksheet(None)?;
        for i in 0..cols {
            worksheet.set_column(i as u16, i as u16, 20.0, None)?;
        }
        worksheet.merge_range(0, 0, 0, cols as u16 - 1, &self.query_string, None)?;     //第一行写query_string
        let mut col_index = 0;
        for field in self.fields.split(',') {               //第二行写fields
            worksheet.write_string(1, col_index, field, None)?;
            col_index += 1;
        }
        for i in 0..rows+2 {            //+2行抬头
            if i < 2 {
                continue;
            }
            for j in 0..cols {          //第三行开始写正文
                if let Some(value) = data.get((i - 2) as usize).and_then(|row| row.get(j as usize)) {
                    worksheet.write_string(i as u32, j as u16, value, None)?;
                } else {
                    worksheet.write_blank(i as u32, j as u16, None)?;
                }
            }
        }
        if cfg!(debug_assertions) {
            println!("一共写入{}条数据到xlsx", rows);
        }
        Ok(())
    }
    fn write_data_to_json(&self, raw_response:FofaApiSearch){
        let data = raw_response.results.clone();
        let json = json!(data);
        let json_string = serde_json::to_string(&json).unwrap();
        std::fs::write(&self.json, json_string).expect("Failed to write JSON file");
    }
    fn gettime() -> u64 {                               //写到文件名里的时间戳
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).expect("SystemTime before UNIX EPOCH!");
        let timestamp = duration.as_secs();
        timestamp
    }
}

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let fields = "host,ip,port,domain,server,os".to_string();
        let mut client = FofaClient {
            key : "12345".to_string(),
            fields : fields.clone(),
            query_string: "domain=apple.com".to_string(),
            size: 10,
            outfile: false, // Default to false
            workbook: Workbook::new(&format!("fofa导出_fields={}_{}.xlsx", fields, FofaClient::gettime())[..]).unwrap(),
            json: format!("fofa导出_fields={}_{}.json", fields, FofaClient::gettime()),
        };

        /*
        Export to json
        let response = client.search(1).await.unwrap();
        client.write_data_to_json(response);
        */
        //println!("{}", json_string);
        /*
        let raw_response = client.search(1).await.unwrap();
        let selected_lines = client.get_lines(raw_response, 0, 5);
        for row in &selected_lines {
            for item in row {
                print!("{} ", item);
            }
            println!();
        }
        */
        //let selected_fields = "ip,port,host".to_string(); // 你可以根据需要更改这个字段
        //let selected_values = client.get_selected_fields("ip,port,host".to_string(), raw_response);
        /*
        for value in client.get_selected_fields("ip,port,host".to_string(), raw_response) {
            println!("{}", value);
        }
        */
        Ok(())
    }