use std::io;
#[cfg(windows)] use winres::WindowsResource;

fn main() -> io::Result<()> {
    #[cfg(windows)] {
        WindowsResource::new()
            .set_icon("./assets/smartcalc.ico")
            .compile()?;
    }
    Ok(())
}