#[allow(unused_imports)]
use std::io;
use std::{
    env,
    fmt::format,
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!(
            "[+] Usage: ./spliter --split <file> <chunks>\n ./spliter --merge <file> <chunks> "
        );
    }
    //    split(&args[1], args[2].parse::<u64>().unwrap());

    let mode = &args[1];
    let file = &args[2];
    let num = args[3].parse::<u64>().unwrap();

    match mode.as_str() {
        "--split" => split(file, num)?,
        "--merge" => merge(file, num)?,
        _ => eprint!("there is a issue"),
    }
    Ok(())
}

// There is no need of nested pattern matching can be done by just using "?" and unwrap here i'm
// just doing this for fun - I know the clean way, let me suffer artistically!!
//

#[allow(dead_code)]
#[allow(unused_variables)]
fn split<P: AsRef<Path>>(file: P, num: u64) -> io::Result<()> {
    let mut infile = File::open(&file)?;
    let size = fs::metadata(&file).unwrap().len();
    let partsize = size / num;
    let reminder_size = size % num;
    println!("file part-size: {}", partsize);
    println!("reminder part-size: {}", reminder_size);
    println!("file-size: {:?}", size);

    let filename = file.as_ref().to_string_lossy();

    //    let mut chunk = Vec::with_capacity(num as usize);

    for i in 0..num {
        let chunk_size = if i == num - 1 {
            partsize + reminder_size
        } else {
            partsize
        };

        let mut data = vec![0u8; chunk_size as usize];
        infile.read_exact(&mut data)?;
        println!("[+] Chunk {} → {} bytes", i, data.len());

        let parts_name = format!("{filename}.part{i}");
        let mut outfile = File::create(&parts_name)?;

        outfile.write_all(&data);

        println!("[+] Written {} ({} bytes)", parts_name, data.len());

        //        chunk.push(data);
    }
    Ok(())
}

fn merge<P: AsRef<Path>>(file: P, num: u64) -> io::Result<()> {
    let filename = file.as_ref().to_string_lossy();
    let merged_name = format!("{}.merged", filename);

    let mut out_file = File::create(&merged_name).unwrap();
    for i in 0..num {
        let part_name = format!("{filename}.part{i}");
        let mut infile = File::open(&part_name).unwrap();
        let mut buffer = Vec::new();
        infile.read_to_end(&mut buffer);

        out_file.write_all(&buffer).unwrap();
        println!("[+] Merged {}", part_name);
    }
    println!("[+] Final output: {}", merged_name);

    Ok(())
}
