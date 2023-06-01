use std::env;
use std::path::Path;
use std::fs;
fn main() {
    let mut rblxpath = String::new();
    if let Some(lad) = env::var("LOCALAPPDATA").ok() {
        let temprbx_lad = format!("{}\\Roblox",lad);
        if Path::new(&temprbx_lad).is_dir() {
            rblxpath = temprbx_lad;
        }
    } else if let Some(pfx86) = env::var("ProgramFiles(x86)").ok() {
        let temprbx_pfx86 = format!("{}\\Roblox",pfx86);
        if Path::new(&temprbx_pfx86).is_dir() {
            rblxpath = temprbx_pfx86;
        }
    }
    if rblxpath != "" {
        println!("Found path: {}",rblxpath);
    } else {
        println!("Roblox not found (Die)");
        return;
    }
    let versions = format!("{}\\Versions\\",rblxpath);
    for folder in fs::read_dir(versions).unwrap() {
        let cur_folder = folder.unwrap().path().display().to_string();
        if cur_folder.to_lowercase().contains("version") {
            let executable = format!("{}\\RobloxPlayerBeta.exe",cur_folder);
            if Path::new(&executable).is_file() {
                let cs_folder = format!("{}\\ClientSettings",cur_folder);
                if !Path::new(&cs_folder).is_dir() {
                    fs::create_dir(&cs_folder).ok();
                    println!("Created ClientSettings folder in {}",cur_folder);
                } else {
                    println!("ClientSettings folder already exists in {}",cur_folder);
                }
                let cas_json = format!("{}\\ClientAppSettings.json",cs_folder);
                if !Path::new(&cas_json).is_file() {
                    fs::write(cas_json, "{\"FFlagHandleAltEnterFullscreenManually\":\"False\",\"DFIntTaskSchedulerTargetFps\":5588562}").ok();
                    println!("Wrote ClientAppSettings.json to {}",cur_folder)
                } else {
                    println!("ClientAppSettings.json already exists in {}",cur_folder)
                }
            }
        }
    }
}
