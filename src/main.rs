
mod readfile;
mod tracefile;

fn main() -> std::io::Result<()> {
  let trace_dirs = std::env::args().collect::<Vec<String>>();
  let mut trace_threads = vec![];
  trace_threads.reserve(trace_dirs.len());

  for (i, dir) in trace_dirs.into_iter().enumerate() {
    trace_threads.push(std::thread::spawn(move || -> std::io::Result<()> {
      let mut trace_file = tracefile::TraceFile::new(dir)?;
    
      loop {
        trace_file.trace_tree();
        trace_file.gen_tree()?;
        // for ubuntu/debian, there is a bug with .goutputstream-XXXXXX files clogging up the file tree
        std::thread::sleep(std::time::Duration::from_millis(75)); // delay long enough for temp files to disappear
      }

      Ok(())
    }));
  }

  for thread in trace_threads {
    thread.join();
  }
  
  Ok(())
}

// TODO:
// - ...