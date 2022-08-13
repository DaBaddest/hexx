use std::io::Read;
use std::process;
use std::fs;
use std::fmt::Write;
use std::env;

struct FileInfo {
  filename     : String,
  start        : usize,
  end          : usize,
  // patch_offset : usize,
}

const HEX_CHARSET: &'static str = "0123456789ABCDEF";


fn color_code_hex(elem: u8) -> String {
  return match elem{
    0         => format!("\x1b[31m{:02x}\x1b[0m", elem),
    1..=32    => format!("\x1b[33m{:02x}\x1b[0m", elem),
    48..=57   => format!("\x1b[35m{:02x}\x1b[0m", elem),
    127..=255 => format!("\x1b[34m{:02x}\x1b[0m", elem),
    _         => format!("\x1b[32m{:02x}\x1b[0m", elem),
  };
}

fn color_code_char(elem: u8) -> String {
  return match elem{
    0         => format!("\x1b[31m⋄\x1b[0m"),
    1..=32    => format!("\x1b[33m×\x1b[0m"),
    48..=57   => format!("\x1b[35m{}\x1b[0m", elem as char),
    127..=255 => format!("\x1b[34m•\x1b[0m"),
    _         => format!("\x1b[32m{}\x1b[0m", elem as char),
  };
}

// Read the file contents and return them
fn read_file(filename: String) -> Vec<u8> {
  let mut contents: Vec<u8> = Vec::new();

  let mut fp = fs::File::open(filename)
    .expect("Failed in opening file");

  if fp.read_to_end(&mut contents).is_err(){
    print_error_and_exit("Failed while reading file contents".to_string());
  }

  return contents;
}

// Print the usage and exit
fn print_usage() {
  let mut usage: String = String::new();
  
  write!(usage, "A simple hex viewer written in Rust\n").unwrap();
  write!(usage, "Usage: hexx [options] filename\n\n").unwrap();
  write!(usage, "Options:\n").unwrap();
  write!(usage,
    "{}{:>8}{}",
    "-s, --start:\n",
    "",
    "Specify the START index, processing of the file will start from here\n\n")
    .unwrap();


  write!(usage,
    "{}{:>8}{}", 
    "-e, --end:\n",
    "",
    "Specify the END index, processing of the file will end here\n\n")
    .unwrap();


  write!(usage,
    "{}{:>8}{}", 
    "-h, --help:\n",
    "",
    "Print this help message\n\n")
    .unwrap();

  write!(usage,
    "{}{:>8}{}{:>8}{}",
    "Examples:\n",
    "",
    "hexx -s 100 filename.bin\n",
    "",
    "hexx --start 10 --end 0x40 filename.bin",
    )
  .unwrap();


  println!("{}", usage);

  process::exit(1);
}

fn print_error_and_exit(s: String) {
  println!("\x1b[31mERROR: {}\x1b[0m", s);
  process::exit(1);
}


// Converts a str to int. Also supports hexadecimal numbers
fn atoi(num: &str) -> usize {
  let mut val: usize = usize::MAX;

  // Hexadecimal number
  if num.trim().starts_with("0x") {
    // removing the 0x from start
    val = usize::from_str_radix(num.trim_start_matches("0x"), 16).unwrap();
  }
  
  // Decimal number
  // Should be a better way to do this
  else if num.trim().parse::<usize>().is_ok() {
    val = num.trim().parse().unwrap();
  }

  return val;
}

