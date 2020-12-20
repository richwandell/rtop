pub fn left_pad(text: String, len: usize) -> String {
    let mut pad_text = String::new();
    for _ in 0..(len - text.len()) {
        pad_text += " ";
    }
    return pad_text.to_string() + &text;
}

pub fn right_pad(text: String, len: usize) -> String {
    let mut pad_text = String::new();
    if text.len() < len {
        for _ in 0..(len - text.len()) {
            pad_text += " ";
        }
    }
    return text + &pad_text.to_string();
}

pub fn center(text: String, len: usize) -> String {
    let pad_size = (len / 2) - (text.len() / 2);
    let mut pad = String::new();

    for _ in 0..pad_size {
        pad += " ";
    }

    return pad.clone() + &text + &pad;
}

pub fn make_bars(perc: f64) -> String {
    let num_bars = (perc * 25.0) as i64;
    let mut bars = "".to_string();
    for _ in 0..num_bars {
        bars += "|";
    }
    if bars.len() < 25 {
        for _ in 0..25-bars.len() {
            bars += " ";
        }
    }
    return bars.to_string();
}