use std::{process::{Command, Output}, io::{Error, Read, Write}, path::{Path, PathBuf}, fs::File};

pub struct ADB{

}

impl ADB {
    pub async fn new() -> Self {
        // 检查adb
        if ADB::get_version().is_err() {
                let adb_zip_file = if cfg!(target_os = "windows"){
                    "platform-tools-latest-windows.zip"
                }else if cfg!(target_os = "linux"){
                    "platform-tools-latest-linux.zip"
                }else if cfg!(target_os = "macos"){
                    "platform-tools-latest-darwin.zip"
                } else {
                    panic!("can not know target os")
                };
                let adb_zip_path = download_adb(adb_zip_file).await.unwrap();
                unzip(adb_zip_path)

                // add_to_env_variable();
        };
        
        Self {  }
    }

    fn get_version() -> Result<Output, Error>{
        Command::new("adb").arg("--version").output()
    }
}

/// 下载 adb 到home目录的下载文件中
async fn download_adb(filename: &str) -> Result<PathBuf, Box<dyn std::error::Error>>{
    // windows
    // https://googledownloads.cn/android/repository/platform-tools-latest-windows.zip
    // linux
    // https://googledownloads.cn/android/repository/platform-tools-latest-linux.zip
    // ios
    // https://googledownloads.cn/android/repository/platform-tools-latest-darwin.zip
    
    // let filename = if cfg!(target_os = "windows"){
    //     "platform-tools-latest-windows.zip"
    // } else if cfg!(target_os = "linux"){        
    //     "platform-tools-latest-linux.zip"        
    // } else {
    //     "platform-tools-latest-darwin.zip"
    // };
    let file_path = format!("./cache/{}",filename.to_string());
    let file_path = Path::new(&file_path);
    if !file_path.exists(){
        let client = reqwest::Client::new();
        let url = format!("https://googledownloads.cn/android/repository/{}", filename);
        println!("正在下载 adb tool: {}", filename);
        let body = client.get(url).send().await?.bytes().await?;
        
        let mut file = match File::create(file_path) {
            Err(why) => panic!("couldn't create {}", why),
            Ok(file) => file,
        };
        let content = body.bytes();
        let data: std::result::Result<Vec<_>,_> = content.collect();
        file.write_all(&data.unwrap())?;
        println!("下载写入完成");
    }


    Ok(file_path.to_owned())
}


fn add_to_env_variable(){}