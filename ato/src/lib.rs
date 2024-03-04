use walkdir::WalkDir;
use std::fs;

pub fn process(_pth: &str, _e:&str) -> Result<(), Box<dyn std::error::Error>> {
    let _root = format!("{}_md5", _pth.to_owned());
    let _ = fs::create_dir_all(&_root)?;

    for entry in WalkDir::new(_pth).into_iter().filter_map(|e| e.ok()) {
        let pth = entry.path();
        if pth.display().to_string().contains("gt") {
            let _dir = pth.with_file_name("");
            let _num = _dir.file_name().unwrap().to_str().unwrap();
            let _in = pth.with_file_name(format!("{}_{}_{}", _num, "total", _e));
            let _up = pth.with_file_name(format!("{}_{}_{}", _num, "up", _e));
            let _low = pth.with_file_name(format!("{}_{}_{}", _num, "low", _e));
            if _in.is_file() && _up.is_file() && _low.is_file() {
            let contents = fs::read(&_in).expect("Unable to read the file");
            let digest = md5::compute(&contents);
            let md5_dir = format!("{}/{:x}", _root, digest);
                let _ = fs::create_dir_all(&md5_dir)?;
                let _ = fs::copy(_in, format!("{}/in.stl", &md5_dir))?;
                let _ = fs::copy(_up, format!("{}/up.stl", &md5_dir))?;
                let _ = fs::copy(_low, format!("{}/low.stl", &md5_dir))?;
                let _ = fs::copy(pth, format!("{}/gt.stl", &md5_dir))?;
            }
            else {
                println!("{:#?}", pth);
            }

        }
    }
    Ok(())
}