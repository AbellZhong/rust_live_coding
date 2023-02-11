use std::{path::Path, ffi::OsStr, f32::consts::E};

use clap::{Parser};

use url::Url;

mod web2image;

fn get_file_ext(path: &Path) -> Option<String>{
    path.extension()
    .and_then(|p| OsStr::to_str(p))
    .and_then(|ext| {
        let ext = ext.to_lowercase();

        match ext.as_str() {
            "jpg" | "png" | "jpeg" => Some(ext),
            _ => None
        }
    })
}
 
fn valid_filename(name: &str) -> Result<(), String> {
    let path = Path::new(name);

    let parent = path.parent().and_then(|p| p.is_dir().then(|| p));

    let file_ext = get_file_ext(path);

    if parent.is_none() || file_ext.is_none() {
        Err("文件夹不存在或者文件类型不符合(只能由jpg、jpeg、png结尾)".to_string())
    }else{
        Ok(())
    }
}

fn valid_url(url: &str) -> Result<(), String> {
    Url::parse(url).expect("URL格式错误");

    Ok(())
}


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author = "zhongshenchao@foxmail.com", version = "0.1", about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long, default_value = "/tmp/snapshort.jpg", value_parser = valid_filename)]
   output: String,

   #[arg (value_parser = valid_url)]
   url: String,

}


fn main() {
    let args = Args::parse();

    println!("{:#?}", args);


}
