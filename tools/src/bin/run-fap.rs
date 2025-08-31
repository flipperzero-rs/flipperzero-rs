use std::{
    fmt,
    io::{self, Write},
    path::PathBuf,
    thread,
    time::Duration,
};

use clap::Parser;
use flipperzero_tools::{serial, storage};
use rand::{rng, Rng};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Serial port (e.g. `COM3` on Windows or `/dev/ttyUSB0` on Linux)
    #[arg(short, long)]
    port: Option<String>,

    /// Path to the FAP binary to run.
    fap: PathBuf,

    /// Arguments to provide to the FAP binary.
    ///
    /// Ignored until [flipperdevices/flipperzero-firmware#2505][1] is resolved.
    ///
    /// [1]: https://github.com/flipperdevices/flipperzero-firmware/issues/2505
    args: Vec<String>,
}

enum Error {
    FapIsNotAFile,
    FlipperZeroNotFound,
    FailedToOpenSerialPort(serialport::Error),
    FailedToStartSerialInterface(io::Error),
    FailedToUploadFap(io::Error),
    MkdirFailed(storage::FlipperPath, io::Error),
    RemoveFailed(storage::FlipperPath, io::Error),
    Io(io::Error),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::FapIsNotAFile => write!(f, "provided FAP path is not to a file"),
            Error::FlipperZeroNotFound => write!(f, "unable to find Flipper Zero"),
            Error::FailedToOpenSerialPort(e) => write!(f, "unable to open serial port: {}", e),
            Error::FailedToStartSerialInterface(e) => {
                write!(f, "unable to start serial interface: {}", e)
            }
            Error::FailedToUploadFap(e) => write!(f, "unable to upload FAP: {}", e),
            Error::MkdirFailed(path, e) => write!(f, "unable to make directory '{}': {}", path, e),
            Error::RemoveFailed(path, e) => write!(f, "unable to remove '{}': {}", path, e),
            Error::Io(e) => e.fmt(f),
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::Io(value)
    }
}

fn wait_for_idle(serial_cli: &mut serial::SerialCli) -> io::Result<()> {
    loop {
        serial_cli.send_and_wait_eol("loader info")?;
        if serial_cli
            .consume_response()?
            .contains("No application is running")
        {
            break Ok(());
        }
        thread::sleep(Duration::from_millis(200));
    }
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    if !cli.fap.is_file() {
        return Err(Error::FapIsNotAFile);
    }
    let file_name = cli
        .fap
        .file_name()
        .ok_or(Error::FapIsNotAFile)?
        .to_str()
        // If the FAP filename is not valid UTF-8, use a placeholder.
        .unwrap_or("tmp-filename.fap");

    let port_info =
        serial::find_flipperzero(cli.port.as_deref()).ok_or(Error::FlipperZeroNotFound)?;
    let port = serialport::new(port_info.port_name, serial::BAUD_115200)
        .timeout(Duration::from_secs(30))
        .open()
        .map_err(Error::FailedToOpenSerialPort)?;
    let mut store = storage::FlipperStorage::new(port);
    store.start().map_err(Error::FailedToStartSerialInterface)?;

    // Remove any stale output file
    let output_file = storage::FlipperPath::from("/ext/flipperzero-rs-stdout");
    if store.exist_file(&output_file)? {
        store.remove(&output_file)?;
    }

    // Upload the FAP to a temporary directory.
    let dest_dir =
        storage::FlipperPath::from(format!("/ext/.tmp/rs-{:08x}", rng().random::<u32>()));
    let dest_file = dest_dir.clone() + file_name;
    store
        .mkdir(&dest_dir)
        .map_err(|e| Error::MkdirFailed(dest_dir.clone(), e))?;
    store
        .send_file(&cli.fap, &dest_file)
        .map_err(Error::FailedToUploadFap)?;

    let serial_cli = store.cli_mut();

    // Wait for no application to be running.
    wait_for_idle(serial_cli)?;

    // Run the FAP.
    serial_cli.send_and_wait_eol(&format!("loader open {} {}", dest_file, cli.args.join(" ")))?;

    // Wait for the FAP to finish.
    wait_for_idle(serial_cli)?;

    // Download and print the output file, if present.
    if store.exist_file(&output_file)? {
        let output = store.read_file(&output_file)?;
        io::stdout().write_all(output.as_ref())?;
        store.remove(&output_file)?;
    }

    // Remove the FAP and temporary directory.
    store
        .remove(&dest_file)
        .map_err(|e| Error::RemoveFailed(dest_file, e))?;
    store
        .remove(&dest_dir)
        .map_err(|e| Error::RemoveFailed(dest_dir, e))?;

    Ok(())
}
