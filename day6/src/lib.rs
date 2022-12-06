// Solution for part 1
pub fn find_packet_markers(signals: &[&str]) -> Vec<usize> {
    let mut markers = Vec::new();
    for signal in signals {
        markers.push(find_first_marker(&signal, 4));
    }

    markers
}

// Solution for part 2, mostly identical except for larger marker size.
pub fn find_message_markers(signals: &[&str]) -> Vec<usize> {
    let mut markers = Vec::new();
    for signal in signals {
        markers.push(find_first_marker(&signal, 14));
    }

    markers
}

fn find_first_marker(signal: &str, len: usize) -> usize {
    let bytes: Vec<u8> = signal.bytes().collect();
    for i in 0..bytes.len() - len {
        let mut quartet = vec![0; len];
        quartet.clone_from_slice(&bytes[i..i+len]);
        quartet.sort();
        quartet.dedup();
        if quartet.len() == len {
            return i+len
        }
    }
    0
}