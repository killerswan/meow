extern mod extra;
use std::os;
use std::io;

#[main]
fn testloop () {
   let args = os::args();

   let mut has_changed = false;
   let mut latest = last_modified();

   let timer = io::timer::Timer::new();

   while true {
      let (has_changed, latest) = modified_since(latest);

      if has_changed {
         request_build(args)
      }

      io::timer::sleep(200);
   }

   os::set_exit_status(0);
}

fn last_modified() -> u64 {
   return 0;
}

fn modified_since(last_modified: u64) -> (bool, u64) {
   return (true, 0);
}

fn run(exe: &str, args: &[~str]) -> std::io::process::ProcessExit {
   let ps: std::run::ProcessOutput = std::run::process_output(exe, args);

   let out: &str = std::str::from_utf8(ps.output);
   let err: &str = std::str::from_utf8(ps.error);
   let stat: std::io::process::ProcessExit = ps.status;

   if out.len() > 0 {
      println!("STDOUT:\n{}", out);
   }
   if err.len() > 0 {
      println!("STDERR:\n{}", err);
   }
   if stat != std::io::process::ExitStatus(0) {
      println!("STATUS: {:?}", stat);
   }
   return stat;
}

fn exists(path: &str) -> bool {
   let p = Path::new(path);
   match io::result(|| std::io::fs::stat(&p)) {
       Ok(stat) => { return true; }
       Err(e)   => { return false; }
   }
}

fn is_file(path: &str) -> bool {
   let p = Path::new(path);
   match io::result(|| std::io::fs::stat(&p)) {
      Ok(stat) => {
         match stat.kind {
            std::io::TypeFile => { return true; }
            _                 => { return false; }
         }
      }
      Err(e)   => { return false; }
   }
}

fn request_build(args: &[~str]) {
   let crate    : ~str = args[1];
   let test_bin : ~str = ~"./loop_test";
   let test_args: &[~str] = args.slice_from(2);

   if is_file(crate) {
      // cleanup
      if is_file(test_bin) {
         let p = Path::new(test_bin);
         std::io::fs::unlink(&p);
      }
      
      // build
      println("<<<< building tests >>>>");
      run("/usr/local/bin/rustc", [~"-o", test_bin,
                                   ~"--test", crate,
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



