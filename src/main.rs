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
    filename: String,

    #[structopt(short = "a", long = "output-full", help = "输出完整地址")]
    of: bool,
}
static OPT: OnceLock<Opt> = OnceLock::new();

fn init() {
    std::thread::spawn(|| {
         OPT.get_or_init(|| {
            Opt::from_args()
        });
    }).join().unwrap();
}
fn main() {
    // 解析命令参数
    
    init();
    
    let bind = env::current_dir().expect("获取工作路径失败");
    let path_buf = bind.as_os_str();

    find(path_buf, OPT.get().unwrap().filename.as_str());
}
fn find(path: &OsStr, target: &str) {
    find_current_path(path, target);
    find_depth_path(path, target);
}
fn find_depth_path(path: &OsStr, target: &str) {
    let entries = fs::read_dir(path).expect("读取目录失败");
    for entry in entries {
        let entry = entry.expect("错误");
        let file_type = entry.file_type().expect("获取文件类型错误");
        if file_type.is_dir() {
            find(entry.path().as_os_str(), target);
        }
    }
}
fn find_current_path(path: &OsStr, target: &str) {
    let entries = fs::read_dir(path)
        .map_err(|err| format!("{:?}, {}", path, err))
        .expect("读取目录失败");
    for entry in entries {
        let entry = entry.expect("错误");
        let file_type = entry.file_type().expect("获取文件类型错误");
        if !file_type.is_file() {
            continue;
        }
        let file_name = entry.file_name();

        let str_file_name = file_name.to_str().expect("转换string失败").to_lowercase();
        if str_file_name.contains(target.to_lowercase().as_str()) {
            output(&entry, &str_file_name);
        }
    }
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
