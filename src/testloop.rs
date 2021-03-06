#[crate_id="testloop#0.1-pre"];
#[crate_type="bin"];

use std::io;
use std::io::fs;
use std::os;
use std::str;
use std::io::Process;

#[main]
fn testloop () {
   let mut latest = 0;

   let args = os::args();
   let src = expandpath(args[1].to_str());
   let test_args = args.slice_from(2);
   let absdir = absdirname(&src);

   os::setenv("RUST_LOG", "rustc=1");

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
   match fs::stat(path) {
      Ok(st) => {
         return st.modified;
      }
      Err(_err) => {
         println!("FIXME: modified has errored in stat()");
         return 0;
      }
   }
}

fn last_modified(pp: &Path) -> u64 {
   match fs::walk_dir(pp) {
      Ok(wi) => {
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
      Err(_err) => {
         println!("FIXME: last_modified has errored in walk_dir()");
         return 0;
      }
   }

}

fn modified_since(latest: u64, dir: &Path) -> (bool, u64) {
   let new_latest = last_modified(dir);
   (last_modified(dir) > latest, new_latest)
}

// run and selectively print results of `process_output`
fn run(exe: &Path, args: &[~str])
   -> Option<io::process::ProcessExit> {

   println!("Running `{}` with args: {:?}", ps(exe), args);

   match Process::output(ps(exe), args) {
      Err(_err) => {
         println!("Failed to run `{}` with args: {:?}", ps(exe), args);
         return None;
      }
      Ok(output) => {
         let out: &str = match str::from_utf8(output.output) {
                           Some(o) => o,
                           None => ""
                         };
         let err: &str = match str::from_utf8(output.error) {
                           Some(o) => o,
                           None => ""
                         };
         let stat: io::process::ProcessExit = output.status;

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
   match fs::stat(path) {
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
fn request_build(cratepath: &Path, test_args: &[~str]) {
   let test_bin = &expandpath("./.tests_in_loop.exe");

   if !is_file(cratepath) {
      println!("ERROR: crate to test is missing: {}", ps(cratepath));
      os::set_exit_status(1);
   } else {
      // cleanup
      if is_file(test_bin) {
         let res = fs::unlink(test_bin);
         match res {
            Ok(_) => (),
            Err(_) => io::println("<<<< error: couldn't cleanup! >>>>"),
         }
      }
      
      // build
      io::println("");
      io::println("<<<< building tests >>>>");
      run(&Path::new("/usr/local/bin/rustc"),
         [~"-o", ps(test_bin),
          ~"--test", ps(cratepath),
          ~"--opt-level", ~"0"]);

      // run
      io::println("");
      if is_file(test_bin) {
         io::println("<<<< running tests >>>>");
         run(test_bin, test_args);
      }
      else {
         io::println("<<<< NOT running tests >>>>");
      }
   }
}

