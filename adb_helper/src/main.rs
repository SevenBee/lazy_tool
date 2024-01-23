use adb_helper::adb::ADB;

#[tokio::main]
async fn main(){
    let adb = ADB::new().await;
    println!("hello world");
}