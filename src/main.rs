fn main() -> anyhow::Result<()> {
    use std::fs::{read_dir, File};
    use std::io::{Read, Seek, Write};

    use esp_idf_svc::fs::fatfs::Fatfs;
    use esp_idf_svc::hal::gpio;
    use esp_idf_svc::hal::prelude::*;
    use esp_idf_svc::hal::sd::{
        mmc::SdMmcHostConfiguration, mmc::SdMmcHostDriver, SdCardConfiguration, SdCardDriver,
    };
    use esp_idf_svc::io::vfs::MountedFatfs;
    use esp_idf_svc::log::EspLogger;

    use log::info;

    esp_idf_svc::sys::link_patches();

    EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;
    let pins = peripherals.pins;
    let mut host_config = SdMmcHostConfiguration::new();
    host_config.enable_internal_pullups = false;
    let sd_card_driver = SdCardDriver::new_mmc(
        // => Data width = 4 bits
        SdMmcHostDriver::new_1bit(
            peripherals.sdmmc1,
            pins.gpio35,
            pins.gpio36,
            pins.gpio37,
            /*
            pins.gpio38,
            pins.gpio33,
            pins.gpio34,*/
            None::<gpio::AnyIOPin>,
            None::<gpio::AnyIOPin>,
            &host_config,
        )?,
        // => Data width = 1 bit
        // Comment out the above configuration and uncomment this block
        // if you have connected only the d0 pin
        // SdMmcHostDriver::new_slot1_1bit(
        //     peripherals.sdmmc1,
        //     pins.gpio15,
        //     pins.gpio14,
        //     pins.gpio2,
        //     None::<gpio::AnyIOPin>,
        //     None::<gpio::AnyIOPin>,
        //     &SdMmcHostConfiguration::new(),
        // )?,
        &SdCardConfiguration::new(),
    )?;

    // Keep it around or else it will be dropped and unmounted
    let _mounted_fatfs = MountedFatfs::mount(Fatfs::new_sdcard(0, sd_card_driver)?, "/sdcard", 4)?;

    let content = b"Hello, world!";

    {
        let mut file = File::create("/sdcard/test.txt")?;

        info!("File {file:?} created");

        file.write_all(content).expect("Write failed");

        info!("File {file:?} written with {content:?}");

        file.seek(std::io::SeekFrom::Start(0)).expect("Seek failed");

        info!("File {file:?} seeked");
    }

    {
        let mut file = File::open("/sdcard/test.txt")?;

        info!("File {file:?} opened");

        let mut file_content = String::new();

        file.read_to_string(&mut file_content).expect("Read failed");

        info!("File {file:?} read: {file_content}");

        assert_eq!(file_content.as_bytes(), content);
    }

    {
        let directory = read_dir("/sdcard")?;

        for entry in directory {
            log::info!("Entry: {:?}", entry?.file_name());
        }
    }

    Ok(())
}
