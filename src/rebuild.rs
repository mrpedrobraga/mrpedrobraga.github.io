use miette::IntoDiagnostic;
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

pub fn rebuild_markdown_directory_recursive(
    source: PathBuf,
    target: PathBuf,
) -> miette::Result<()> {
    let source_dir = fs::read_dir(source).into_diagnostic()?;
    fs::create_dir_all(target.as_path()).into_diagnostic()?;
    let _ = source_dir
        .par_bridge()
        .map(|entry| -> miette::Result<()> {
            let entry = entry.into_diagnostic()?;
            if entry.metadata().into_diagnostic()?.is_dir() {
                rebuild_markdown_directory_recursive(
                    entry.path(),
                    target.clone().join(entry.path().file_name().unwrap()),
                )?;
            } else {
                let mut target_file = File::create(
                    target
                        .clone()
                        .join(entry.file_name())
                        .with_extension("html"),
                )
                .unwrap();
                let source_raw_markdown = fs::read_to_string(entry.path()).unwrap();

                let parsed_html: String = md_to_html(source_raw_markdown);

                target_file.write(parsed_html.as_bytes()).unwrap();
            }
            Ok(())
        })
        .collect::<Vec<_>>();

    Ok(())
}

pub fn md_to_html(source_raw_markdown: String) -> String {
    markdown::to_html(source_raw_markdown.as_str())
}
