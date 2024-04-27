use reqwest;
use std::str;
use base64::{Engine as _, engine::{general_purpose}};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use xlsxwriter::*;
use std::collections::HashMap;
use serde_json::json;

#[derive(Serialize, Deserialize)]
/// fofa的返回值
pub struct FofaApiSearch {    //接受fofa返回的response
    pub error: bool,
    pub consumed_fpoint: u16,
    pub required_fpoints: u16,
    pub size: u32,
    pub mode: String,
    pub query: String,
    pub results: Vec<Vec<String>>,
    
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

/// 请求客户端的数据结构，这里的page单独以一个usize传入到search，提供遍历
pub struct FofaClient {     //发送给fofa的request
    pub key: String,
    pub fields: String,
    pub query_string: String,
    pub size: u32,
}
impl FofaClient{
    /// 大批量调用的情况下使用search接口多些，host聚合用的不多
    /// 所以这里就只写了search接口调用
    /// Usage: let client = fofa::FofaClient {.....};
    /// client.search(1);  //1是页数，第几页
    pub async fn search(&self, page: u16) -> Result<FofaApiSearch, reqwest::Error> {
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
    /// 获取剩余F点数量，返回一个u16
    pub fn get_consumed_fpoint(&self, raw_response:&FofaApiSearch) -> u16 {
        if cfg!(debug_assertions){
            println!("剩余fpoint获取成功");
        }
        raw_response.consumed_fpoint.clone()
    }
    /// 获取本次查询需要的F点数量，返回一个u16
    pub fn get_required_fpoints(&self, raw_response:&FofaApiSearch) -> u16 {
        if cfg!(debug_assertions){
            println!("需要fpoint获取成功");
        }
        raw_response.required_fpoints.clone()
    }
    /// 获取本次查询结果的数据总条数，返回一个u32
    pub fn get_size(&self, raw_response:&FofaApiSearch) -> u32 {
        if cfg!(debug_assertions){
            println!("Total数据量获取成功");
        }
        raw_response.size.clone()
    }
    /// 获取本次查询执行的接口，返回String
    pub fn get_mode(&self, raw_response:&FofaApiSearch) -> String {
        if cfg!(debug_assertions){
            println!("接口模式获取成功");
        }
        raw_response.mode.clone()
    }
    /// 获取本次查询传入的查询语句，返回String
    pub fn get_query(&self, raw_response:&FofaApiSearch) -> String {
        if cfg!(debug_assertions){
            println!("Querry String获取成功");
        }
        raw_response.query.clone()
    }
    /// 获取到API返回的result，这里使用`Vec<Vec<String>>`来存，也就是[[..]:[..],[..]:[..],...]这样的JSON
    /// 这里没有经过url_escape，在做GUI的时候记得添加，不然会乱码，返回`Vec<Vec<String>>`
    pub fn get_results(&self, raw_response:&FofaApiSearch) -> Vec<Vec<String>> {
        if cfg!(debug_assertions){
            println!("成功获取到Vec<Vec<String>>");
        }
        raw_response.results.clone()
    }
    /// 按结果中的行获取数据，返回result
    pub fn get_lines(&self, raw_response:&FofaApiSearch, start: usize, end: usize) -> Vec<Vec<String>> {
        let results = raw_response.results.clone();
        let selected_lines = results[start..end].to_vec();
        if cfg!(debug_assertions){
            println!("正在获取{}到{}行的数据",start,end);
        }
        selected_lines
    }
    /// 把传入的`Vec<Vec<String>>`转换成一个Hashmap，供输入字段进行查询，这里考虑过使用数学方法(遍历pop)降低时间复杂度到O(n)
    /// 但是后来觉得还是直观一点比较好，返回的是一个Fofaclient，O(n^2)
    pub fn get_fields_in_range(&self, selected_fields: String, raw_response:&FofaApiSearch) -> Vec<String> {
        let fields: Vec<&str> = self.fields.split(',').collect();
        let mut selected_values = Vec::new();
        for result in &raw_response.results {
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
    /// 调用search函数，获取search的response并拼接，返回`Vec<Vec<String>>`,O(n)
    pub async fn get_selected_pages(&self, start: u16, end: u16) -> Result<Vec<Vec<String>>, reqwest::Error> {
        let mut all_responses = Vec::new();
        for i in start..=end {
            let raw_response = self.search(i).await?;
            let wait_to_write = self.get_results(&raw_response);
            all_responses.extend(wait_to_write);
            if cfg!(debug_assertions) {
                println!("正在获取第{}页数据", i);
            }
        }
        Ok(all_responses)
    }
    /// 遍历`Vec<Vec<String>>`并写入xlsx，O(n)
    pub fn write_data_to_excel(&mut self, location:String, pages :u16, raw_response:&Vec<Vec<String>>) -> Result<(), XlsxError> {
        let rows = self.size * pages as u32;
        let cols = self.fields.split(',').count() as u32;
        let workbook= Workbook::new(&format!("{}fofa导出_query={}_fields={}_{}.xlsx", location, self.query_string, self.fields, FofaClient::gettime())[..]).unwrap();
        let mut worksheet = workbook.add_worksheet(None)?;
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
                if let Some(value) = raw_response.get((i - 2) as usize).and_then(|row| row.get(j as usize)) {
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
    /// 将数据写入到json，O(n)
    pub fn write_data_to_json(&self,location: String, raw_response:&Vec<Vec<String>>){
        let json = json!(raw_response);
        let json_string = serde_json::to_string(&json).unwrap();
        let json_file_name = format!("{}fofa导出_query={}_fields={}_{}.json", location, self.query_string ,self.fields, FofaClient::gettime());
        std::fs::write(json_file_name, json_string).expect("Failed to write JSON file");
    }
    /// 获取当前时间戳的函数，返回u64
    pub fn gettime() -> u64 {                               //写到文件名里的时间戳
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).expect("SystemTime before UNIX EPOCH!");
        let timestamp = duration.as_secs();
        timestamp
    }
}