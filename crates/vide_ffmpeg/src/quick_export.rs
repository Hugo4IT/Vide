use crate::FFmpegExporter;

pub fn to(output_file: impl ToString) -> FFmpegExporter {
    let extension = output_file
        .to_string()
        .split('.')
        .last()
        .unwrap_or_else(|| panic!("Vide Quick Export couldn't detect the file extension for {}", output_file.to_string()))
        .to_string();
    let extension = extension.as_str();

    match extension {
        "mp4" => FFmpegExporter::new(output_file, "mp4", "libx264", None),
        other => panic!("Vide Quick Export does not support or recognize {} (yet", other),
    }
}