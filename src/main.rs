extern crate getopts;
use std::io::{File, BufferedReader};
use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};
use std::thread::Thread;
use std::os;

fn main() {
  let args: Vec<String> = os::args();
  let ref program = args[0];

  let opts = [
    getopts::optflag("s", "server", "run as server"),
    getopts::optflag("c", "client", "run as client"),
    getopts::optopt("p", "port",    "set using port", "VALUE"),
    getopts::optopt("d", "debug", "run as client", "VALUE"),
  ];

  let matches = match getopts::getopts(args.tail(), &opts) {
    Ok(m) => m,
    Err(f) => {
      println!("{}", f);
      os::set_exit_status(1);
      return;
    }
  };

  let c = match matches.opt_str("p") {
    Some(s) => s,
    None => String::from_str(""),
  };

  println!("{}", c);

  // server
  if matches.opt_present("s") {
    println!("server");
    let listener = TcpListener::bind("0.0.0.0:80").unwrap();
    let mut acceptor = listener.listen().unwrap();

    for stream in acceptor.incoming() {
      match stream {
        Err(e) => {},
        Ok(stream) => {
          Thread::spawn(move||{
            handle_client(stream);
          });
        }
      }
    }

    drop(acceptor);
  }

  if matches.opt_present("c") {
    println!("client");
    let mut stream = TcpStream::connect("127.0.0.1:80").unwrap();

    let _ = stream.write_str("Hello");
    stream.close_write();

    let mut buf = [0];
    let _ = stream.read(&mut buf);
  }

  let closure = |&: line_str: &str| {
    print!("{}", line_str);
  };

  let file_path = match matches.opt_str("d") {
    Some(s) => read_file( s.as_slice(), closure ),
    None => (),
  };

}

fn read_file<F: Fn(&str)>(path_str: &str, f: F){
  let path = Path::new(path_str);
  let mut file = BufferedReader::new(File::open(&path));
  for line in file.lines() {
    f(line.unwrap().as_slice());
  }
  return ();
}

fn handle_client(mut stream: TcpStream) {
  let req = match stream.read_to_string() {
    Ok(s) => s,
    Err(f) => {
      println!("{}", f);
      return;
    }
  };
  stream.write(&[1]);
  println!("=> {}", req);
}
