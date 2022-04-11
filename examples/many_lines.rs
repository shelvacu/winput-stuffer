use winput_stuffer::send::send_text;

fn main() -> std::io::Result<()> {
    send_text("nline 1\nnline 2\nnline 3\n\n")?;
    send_text("rline 1\rrline 2\rrline 3\r\r")?;
    send_text("rnline 1\r\nrnline 2\r\nrnline 3\r\n\r\n")?;

    Ok(())
}