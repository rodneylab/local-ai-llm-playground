use std::io::Write;

use miette::{Context, IntoDiagnostic};

pub fn display_chunk(stdout_handle: &mut impl Write, chunk: &str) -> miette::Result<()> {
    if let Some((complete_lines, rest)) = chunk.rsplit_once('\n') {
        writeln!(stdout_handle, "{complete_lines}")
            .into_diagnostic()
            .wrap_err("Writing chunk to stdout")?;
        write!(stdout_handle, "{rest}")
            .into_diagnostic()
            .wrap_err("Writing chunk to stdout")?;
    } else {
        write!(stdout_handle, "{chunk}")
            .into_diagnostic()
            .wrap_err("Writing chunk to stdout")?;
    }
    stdout_handle
        .flush()
        .into_diagnostic()
        .wrap_err("Flushing stdout")?;

    Ok(())
}

pub fn get_prompt(
    mut reader: impl std::io::BufRead,
    mut writer: impl std::io::Write,
) -> miette::Result<String> {
    write!(writer, "Enter prompt for Gemma model...\n>>> ")
        .into_diagnostic()
        .wrap_err("Writing prompt to writer")?;
    writer
        .flush()
        .into_diagnostic()
        .wrap_err("Writing prompt to stdout")?;
    let mut prompt = String::new();
    reader
        .read_line(&mut prompt)
        .into_diagnostic()
        .wrap_err("Reading prompt from stdin")?;
    let prompt = prompt.trim();

    Ok(prompt.to_string())
}

#[cfg(test)]
mod tests {
    use crate::ui::display_chunk;

    use super::get_prompt;

    #[test]
    fn display_chunk_outputs_single_word() {
        // arrange
        let chunk = "But";
        let mut output: Vec<u8> = Vec::new();

        // act
        display_chunk(&mut output, chunk).unwrap();

        // assert
        let output = str::from_utf8(&output).unwrap();
        assert_eq!(output, "But");
    }

    #[test]
    fn display_chunk_outputs_line() {
        // arrange
        let chunk = "But dangers lurked, beneath the emerald sheen,\n";
        let mut output: Vec<u8> = Vec::new();

        // act
        display_chunk(&mut output, chunk).unwrap();

        // assert
        let output = str::from_utf8(&output).unwrap();
        assert_eq!(output, chunk);
    }

    #[test]
    fn display_chunk_outputs_paragraphs() {
        // arrange
        let chunk = "
But dangers lurked, beneath the emerald sheen,
The Rajah’s tigers, hungry and keen.
And Kobra’s coils, a venomous embrace,
Threatened the cub, and Pipkin’s valiant space.
Pipkin fought with branches, cunning and with speed,
Protecting Shade, fulfilling a noble deed.

The Mandrake taught him patience, slow and true,
To read the jungle’s signs, the dew, the hue


";
        let mut output: Vec<u8> = Vec::new();

        // act
        display_chunk(&mut output, chunk).unwrap();

        // assert
        let output = str::from_utf8(&output).unwrap();
        assert_eq!(output, chunk);
    }

    #[test]
    fn display_chunk_outputs_fragmented_paragraphs() {
        // arrange
        let chunks = vec![
            "
But dangers ",
            "lurked, beneath",
            " the emerald sheen,
The Rajah’s tigers, hungry and keen.
And Kobra’s coils, a venomous embrace,",
            "
Threatened the cub, and Pipkin’s valiant space.
",
            "Pipkin fought with branches, cunning and with speed,
Protecting Shade, fulfilling a noble deed.

The Mandrake taught him patience, slow and true,
To read the jungle’s signs, the dew, the hue


",
        ];
        let mut output: Vec<u8> = Vec::new();

        // act
        for chunk in &chunks {
            display_chunk(&mut output, chunk).unwrap();
        }

        // assert
        let output = str::from_utf8(&output).unwrap();
        assert_eq!(output, chunks.join(""));
    }

    #[test]
    fn get_prompt_outputs_expected_user_prompt() {
        // arrange
        let input_buffer = b"Please tell me a funny joke.\n".to_vec();
        let mut output_buffer: Vec<u8> = Vec::new();

        // act
        let outcome = get_prompt(&input_buffer[..], &mut output_buffer).unwrap();

        // assert
        assert!(
            str::from_utf8(&output_buffer)
                .unwrap()
                .starts_with("Enter prompt for Gemma model...\n>>> ")
        );
        assert_eq!(outcome, "Please tell me a funny joke.".to_string());
    }
}
