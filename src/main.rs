extern crate wav;

use wav::WavFileHeader;

fn main() {
  let args = std::os::args(); 

  if args.len() != 2 {
    println!("fale");
  }
  else {
    let p = Path::new(&args[1]); 
    match WavFileHeader::read(p) {
      Ok(wf) => { 
        println!("opened");
        println!("{} channels of {} bits at {} samples per second",
            wf.format.channels,
            wf.format.bitsPerSample,
            wf.format.samplesPerSec
            );
        println!("{} bytes, ie. {} samples", wf.length, ((wf.length)/((wf.format.bitsPerSample/8) as u32))/wf.format.channels as u32);
        match wf.fact {
          Some(fact) => { println!("{} samples long", fact.noSamples); },
            None => { println!("no fact chunk"); },
        }
      },
        Err(e)=> println!("{}",e)
    } 
  }
}

