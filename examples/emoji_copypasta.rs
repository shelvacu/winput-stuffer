fn main() {
    println!("Typing a bunch of stuff in 5 seconds!");
    std::thread::sleep_ms(5000);

    winput_stuffer::send::send_text("my top genreπΆ is pop π€???? POP ??? π‘π€ i am FUCKING fuming. i listen to indie π and indie only π WHY IS MY TOP GENRE POP π© spotify you are really π‘ really π‘ really π‘ pissing me off π I LISTEN πTO STRICTLY INDIE MUSIC ππ€ I BLEED AND BREATHE π INDIE MUSICπ NO POPπ‘").unwrap();

    std::thread::sleep_ms(100);
    println!();
    println!();
}