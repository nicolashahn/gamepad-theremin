use gilrs::{Axis, Event, EventType, Gilrs};

use adi::speaker::Speaker;
use twang::Sound;

const BASE_FREQ: f64 = 440.; // A4

fn main() {
    let mut gilrs = Gilrs::new().unwrap();
    let mut speaker = Speaker::new(0, false).unwrap();
    let mut freq = BASE_FREQ;
    let mut vol = 0.5;
    let mut sound = Sound::new(None, freq);

    loop {
        while let Some(Event {
            id: _id,
            event,
            time: _time,
        }) = gilrs.next_event()
        {
            match event {
                // left analog stick for pitch/frequency
                EventType::AxisChanged(Axis::LeftStickY, value, _) => {
                    // stick in resting position = BASE_FREQ
                    // range: [0.75 * BASE_FREQ, 1.5 * BASE_FREQ] or 1 octave
                    // if BASE_FREQ = 440 (A4), range is [330,660] (or [E4, E5] in notes)
                    let mult = f64::powf(2., value as f64) / 2.;
                    freq = BASE_FREQ / 2. + BASE_FREQ * mult;
                    println!("frequency: {}", freq);
                }
                // right analog stick for amplitude/volume
                EventType::AxisChanged(Axis::RightStickY, value, _) => {
                    // converts range of [-1.0, 1.0] to [0.0, 1.0]
                    vol = (value as f64 + 1.) / 2.;
                    println!("volume: {}", vol);
                }
                _ => {}
            }
        }

        sound.pitch(freq);
        speaker.update(&mut || {
            let x = sound.next().unwrap();
            let raw: i16 = (x.sin().pos() + x.tri().neg()).into();

            (raw as f64 * vol) as i16
        });
    }
}
