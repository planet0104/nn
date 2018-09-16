use std::os::raw::{c_char};
use std::ffi::{CString, CStr};
#[macro_use] extern crate log;
extern crate android_logger;
use std::fs;
use std::io;
use std::fs::File;
use std::io::prelude::*;
extern crate zip;
use log::Level;
use android_logger::Filter;

// 获取包名
fn get_package_name() -> String{
    match File::open("/proc/self/cmdline"){
        Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents){
                Ok(_count) => String::from(contents.trim_matches(char::from(0))),
                Err(err) =>{
                    trace!("get_package_name>>{:?}", err);
                    String::new()
                }
            }
        }
        Err(err) => {
            trace!("get_package_name>>{:?}", err);
            String::new()
        }
    }
}

//获取apk路径
fn get_apk_file_path() -> String{
    let package = get_package_name();
    match File::open("/proc/self/maps"){
        Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents){
                Ok(_count) =>{
                    let mut apk_file = "";
                    for line in contents.lines(){
                        if line.contains(".apk") && line.contains(&package){
                            for p in line.split(" "){
                                if p.ends_with("apk"){
                                    apk_file = p;
                                    break;
                                }
                            }
                        }
                    }
                    String::from(apk_file)
                }
                Err(err) =>{
                    error!("get_package_name>>{:?}", err);
                    String::new()
                }
            }
        }
        Err(err) => {
            error!("get_apk_file_path>>{:?}", err);
            String::new()
        }
    }
}

fn get_manifest_mf() -> String{
        match File::open(get_apk_file_path()){
            Ok(file) =>{
                trace!("开始解压文件..");
                let mut archive = zip::ZipArchive::new(file).unwrap();
                for i in 0..archive.len() {
                    let mut file = archive.by_index(i).unwrap();
                    let outpath = file.sanitized_name();
                    if outpath.to_str().unwrap() == "META-INF/MANIFEST.MF"{
                        let mut manifest_mf = String::new();
                        file.read_to_string(&mut manifest_mf).unwrap();
                        return manifest_mf;
                    }
                }
            }
            Err(err) => trace!("文件打开失败{:?}", err)
        }
        String::new()
}

fn get_digest(manifest_mf: &String, file: &str) -> String{
    let mut next = false;
    for line in manifest_mf.lines(){
        if next{
            return line.replace("SHA-256-Digest: ", "");
        }
        if line.ends_with(file){
            next = true;
        }
    }
    String::new()
}

#[no_mangle]
pub extern fn rust_greeting(_to: *const c_char) -> *mut c_char {
     android_logger::init_once(
        Filter::default()
            .with_min_level(Level::Trace)
    );
    let manifest_mf = get_manifest_mf();
    let dex_digest = get_digest(&manifest_mf, "classes.dex");
    if dex_digest == "lQN8V7e2a2bNfW38lTJDx2qEtMjsC0U0XBLkKRmxJ1s="{
        trace!("dex摘要验证成功!");
    }else{
        error!("dex摘要错误!");
    }
    trace!("dex_digest={}", dex_digest);
    CString::new(dex_digest).unwrap().into_raw()
}

/// Expose the JNI interface for android below
#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use super::*;
    use self::jni::JNIEnv;
    use self::jni::objects::{JClass, JString};
    use self::jni::sys::{jstring};

    #[no_mangle]
    pub unsafe extern fn Java_com_mozilla_greetings_RustGreetings_greeting(env: JNIEnv, _: JClass, java_pattern: JString) -> jstring {
        // Our Java companion code might pass-in "world" as a string, hence the name.
        let world = rust_greeting(env.get_string(java_pattern).expect("invalid pattern string").as_ptr());
        // Retake pointer so that we can use it below and allow memory to be freed when it goes out of scope.
        let world_ptr = CString::from_raw(world);
        let output = env.new_string(world_ptr.to_str().unwrap()).expect("Couldn't create java string!");

        output.into_inner()
    }
}

