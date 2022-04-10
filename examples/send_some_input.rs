use winput_stuffer::input::{
    Input,
    KeyboardInput,
    KeyboardInputEnum,
    KeyboardInputSys,
    send_input,
};

fn main() {
    let kb_events:Vec<KeyboardInputSys> = "abc123".chars().into_iter().flat_map(|c| {
        [
            KeyboardInput{
                e: KeyboardInputEnum::UnicodeCodeUnit((c as u32).try_into().unwrap()),
                key_up: false,
                msg: None,
                time: None,
            }.into(),
            KeyboardInput{
                e: KeyboardInputEnum::UnicodeCodeUnit((c as u32).try_into().unwrap()),
                key_up: true,
                msg: None,
                time: None,
            }.into(),
        ].into_iter()
    }).collect();

    let events:Vec<Input> = kb_events.into_iter().map(|kb| Input::from_keyboard(&kb)).collect();

    println!("Typing some stuff in 5 seconds!");
    std::thread::sleep_ms(5000);

    send_input(events.as_slice()).unwrap();

    std::thread::sleep_ms(100);
    println!();
    println!();
}