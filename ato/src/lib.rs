use rayon::prelude::*;
pub mod mesh;

pub fn two_stage(_pth: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = format!("{}_sha", _pth.to_owned());
    let _ = std::fs::create_dir_all(&root)?;

    let dataset: Vec<_> = walkdir::WalkDir::new(_pth)
        .into_iter()
        .map(|e| e.unwrap().into_path())
        .filter(|c| {c.display().to_string().contains("constructionInfo")})
        .map(|f| f.parent().unwrap().to_path_buf())
        .collect();

    let _ = dataset.par_iter().for_each(|pth| {
        match mesh::Data::from(&pth) {
            Ok(d) => if let Err(e) = d.cp(root.clone().into()) {
                eprintln!("Error copying data from {}: {:?}", pth.display(), e);
            }
            Err(e) => eprintln!("Error loading data from {}: {:?}", pth.display(), e)
        };
    });
    Ok(())
}