//This is gonna be used to compress a directory into a tarball
//tarball is esentially a single file that groups all the files together with their metadata


//this code uses the flate2 compresssion/decompression library



use std::fs::File;
use flate2::Compresssion;
use flate2::write::GZEncoder;



fn main() -> Result<(), std::io::Error> {

    let gz =  File::create("archive.tar.gz");
    let encoder = GZEncoder::new(gz,Compresssion::default());
    let mut tar  =  tar::Builder::new(enc);



    //adding all files to the current directory to current backup

    tar.append_dir_all(".","current_backup")?;
    Ok(());

}
