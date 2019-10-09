use rfid_reader::RfidReader;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let reader = RfidReader::open()?;
    let tag = reader.get_tag()?;
    println!("tag: {:?}", tag);
    Ok(())
}
