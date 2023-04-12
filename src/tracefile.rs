
use crate::readfile;

pub struct TraceFile {
  path: std::path::PathBuf,
  metadata: std::fs::Metadata,
  is_dir: bool,
  hash: Option<String>,
  files: Vec<TraceFile>,
}

impl TraceFile {
  pub fn new(file_path: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
    let handle = std::fs::File::open(&file_path)?;
    
    let path = file_path.as_ref().to_path_buf();
    let metadata = handle.metadata()?;
    let is_dir = metadata.is_dir();
    let hash = if !is_dir { Some(readfile::encode_file(&path)?) } else { None };
    let files = vec![];

    Ok( Self { path, metadata, is_dir, hash, files } )
  }

  pub fn gen_tree(&mut self) -> std::io::Result<()> {
    // could technically have this as a static function, but this is convenient to reuse old objects
    *self = TraceFile::new(&self.path)?; // refresh current directory
    self.gen_tree_fn_for_t()
  }

  // fn-for-t
  fn gen_tree_fn_for_t(&mut self) -> std::io::Result<()> {
    if self.is_dir {
      self.gen_tree_fn_for_lot()?;
    } else {
      // self.files = vec![];
    }

    Ok(())
  }

  // fn-for-lot
  fn gen_tree_fn_for_lot(&mut self) -> std::io::Result<()> {
    for file in std::fs::read_dir(&self.path)? {
      let mut new_file = TraceFile::new(&file?.path());
      if new_file.is_err() { // unable to create a new file with that path; invalid path
        println!("Invalid file path for new_file in gen_tree_fn_for_lot");
        continue;
      }

      let mut new_file = new_file.unwrap();
      new_file.gen_tree_fn_for_t()?;
      self.files.push(new_file);
    }

    Ok(())
  }

  pub fn trace_tree(&self) -> Vec<std::path::PathBuf> {
    let mut result = vec![];

    self.trace_tree_fn_for_t(&mut result);

    result
  }

  fn trace_tree_fn_for_t(&self, rsf: &mut Vec<std::path::PathBuf>) {
    if let Some(ref hash) = self.hash {
      assert!(!self.is_dir);
      let new_hash = readfile::encode_file(&self.path).unwrap_or(String::from(""));
      if new_hash.cmp(hash) != std::cmp::Ordering::Equal {
        println!("{} modified!", &self.path.display());
      }
    }

    self.trace_tree_fn_for_lot(rsf);
  }

  fn trace_tree_fn_for_lot(&self, rsf: &mut Vec<std::path::PathBuf>) {
    for tree in &self.files {
      tree.trace_tree_fn_for_t(rsf);
    }
  }
}