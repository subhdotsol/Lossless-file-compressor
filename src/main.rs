use std::fs;

// CLI entrypoint and argument handling

fn main() {
    // Read the file as raw bytes
    let data = match fs::read("files_to_test/hello.txt") {
        Ok(data) => data,
        Err(e) => {
            println!("Error reading file: {}", e);
            return;
        }
    };

    println!("File opened successfully!");

    // Display raw bytes
    println!("Raw bytes: {:?}", data);

    // Display total bytes
    println!("Total bytes: {}", data.len());

    // Frequency analysis
    let freq = frequency_analysis(&data);

    println!("\n--- Frequency Analysis ---");

    for (byte, frequency) in freq.iter().enumerate() {
        if *frequency > 0 {
            println!("Byte: {}, Frequency: {}", byte, frequency);
        }
    }
}

// Frequency analysis function
fn frequency_analysis(data: &[u8]) -> [usize; 256] {
    let mut freq = [0usize; 256];

    for &byte in data {
        freq[byte as usize] += 1;
    }

    freq
}
