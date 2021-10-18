use opencv::prelude::*;
use opencv::videoio;

fn main() {
    run().unwrap_or_else(|error| {
        panic!("{}", error);
    })
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut cap = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let mut frame = Mat::default();
    loop {
        cap.read(&mut frame)?;
        opencv::highgui::imshow("camera", &mut frame);
        match opencv::highgui::wait_key(2) {
            Ok(key) => match key {
                // q key
                113 => break,
                _ => continue,
            },
            _ => continue,
        }
    }
    Ok(())
}
