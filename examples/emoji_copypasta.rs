fn main() {
    println!("Typing a bunch of stuff in 5 seconds!");
    std::thread::sleep_ms(5000);

    winput_stuffer::send::send_text("my top genre🎶 is pop 🎤???? POP ??? 😡😤 i am FUCKING fuming. i listen to indie 😎 and indie only 😔 WHY IS MY TOP GENRE POP 😩 spotify you are really 😡 really 😡 really 😡 pissing me off 🙄 I LISTEN 👂TO STRICTLY INDIE MUSIC 👍😤 I BLEED AND BREATHE 👄 INDIE MUSIC😎 NO POP😡").unwrap();

    std::thread::sleep_ms(100);
    println!();
    println!();
}