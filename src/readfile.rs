use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};

fn sha256_digest(mut reader: impl std::io::Read) -> std::io::Result<Digest> {
  let mut context = Context::new(&SHA256);
  let mut buffer = [0; 1024];

  loop {
    let count = reader.read(&mut buffer)?;
    if count == 0 {
      break;
    }
    context.update(&buffer[..count]);
  }

  Ok(context.finish())
}

pub fn encode_file(path: impl AsRef<std::path::Path>) -> std::io::Result<String> {
  let input = std::fs::File::open(path)?;
  let reader = std::io::BufReader::new(input);
  let digest = sha256_digest(reader)?;

  Ok(HEXUPPER.encode(digest.as_ref()))
}
