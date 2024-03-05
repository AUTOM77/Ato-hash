use rayon::prelude::*;
use walkdir::WalkDir;

pub fn _hash(r:  &str, p: &std::path::Path, e: &str) -> Result<(), Box<dyn std::error::Error>>{
    let _dir = p.with_file_name("");
    let _num = _dir.file_name().unwrap().to_str().unwrap();
    let _in = p.with_file_name(format!("{}_{}_{}", _num, "total", e));
    let _up = p.with_file_name(format!("{}_{}_{}", _num, "up", e));
    let _low = p.with_file_name(format!("{}_{}_{}", _num, "low", e));
    if _in.is_file() && _up.is_file() && _low.is_file() {
        let contents = std::fs::read(&_in).expect("Unable to read the file");
        let digest = md5::compute(&contents);
        let md5_dir = format!("{}/{:x}", r, digest);

        let _ = std::fs::create_dir_all(&md5_dir)?;
        let _ = std::fs::copy(_in, format!("{}/in.stl", &md5_dir))?;
        let _ = std::fs::copy(_up, format!("{}/up.stl", &md5_dir))?;
        let _ = std::fs::copy(_low, format!("{}/low.stl", &md5_dir))?;
        let _ = std::fs::copy(p, format!("{}/gt.stl", &md5_dir))?;
    }
    else {
        println!("{}", p.with_file_name("").display().to_string());
    }
    Ok(())
}

pub fn process(_pth: &str, _e:&str) -> Result<(), Box<dyn std::error::Error>> {
    let root = format!("{}_sha", _pth.to_owned());
    let _ = std::fs::create_dir_all(&root)?;

    let pdd: Vec<std::path::PathBuf> = 
        WalkDir::new(_pth).into_iter()
        .map(|e| e.unwrap().into_path())
        .filter(|c| {
            c.display().to_string().contains("gt")
        })
        .collect();

    pdd.par_iter().for_each(|p| {
        let _ = _hash(&root, &p, _e);
    });

    Ok(())
}