use std::string::{String};
use std::vec::{Vec};
use std::old_io::{File, IoResult};

pub struct FormatChunk {
  pub tag: u16,
  pub channels: u16,
  pub samplesPerSec: u32,
  pub bytesPerSec: u32,
  pub blockAlign: u16,
  pub bitsPerSample: u16,
}

pub struct FactChunk {
  pub noSamples: u32,
}

pub struct WavFileHeader {
  pub format: FormatChunk,
  pub length: u32,
  pub fact: Option<FactChunk>,
  beginning: u32,
}

impl WavFileHeader { 
  pub fn read(p: Path) -> IoResult<WavFileHeader> {
    fn read_chunk_name(f: &mut File) -> IoResult<String> {
      let mut name = [0u8; 4];
      let mut v = Vec::with_capacity(name.len());
      try!(f.read(&mut name));
      for c in name.iter() {
        v.push(*c);
      }
      Ok(String::from_utf8(v).unwrap())
    }

    fn read_fact_chunk(f: &mut File) -> IoResult<FactChunk> { 
      let chkSize = try!(f.read_le_u32());
      println!("fact {} bytes", chkSize);
      let block = FactChunk {
        noSamples: try!(f.read_le_u32()),
      }; 
      for i in 0..(chkSize-4) {
        f.read_byte();
      }
      Ok(block)
    }
    
    fn read_format_chunk(f: &mut File) -> IoResult<FormatChunk> {
      let chkSize = try!(f.read_le_u32());
      println!("format {} bytes", chkSize);
      let block = FormatChunk {
        tag: try!(f.read_le_u16()),
        channels: try!(f.read_le_u16()),
        samplesPerSec: try!(f.read_le_u32()),
        bytesPerSec: try!(f.read_le_u32()),
        blockAlign: try!(f.read_le_u16()),
        bitsPerSample: try!(f.read_le_u16()),
      };
      println!("skipping {}  bytes of the format chunk", chkSize-16);
      for i in 0..(chkSize-16) {
        f.read_byte();
      }
      Ok(block)
    }

    fn diregard_chunk(f: &mut File) -> IoResult<()> {
      let chkSize = try!(f.read_le_u32());
      println!("disregarding {} bytes", chkSize);
      for i in 0..chkSize {
        f.read_byte();
      }
      Ok(())
    }
    
    fn read_wav_chunk(f: &mut File) -> IoResult<WavFileHeader> {
      let mut formatO = None::<FormatChunk>;
      let mut factO = None::<FactChunk>;

      let mut chunk_name;
      while { 
        chunk_name = try!(read_chunk_name(f)); 
        chunk_name != "data" } { 
          match chunk_name.as_slice() {
            "fmt " => { formatO = Some(try!(read_format_chunk(f))) },
              "fact" => { factO = Some(try!(read_fact_chunk(f))) },
              _ => { println!("what's a {} chunk?!", chunk_name); try!(diregard_chunk(f)) },
          }
        }

      match formatO {
        Some(format) => {
          let len = try!(f.read_le_u32());
          let beginning = try!(f.tell()) as u32;
          println!("start of data chunk is at offset {}", beginning);
          Ok(WavFileHeader { length: len, format: format, fact: factO, beginning: beginning } )
        }
        None => panic!("no format"),
      }
    }

    let mut f = try!(File::open(&p));
    let head = try!(read_chunk_name(&mut f));
    if head == "RIFF" { 
      try!(f.read_le_u32()); // skip filesize
      try!(f.read_le_u32()); // skip 'WAVE'
      read_wav_chunk(&mut f)
    } else {
      panic!("bad head")
    }
  }
}

#[test]
fn it_works() {
}
