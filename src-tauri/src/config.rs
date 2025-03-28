
use std::error::Error;

use tauri::{utils::config, State};

use crate::error::NotInitedBackupError;
#[derive(Serialize, Deserialize)]
struct LocalConfig{
    is_inited:bool,
    backup_place:String,
    backup_place_type:String,
    backup_place_dir:String,
    username:String, 
    password:String
}
impl LocalConfig {
    fn new()->Self{
        return LocalConfig{
            is_inited:false,
            backup_place: "".to_string(),
            backup_place_type: "".to_string(),
            backup_place_dir:"".to_string(),
            username: "".to_string(),
            password: "".to_string()
        };
    }
}
pub struct Config{
    is_inited:bool,
    backup_place:String,
    backup_place_type:String,
    username:String, 
    password:String,
    save_how_often_week:String,
    save_how_often_month:String,
    save_how_often_year:String,
    save_how_often_far:String
}
impl Config {
    fn new()->Self{
        return Config{
            is_inited:false,
            backup_place:"".to_string(),
            backup_place_type:"".to_string(),
            username:"".to_string(), 
            password:"".to_string(),
            save_how_often_week:"".to_string(),
            save_how_often_month:"".to_string(),
            save_how_often_year:"".to_string(),
            save_how_often_far:"".to_string()
        };
    }    
}
//加载配置
#[tauri::command]
pub fn load_config() -> Result<Config, dyn Error>{
    let path:&Path=Path::new("./config.json");
    let mut file:File=match File::open(path){
        Ok(f)=>f,
        Err(E)=>{
            {
                let f_write:File=File::create(path).unwrap();
                write!(&f_write, "{{\"backupPlace\":\"\", \"backupPlaceType\":\"\"}}").unwrap();
            }
            let f:File=File::open(path).unwrap();
            f
        }
    };
    let mut buf:String=String::new();
    let _= file.read_to_string(&mut buf);
    let local_config:LocalConfig=match serde_json::from_str(&buf) {
        Ok(c)=>c,
        Err(E)=>LocalConfig::new()
    };
    let config:Config=Config::new();
    if local_config.is_inited==true{
        if local_config.backup_place_type == "smb".to_string() {
            let client=match connect_smb(local_config.backup_place, 
                local_config.backup_place_dir, local_config.username, local_config.password){
                    Ok(c)=>c,
                    Err(e)=>return Err(E)
                };
            let path=std::env::temp_dir();
            path.push("solar-backup");
            path.push("database.db");
            let file: Box<File>=Box::new(match File::create(path) {
                Ok(f)=>f,
                Err(E)=>return Err(E)
            });
            let remote_path=path::new("database.db");
            client.open_file(remote_path, file);
            drop(file);
            if let Err(e) = get_config_from_sqlite(path, &mut config) {
                return Err(e)
             };
        }
        else if local_config.backup_place_type == "alipan".to_string(){
            panic!("Unsupported");
        }
        else if local_config.backup_place_type == "local".to_string(){
            let path=path::new(local_config.backup_place);
            if let Err(e) = get_config_from_sqlite(path, &mut config) {
               return Err(e)
            };
        }
    }
    return Ok(config);
}
//连接SMB（Windows）
#[cfg(target_os="windows")]
fn connect_smb(backup_place:String, backup_place_dir:String, username:String, password:String) 
    -> Result<SmbFs, String>{
    let mut client = SmbFs::new(
        SmbCredentials::new(&backup_place, &backup_place_dir)
        .username(&username)
        .password(&password)
    );
    if client.connect().is_ok(){
        return Ok(client);
    }
    else {
        return Err("Can't connect to SMB!".to_string());
    }
}
//连接SMB（Linux）
#[cfg(target_os="linux")]
fn connect_smb(backup_place:String, backup_place_dir:String, username:String, password:String) 
    -> Result<SmbFs, dyn Error>{
    let mut client = match SmbFs::try_new(
            SmbCredentials::default()
                .server(&backup_place)
                .share(&backup_place_dir)
                .username(username)
                .password(password)
                .workgroup("WORKGROUP"),
            SmbOptions::default()
                .case_sensitive(true)
                .one_share_per_server(true),
        ){
            Ok(c)=>c,
            Err(E)=>return E
        };
    match client.connect() {
        Ok(c)=>c,
        Err(E)=>return E
    }
    return Ok(client);
}
//从SQLite读取配置
fn get_config_from_sqlite(path:Path, config:&mut config)->Result<_, dyn Error>{
    let connection = sqlite::open(path).unwrap();
    let query="select * from configs";
    let mut statement=connection.prepare(query);
    match statement.next() {
        Ok(res)=>{
            if(res==State::Row){
                config.save_how_often_week=match statement.read::<String, _>("save_how_often_week") {
                    Ok(res_str)=>res_str,
                    Err(e)=>return Err(E)
                };
                config.save_how_often_month=match statement.read::<String, _>("save_how_often_month"){
                    Ok(res_str)=>res_str,
                    Err(e)=>return Err(E)
                };
                config.save_how_often_year=match statement.read::<String, _>("save_how_often_year"){
                    Ok(res_str)=>res_str,
                    Err(e)=>return Err(E)
                };
                config.save_how_often_far=match statement.read::<String, _>("save_how_often_far"){
                    Ok(res_str)=>res_str,
                    Err(e)=>return Err(E)
                };
            }else if(res==State::Done){
                return Err(NotInitedBackupError);
            }
        },
        Err(e)=>{
            return Err(e);
        }
    }
    return Ok(());
}