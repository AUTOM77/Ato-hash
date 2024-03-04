
use walkdir::WalkDir;
use tokio::{fs, task};
use md5::{Digest, Md5};
use std::path;

pub async fn async_hash(r:  &str, p: &path::Path, e: &str) -> Result<(), Box<dyn std::error::Error>>{
    let mut hasher = Md5::new();

    let _dir = p.with_file_name("");
    let _num = _dir.file_name().unwrap().to_str().unwrap();
    let _in = p.with_file_name(format!("{}_{}_{}", _num, "total", e));
    let _up = p.with_file_name(format!("{}_{}_{}", _num, "up", e));
    let _low = p.with_file_name(format!("{}_{}_{}", _num, "low", e));
    if _in.is_file() && _up.is_file() && _low.is_file() {
        let contents = fs::read(&_in).await?;
        let _ = hasher.update(&contents);
        let digest = hasher.finalize();
        let md5_dir = format!("{}/{:x}", r, digest);

        let _ = fs::create_dir_all(&md5_dir).await?;
        let _ = fs::copy(_in, format!("{}/in.stl", &md5_dir)).await?;
        let _ = fs::copy(_up, format!("{}/up.stl", &md5_dir)).await?;
        let _ = fs::copy(_low, format!("{}/low.stl", &md5_dir)).await?;
        let _ = fs::copy(p, format!("{}/gt.stl", &md5_dir)).await?;
    }
    else {
        println!("{:#?}", p);
    }
    Ok(())
}

pub async fn process(_pth: &str, e:&str) -> Result<(), Box<dyn std::error::Error>> {
    let root = format!("{}_sha", _pth.to_owned());
    let _ = fs::create_dir_all(&root).await?;
    let mut pool = vec![];

    let pdd: Vec<path::PathBuf> = 
        WalkDir::new(_pth).into_iter()
        .map(|e| e.unwrap().into_path())
        .filter(|c| {
            c.display().to_string().contains("gt")
        })
        .collect();

    for p in pdd {
        let _root = root.clone();
        let _e = e.to_string();
        pool.push(task::spawn(async move {
            let _ = async_hash(&_root, &p, &_e).await;
        }));
    }

    for task in pool {
        task.await?;
    }

    Ok(())
}