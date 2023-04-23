use log::{info, warn, error, debug};
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();
    info!("Example info messsage");
    warn!("Example warn message");
    error!("Example errror message");
    debug!("Example debug message");
}
