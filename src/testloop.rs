extern mod rustc;

use std::io;
use std::io::fs;
use std::os;
use std::run;
use std::str;

#[main]
fn testloop () {
   let mut latest = 0;

   let args = os::args();
   let src = expandpath(args[1].to_str());
   let test_args = args.slice_from(2);
   let absdir = absdirname(&src);

   loop {
      let (has_changed, latest_) = modified_since(latest, &absdir);
      latest = latest_;

      if has_changed {
         request_build(&src, test_args);
      }

      io::timer::sleep(200);
   }
}

fn ps(path: &Path) -> ~str {
   path.as_str().unwrap().to_str()
}

fn expandpath(path: &str) -> Path {
   let pp = Path::new(path);

   if pp.is_relative() {
      let cwd = os::getcwd();
      let ppp = cwd.join(pp);
      return ppp;
   } else {
      return pp;
   }
}

fn absdirname(path: &Path) -> Path {
   path.dir_path()
}

fn modified(path: &Path) -> u64 {
   let st = fs::stat(path);
   return st.modified;
}

fn last_modified(pp: &Path) -> u64 {
   let wi: fs::WalkIterator = fs::walk_dir(pp);

   let mut rs_files = wi.filter(|path| {
      match path.extension_str() {
         None      => { false }
         Some(ext) => { ext == "rs" }
      }
   });

   let latest: Option<Path> =
      rs_files.max_by(|p| modified(p));

   match latest {
      None       => { 0 }
      Some(path) => { modified(&path) }
   }
}

fn modified_since(latest: u64, dir: &Path) -> (bool, u64) {
   let new_latest = last_modified(dir);
   (last_modified(dir) > latest, new_latest)
}

// run and selectively print results of `process_output`
fn run(exe: &Path, args: &[~str])
   -> Option<io::process::ProcessExit> {

   match run::process_output(ps(exe), args) {
      None => {
         println!("Failed to run `{}` with args: {:?}", ps(exe), args);
         return None;
      }
      Some(ps) => {
         let out: &str = str::from_utf8(ps.output);
         let err: &str = str::from_utf8(ps.error);
         let stat: io::process::ProcessExit = ps.status;

         if stat != io::process::ExitStatus(0) {
            println!("STATUS: {:?}", stat);
         }
         if err.len() > 0 {
            println!("STDERR:\n{}", err);
         }
         if out.len() > 0 {
            println!("STDOUT:\n{}", out);
         }
         return Some(stat);
      }
   }
}

// test if a path is a file (and not missing or something else)
fn is_file(path: &Path) -> bool {
   match io::result(|| fs::stat(path)) {
      Ok(stat) => {
         match stat.kind {
            io::TypeFile => { return true; }
            _            => { return false; }
         }
      }
      Err(_err) => {
         //println!("Error in `stat` on `{}`: {:?}", ps(path), err);
         return false;
      }
   }
}

// if possible, build and run the given crate (first arg)
fn request_build(crate: &Path, test_args: &[~str]) {
   let test_bin = &Path::new("./.tests_in_loop.exe");

   if !is_file(crate) {
      println!("ERROR: crate to test is missing: {}", ps(crate));
      os::set_exit_status(1);
   } else {
      // cleanup
      if is_file(test_bin) {
         fs::unlink(test_bin);
      }
      
      // build
      println("<<<< building tests >>>>");
      rustc::main_args(
         [os::args()[0],
          ~"-o", ps(test_bin),
          ~"--test", ps(crate),
          ~"--allow", ~"dead_code",
          ~"--opt-level", ~"0"]);

      // run
      if is_file(test_bin) {
         println("<<<< running tests >>>>");
         run(test_bin, test_args);
      }
      else {
         println("<<<< NOT running tests >>>>");
      }
   }
}

