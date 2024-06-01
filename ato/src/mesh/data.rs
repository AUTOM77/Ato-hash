use std::path::PathBuf;
use super::Raw;

#[derive(Debug)]
pub struct Data {
    l: Raw,
    r: Raw,
    m: Raw,
} // l -> low, r -> up, m -> gt

impl Data {
    pub fn new(l: Raw, r: Raw, m: Raw) -> Self {
        Self { l, r, m } 
    }

    pub fn from(dir: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let l = Raw::from(dir, "stl", "low")?;
        let r = Raw::from(dir, "stl", "up")?;
        let m = Raw::from(dir, "stl", "gt")?;

        Ok(Self::new(l, r, m))
    }

    pub fn cp(&self, to: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let root = to.join(self.hash());
        let _ = std::fs::create_dir_all(&root);

        let _lp = root.join(format!("low.{}",self.l.ext()));
        let _rp = root.join(format!("up.{}",self.r.ext()));
        let _mp = root.join(format!("gt.{}",self.m.ext()));

        let _ = self.l.cp(_lp);
        let _ = self.r.cp(_rp);
        let _ = self.m.cp(_mp);
        Ok(())
    }

    pub fn hash(&self) -> String {
        let _lh = self.l.hash();
        let _rh = self.r.hash();

        let lh: u128 = u128::from_str_radix(&_lh, 16).unwrap();
        let rh: u128 = u128::from_str_radix(&_rh, 16).unwrap();

        format!("{:032x}", lh+rh)
    }
}