use std::thread::sleep_ms;

use winput_stuffer::send::send_key;

fn main() -> std::io::Result<()> {
    send_key("alt_l", true)?;
    send_key("tab", true)?;
    send_key("tab", false)?;
    sleep_ms(200);
    send_key("tab", true)?;
    send_key("tab", false)?;
    sleep_ms(200);
    send_key("alt_l", false)?;

    Ok(())
}