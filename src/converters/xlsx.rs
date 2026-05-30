//! XLSX to Markdown converter — using calamine (read-only, 5-10x faster than umya)

use super::{ConversionResult, Converter, DocumentConverter, DocumentMetadata};
use crate::utils::{InputFormat, OutputFormat};
use anyhow::Result;
use async_trait::async_trait;
use calamine::{open_workbook_auto, Data, Reader};
use std::path::Path;

pub struct XlsxConverter;

impl XlsxConverter {
    pub fn new() -> Self { Self }
}

#[async_trait]
impl DocumentConverter for XlsxConverter {
    fn format(&self) -> InputFormat { InputFormat::Xlsx }

    async fn convert(&self, path: &Path, _output_format: &OutputFormat) -> Result<ConversionResult> {
        let path = path.to_path_buf();
        let file_size = tokio::fs::metadata(&path).await?.len();
        let result = tokio::task::spawn_blocking(move || -> Result<ConversionResult> {
            let mut workbook = open_workbook_auto(&path)?;
            let mut pages: Vec<(String, String)> = Vec::new();
            let sheet_names = workbook.sheet_names().to_owned();
            for sheet_name in sheet_names {
                if let Ok(range) = workbook.worksheet_range(&sheet_name) {
                    let md = sheet_to_markdown(&sheet_name, &range);
                    pages.push((md, sheet_name));
                }
            }
            let metadata = DocumentMetadata {
                title: None,
                author: None,
                page_count: pages.len(),
                word_count: 0,
                source_format: InputFormat::Xlsx,
                source_path: path.display().to_string(),
                file_size_bytes: file_size,
            };
            Ok(ConversionResult::from_pages(pages, metadata))
        }).await??;
        Ok(result)
    }
}

fn sheet_to_markdown(name: &str, range: &calamine::Range<Data>) -> String {
    let (rows, cols) = range.get_size();
    let mut md = String::with_capacity(rows * cols * 16);
    md.push_str("## ");
    md.push_str(name);
    md.push_str("\n\n");

    if rows == 0 || cols == 0 {
        md.push_str("_(empty)_\n");
        return md;
    }

    // Header row
    for c in 0..cols {
        md.push_str("| ");
        if let Some(cell) = range.get((0, c)) {
            md.push_str(&cell_to_string(cell));
        }
        md.push(' ');
    }
    md.push_str("|\n");

    // Separator
    for _ in 0..cols { md.push_str("|---"); }
    md.push_str("|\n");

    // Data rows
    for r in 1..rows {
        for c in 0..cols {
            md.push_str("| ");
            if let Some(cell) = range.get((r, c)) {
                md.push_str(&cell_to_string(cell));
            }
            md.push(' ');
        }
        md.push_str("|\n");
    }
    md
}

fn cell_to_string(cell: &Data) -> String {
    match cell {
        Data::Empty => String::new(),
        Data::String(s) => s.replace('|', "\\|").replace('\n', " "),
        Data::Float(f) => f.to_string(),
        Data::Int(i) => i.to_string(),
        Data::Bool(b) => b.to_string(),
        Data::DateTime(dt) => dt.to_string(),
        Data::DateTimeIso(s) | Data::DurationIso(s) => s.clone(),
        Data::Error(e) => format!("ERR({:?})", e),
    }
}
