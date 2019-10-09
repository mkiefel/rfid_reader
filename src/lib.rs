#[macro_use]
extern crate quick_error;

/// Represents a RFIDeas pcProx USB reader.
pub struct RfidReader {
    dev: hidapi::HidDevice,
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Device(err: hidapi::HidError) {
            cause(err)
                from()
        }
    }
}

impl RfidReader {
    /// Opens the RFID reader device.
    pub fn open() -> Result<Self, Error> {
        let api = hidapi::HidApi::new()?;
        let dev = api.open(0x0c27, 0x3bfa)?;
        Ok(Self { dev })
    }

    /// Returns the tag that is currently placed on the reader.
    ///
    /// Returns a vector of bytes representing the identity of the RFID card if
    /// a card is present. `None` otherwise.
    pub fn get_tag(&self) -> Result<Option<Vec<u8>>, Error> {
        let cmd_0: [u8; 9] = [0, 0x8f, 0, 0, 0, 0, 0, 0, 0];
        let cmd_1: [u8; 9] = [0, 0x8e, 0, 0, 0, 0, 0, 0, 0];
        let mut report_0: [u8; 9] = [1, 0, 0, 0, 0, 0, 0, 0, 0];
        let mut report_1: [u8; 9] = [1, 0, 0, 0, 0, 0, 0, 0, 0];
        self.dev.send_feature_report(&cmd_0)?;
        self.dev.get_feature_report(&mut report_0)?;
        if report_0.iter().all(|b| *b == 0) {
            return Ok(None);
        }

        self.dev.send_feature_report(&cmd_1)?;
        self.dev.get_feature_report(&mut report_1)?;
        let mut bytes = (report_1[1] / 8) as usize;
        if report_1[1] % 8 != 0 {
            bytes += 1;
        }

        Ok(Some(report_0[0..bytes].to_owned()))
    }
}
