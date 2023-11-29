use std::sync::OnceLock;
use std::{
    env,
    ffi::OsStr,
    fs::{self},
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = env!("CARGO_PKG_NAME"), version=env!("CARGO_PKG_VERSION") ,about = env!("CARGO_PKG_DESCRIPTION"))]
struct Opt {
    #[structopt(short = "f", long = "file", help = "指定文件名")]
    filename: Option<String>,
    #[structopt(short = "d", long = "dir", help = "指定文件夹名")]
    dir: Option<String>,

    #[structopt(short = "a", long = "output-full", help = "输出完整地址")]
    of: bool,
}
static OPT: OnceLock<Opt> = OnceLock::new();

fn init() {
    std::thread::spawn(|| {
        OPT.get_or_init(|| Opt::from_args());
    })
    .join()
    .unwrap();
}
fn main() {
    // 解析命令参数

    init();

    let bind = env::current_dir().expect("获取工作路径失败");
    let path_buf = bind.as_os_str();
    if let Some(dir) = &OPT.get().unwrap().dir {
        find(path_buf, 0, dir.as_str());
        return;
    } else if let Some(filename) = &OPT.get().unwrap().filename {
        find(path_buf, 1, filename.as_str());
        return;
    } else {
        println!("Please provide either --file or --dir parameter.");
        return;
    }
}
fn find(path: &OsStr, st: i32, target: &str) {
    if !find_current_path(path, st, target){
        return;
    };
    find_depth_path(path, st, target);
}
fn find_depth_path(path: &OsStr, st: i32, target: &str)  -> bool{
    let entries = fs::read_dir(path);
    match entries {
        Ok(entries) => {
            for entry in entries {
                let entry = entry.expect("错误");
                let file_type = entry.file_type().expect("获取文件类型错误");
                if file_type.is_dir() {
                    find(entry.path().as_os_str(), st, target);
                }
            }
        }
        Err(err) => {
            println!("error: {}   {:?}", path.to_str().unwrap(), err);
            return false ;

        }
    }
    return true;
}
fn find_current_path(path: &OsStr, st: i32, target: &str) -> bool{
    let entries = match fs::read_dir(path){
        Ok(entries) => entries,
        Err(err) => {
            println!("error: {}   {:?}", path.to_str().unwrap(), err);
            return false;
        }
    };


    for entry in entries {
        let entry = entry.expect("错误");
        let file_type = entry.file_type().expect("获取文件类型错误");
        let file_name = entry.file_name();
        let str_file_name = file_name.to_str().expect("转换string失败").to_lowercase();

        if st == 0 && file_type.is_dir() {
            if str_file_name.contains(target.to_lowercase().as_str()) {
                output(&entry, &str_file_name);
            }
        } else if st == 1 && file_type.is_file() {
            if str_file_name.contains(target.to_lowercase().as_str()) {
                output(&entry, &str_file_name);
            }
        } else {
            continue;
        }
    }
    return true;
}
fn output(entry: &fs::DirEntry, str_file_name: &str) {
    if OPT.get().unwrap().of {
        println!("{}", entry.path().to_str().expect("获取错误"));
    } else {
        println!(
            "{}\t\t{}",
            str_file_name,
            entry.path().to_str().expect("获取错误")
        );
    }
}