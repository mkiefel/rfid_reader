use futures::executor::LocalPool;
use futures_timer::Delay;
use rfid_reader::{Error, RfidReader};

/// Loops until it sees a tag from an RFID reader.
///
/// # Arguments
///
/// * `poll_duration` - Duration between two consecutive polls to the reader.
pub async fn wait_for_next_tag(poll_duration: std::time::Duration) -> Result<Vec<u8>, Error> {
    let reader = RfidReader::open()?;
    loop {
        if let Some(tag) = reader.get_tag()? {
            return Ok(tag);
        }
        Delay::new(poll_duration).await;
    }
}

fn main() {
    let mut pool = LocalPool::new();
    let card_id = pool.run_until(wait_for_next_tag(std::time::Duration::from_secs(1)));

    println!("{:?}", card_id);
}
