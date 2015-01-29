extern crate getopts;
use std::old_io::{File, BufferedReader, BufferedWriter};
use std::old_io::{TcpListener, TcpStream};
use std::old_io::{Acceptor, Listener};
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
    let listener = TcpListener::bind("0.0.0.0:12345").unwrap();
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
    //let server = "172.31.99.66:12345";
    let server = "0.0.0.0:12345";
    let mut stream = TcpStream::connect(server.as_slice()).unwrap();

    let path = Path::new("./test");
    let mut file = BufferedReader::new(File::open(&path));
    loop {
      let mut buf = [0];
      match file.read(&mut buf) {
        Ok(ok) => {
          stream.write_all(&mut buf);
        },
        Err(e) => break,
      }
    }

    stream.close_write();

    let mut buf = [0];
    let _ = stream.read(&mut buf);
  }

  let closure = |&: line_str: &str| {
    print!("{}", line_str);
  };

  let file_path = match matches.opt_str("d") {
    Some(s) => read_file( s.as_slice()),
    None => (),
  };

}

fn read_file(path_str: &str){
  let path = Path::new(path_str);
  let mut file = BufferedReader::new(File::open(&path));
  loop {
    match file.read_byte() {
      Ok(byte) => println!("{}", byte),
      Err(e) => break,
    }
  }
  return;
}

fn handle_client(mut stream: TcpStream) {
  let file = File::create(&Path::new("./received")).unwrap();
  let mut writer = BufferedWriter::new(file);

  loop {
    let mut buf = [0];
    match stream.read(&mut buf) {
      Ok(ok) => {
        writer.write_all(&mut buf);
      },
      Err(e) => break,
    }
  }

  stream.write(&[1]);
}
