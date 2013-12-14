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
      let (has_changed, latest) = sources_modified_since(latest);

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

fn sources_modified_since(last_modified: u64) -> (bool, u64) {
   return (true, 0);
}

fn run(exe: &str, args: &[~str]) {
   let ps: std::run::ProcessOutput = std::run::process_output(exe, args);

   let out: &str = std::str::from_utf8(ps.output);
   let err: &str = std::str::from_utf8(ps.error);
   let stat: std::io::process::ProcessExit = ps.status;

   println(out);
   println(err);
   println!("{:?}", stat);
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
   if is_file(args[1]) {
      if is_file("./loop_test") {
         println(">>>> cleanup <<<<");
         let p = Path::new("./loop_test");
         std::io::fs::unlink(&p);
      }
      
      let args_ = [~"-o", ~"loop_test",
                   ~"--test", args[1].to_str(),
                   ~"--allow", ~"dead_code",
                   ~"--opt-level", ~"0"]; 

      println(">>>> compiling <<<<");
      run("/usr/local/bin/rustc", args_);

      if is_file("./loop_test") {
         println(">>>> testing <<<<");
         run("./loop_test", args.tail().tail()); 
      }
   }
}



