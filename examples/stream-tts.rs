use alltalk_api::{reqwest, Client};
use rodio::{OutputStream, Sink, Source};
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(reqwest::Client::new());
    let mut input = String::new();

    // Check if the server is ready before we start
    match client.get_ready().await {
        Ok(ready) => {
            if ready {
                println!("Server is ready.")
            } else {
                return Err("Server is not ready or unavailable.".into());
            }
        }
        Err(err) => {
            return Err(format!("Error checking server readiness: {err}").into());
        }
    }

    // Try to turn deepspeed on
    client.set_deepspeed(true).await?;

    // Set up the audio output
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    // Choose voice
    let voices = client.get_voices_list().await?;
    println!("Available voices:");
    for (i, voice) in voices.iter().enumerate() {
        println!("  {}: {}", i, voice);
    }
    print!("Enter voice index: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    let voice = &voices[match input.trim().parse::<usize>() {
        Ok(i) if i < voices.len() => i,
        _ => return Err("Invalid voice index.".into()),
    }];
    input.clear();

    // Input loop
    loop {
        // Get input from the user
        print!("Enter text to generate TTS (or type 'q' to quit): ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;

        let trimmed = input.trim().to_owned();

        // Quit if the user types "q"
        if trimmed == "q" {
            break;
        }

        // Generate the TTS stream
        match client.generate_tts_stream(trimmed, voice, "en").await {
            Ok(streaming_source) => {
                println!("Playing audio...");
                sink.append(streaming_source.convert_samples::<f32>());
                sink.sleep_until_end(); // Wait for audio to finish playing
            }
            Err(err) => eprintln!("Error generating TTS stream: {:?}", err),
        }

        input.clear();
    }

    println!("quitting...");
    Ok(())
}
