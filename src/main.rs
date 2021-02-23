use std::time::SystemTime;
fn swap(word_part: &mut [char]) {
    let aux = word_part[0];
    word_part[0] = word_part[1];
    word_part[1] = aux;
}

fn main() {
    let start = SystemTime::now();
    let mut word: [char; 13] = [
        'I', 'N', 'S', 'E', 'R', 'T', 'I', 'O', 'N', 'S', 'O', 'R', 'T',
    ];

    let mut j: usize;

    for index in 0..word.len() {
        j = index;
        while j > 0 && (word[j - 1] > word[j]) {
            swap(&mut word[j - 1..=j]);
            j -= 1;
        }
    }
    let end = SystemTime::now();
    println!(
        "Algorithm lasted:{}",
        end.duration_since(start).unwrap().as_nanos()
    );
    for item in &word {
        print!("{},", item);
    }
}
