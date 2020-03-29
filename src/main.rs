use std;
use std::io::prelude::*;
use std::fs;
use zip::write::FileOptions;
use indicatif::ProgressBar;

fn archiver(dirpath: &str) -> zip::result::ZipResult<()>
{
    // Make a target zip file
    let zip_filename = dirpath.to_string() + ".zip";
    let file = std::fs::File::create(&zip_filename).unwrap(); 
    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored).unix_permissions(0o755);

    for filepath in fs::read_dir(dirpath).unwrap()
    {
        let filepath_data = filepath.unwrap(); 

        // Open target file to read the data
        let filename = filepath_data.path().to_string_lossy().to_string();
        let mut file = fs::File::open(&filename).unwrap();
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).unwrap();
    
        // Define file name for writing into the zip file
        let filename = filepath_data.file_name().to_string_lossy().to_string();
        zip.start_file(filename, options)?;
        zip.write_all(&contents)?;
    }
    zip.finish()?;
    Ok(())
}

fn execute() -> i32 
{
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <directory name>", args[0]);
        return 1;
    }

    let bar = ProgressBar::new(args[1..].len() as u64);
    for dirpath in &args[1..] // the first argument is the .exe file name
    {   
        if fs::metadata(dirpath).unwrap().is_dir()
        {
            match archiver(dirpath)
            {
                Ok(_) => println!("File written to {}", dirpath),
                Err(e) => println!("Error: {:?}", e),
            }
        }
        else
        {
            println!("Error: {} is not a directory name.", dirpath);
        }
        bar.inc(1);
    }
    bar.finish();
    return 0;
}

fn main()
{
    std::process::exit(execute());
}