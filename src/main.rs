extern crate wav;

use wav::WavFile;

fn main() {
  let args = std::os::args(); 

  if args.len() != 2 {
    println!("fale");
  }
  else {
    let p = Path::new(&args[1]); 
    match WavFile::read(p) {
      Ok(wf) => { 
        println!("opened");
        println!("{} channels of {} bits at {} samples per second",
          wf.format.channels,
          wf.format.bitsPerSample,
          wf.format.samplesPerSec
        );
        println!("{} samples long", wf.fact.noSamples);
      },
      Err(e)=> println!("{}",e)
    } 
  }
}