fn parse_arguments() -> FileInfo {
  let mut file_options = FileInfo {
    filename : String::new(),
    start    : 0,
    end      : 0,
  };

  let argv: Vec<String> = env::args().collect();
  let argc: usize = argv.len();

  if argc == 1 {
    print_usage();
  }

  let mut i: usize = 1;
  while i < argc - 1 {
    match argv[i].as_str() {
      // If start or end option is passed as command line, it is expected that
      // the next parameter would be integer
      "-s" | "--start" => {
        i += 1;

        // If incomplete commandline parameters passed
        if i >= argc - 1 {
          println!("Error while parsing commandline argument");
          process::exit(1);
        }

        file_options.start = atoi(argv[i].as_str());
        if file_options.start == usize::MAX {
          print_error_and_exit(
            format!("Invalid argument given for start: {}", argv[i]));
        }
      }

      "-e" | "--end" => {
        i += 1;

        // If incomplete commandline parameters passed
        if i >= argc - 1 {
          println!("Error while parsing commandline argument");
          process::exit(1);
        }

        file_options.end = atoi(argv[i].as_str());
        if file_options.end == usize::MAX {
          print_error_and_exit(
            format!("Invalid argument given for end: {}", argv[i]));
        }

      }

      "-h" | "--help" => {
        print_usage();
      }

      _ => {
        println!("Invalid option {}", argv[i]);
      }
    }

    i += 1;

  }

  // Considering the last argument to be filename
  file_options.filename = String::from(&argv[i]);

  return file_options;
}

// Print the header of the table
fn print_header() {
    let mut s: String = String::new();
    
    write!(s, "┌────────┬").unwrap();
    write!(s, "─────────────────────────┬─").unwrap();
    write!(s, "────────────────────────┬─").unwrap();
    write!(s, "───────").unwrap();

    // Modify this for using in --no-header option
    write!(s, "─").unwrap();
    // write!(s, "┬").unwrap();

    write!(s, "────────┐").unwrap();

    println!("{}", s);

    s = "".to_string();

    write!(s, "{}  {}  {0}", "│", "hexx").unwrap();

    for i in HEX_CHARSET.chars() {
      write!(s, "  {}", i).unwrap();

      // 7 marks the mid of the table
      if 0x37 == i as u8  {
        write!(s, " ┆").unwrap();
      }
    }

    write!(s, " {:7}{:11}{0}", "│", "Ascii").unwrap();

    println!("{}", s);
}

// Print the footer, to complete the rectangular border
fn print_footer() {
  let mut footer: String = String::new();
  write!(footer, "└────────").unwrap();
  write!(footer, "┴─────────────────────────").unwrap();
  write!(footer, "┴─────────────────────────").unwrap();
  write!(footer, "┴────────").unwrap();
  write!(footer, "┴────────┘").unwrap();
  
  println!("{}", footer);
}


fn main() {
  // XXX: Perform validations on the parameters passed
  let mut fileinfo: FileInfo = parse_arguments();

  let data = read_file(fileinfo.filename);
  if fileinfo.end == 0 || fileinfo.end > data.len() {
    fileinfo.end = data.len()
  }


  print_header();
  

  let mut i = fileinfo.start - (fileinfo.start % 16);
  while i < fileinfo.end {
    let mut s: String = String::new();

    write!(s, "│\x1b[36m{:08x}\x1b[0m│", i).unwrap();

    // Hex view implementation
    for j in 0..16 {
      if i + j >= fileinfo.end {
        write!(s, "   ").unwrap();
      }
      else{
        if i + j < fileinfo.start {
          write!(s, "   ").unwrap();
        }
        else {
          write!(s, " {}", color_code_hex(data[i + j])).unwrap();
        }
      }
      if 7 == j {
        write!(s, " ┆").unwrap();
      }
    }
    write!(s, " │").unwrap();

    // Ascii view implementation
    for j in 0..16 {
      // To place spaces at the end
      if (i + j) >= fileinfo.end {
        write!(s, " ").unwrap();
      }

      else {
        if i + j < fileinfo.start {
          write!(s, " ").unwrap();
        }
        else {
          write!(s, "{}", color_code_char(data[i + j])).unwrap();
        }
      }

      if 7 == j {
        write!(s, "┆").unwrap();
      }
    }
    write!(s, "│").unwrap();

    println!("{}", s);

    i += 16;
  }

  print_footer();
}
