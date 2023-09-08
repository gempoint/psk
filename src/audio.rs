use std::{
    fs,
    io::Error,
    path::PathBuf,
    process::{Child, Command, Stdio},
};

pub fn convert(input_file: &PathBuf, output_file: &PathBuf) -> Result<Child, Error> {
    Command::new("ffmpeg")
        .args([
            "-y",
            "-i",
            fs::canonicalize(input_file)
                .unwrap_or(input_file.to_owned())
                .to_str()
                .unwrap(),
            "-vn",
            fs::canonicalize(output_file)
                .unwrap_or(output_file.to_owned())
                .to_str()
                .unwrap(),
        ])
        .stdout(Stdio::piped())
        .spawn()
}
