use std::io::{
    self,
    BufRead,
    Write,
};

use crate::{
    CraneError,
    PathMapping,
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TransformStats {
    pub commit_count: usize,
    pub blob_count: usize,
    pub rewritten_ops: usize,
    pub dropped_ops: usize,
}

pub fn transform_export<R, W>(
    mut reader: R,
    mut writer: W,
    import_ref: &str,
    mappings: &[PathMapping],
) -> Result<TransformStats, CraneError>
where
    R: BufRead,
    W: Write,
{
    let mut stats = TransformStats::default();
    let mut line = Vec::new();

    while read_line(&mut reader, &mut line)? {
        let line_text = String::from_utf8_lossy(&line);

        if line_text.starts_with("blob\n") {
            stats.blob_count += 1;
            writer.write_all(&line).map_err(CraneError::Io)?;
            copy_blob_body(&mut reader, &mut writer)?;
            continue;
        }

        if line_text.starts_with("commit ") {
            stats.commit_count += 1;
            writer
                .write_all(format!("commit {import_ref}\n").as_bytes())
                .map_err(CraneError::Io)?;
            transform_commit_body(
                &mut reader,
                &mut writer,
                mappings,
                &mut stats,
            )?;
            continue;
        }

        if line_text.starts_with("reset ") {
            writer
                .write_all(format!("reset {import_ref}\n").as_bytes())
                .map_err(CraneError::Io)?;
            copy_reset_body(&mut reader, &mut writer)?;
            continue;
        }

        if line_text.starts_with("tag ") {
            skip_tag_body(&mut reader)?;
            continue;
        }

        writer.write_all(&line).map_err(CraneError::Io)?;
    }

    writer.flush().map_err(CraneError::Io)?;
    Ok(stats)
}

fn copy_blob_body<R, W>(
    reader: &mut R,
    writer: &mut W,
) -> Result<(), CraneError>
where
    R: BufRead,
    W: Write,
{
    let mut line = Vec::new();
    loop {
        if !read_line(reader, &mut line)? {
            return Err(CraneError::FastExport(
                "unexpected EOF while reading blob body".to_string(),
            ));
        }
        writer.write_all(&line).map_err(CraneError::Io)?;
        if let Some(data_len) = parse_data_len(&line)? {
            copy_exact(reader, writer, data_len + 1)?;
            return Ok(());
        }
    }
}

fn transform_commit_body<R, W>(
    reader: &mut R,
    writer: &mut W,
    mappings: &[PathMapping],
    stats: &mut TransformStats,
) -> Result<(), CraneError>
where
    R: BufRead,
    W: Write,
{
    let mut line = Vec::new();
    loop {
        if !read_line(reader, &mut line)? {
            writer.write_all(b"\n").map_err(CraneError::Io)?;
            return Ok(());
        }

        if line == b"\n" {
            writer.write_all(&line).map_err(CraneError::Io)?;
            return Ok(());
        }

        if let Some(data_len) = parse_data_len(&line)? {
            writer.write_all(&line).map_err(CraneError::Io)?;
            copy_exact(reader, writer, data_len)?;
            continue;
        }

        if let Some(rewritten) = rewrite_file_op(&line, mappings, stats)? {
            writer
                .write_all(rewritten.as_bytes())
                .map_err(CraneError::Io)?;
            continue;
        }

        writer.write_all(&line).map_err(CraneError::Io)?;
    }
}

fn copy_reset_body<R, W>(
    reader: &mut R,
    writer: &mut W,
) -> Result<(), CraneError>
where
    R: BufRead,
    W: Write,
{
    let mut line = Vec::new();
    loop {
        if !read_line(reader, &mut line)? {
            return Ok(());
        }
        writer.write_all(&line).map_err(CraneError::Io)?;
        if line == b"\n" {
            return Ok(());
        }
    }
}

fn skip_tag_body<R>(reader: &mut R) -> Result<(), CraneError>
where
    R: BufRead,
{
    let mut line = Vec::new();
    loop {
        if !read_line(reader, &mut line)? {
            return Ok(());
        }
        if let Some(data_len) = parse_data_len(&line)? {
            copy_exact(reader, &mut io::sink(), data_len + 1)?;
            continue;
        }
        if line == b"\n" {
            return Ok(());
        }
    }
}

fn rewrite_file_op(
    line: &[u8],
    mappings: &[PathMapping],
    stats: &mut TransformStats,
) -> Result<Option<String>, CraneError> {
    let raw = std::str::from_utf8(line).map_err(|error| {
        CraneError::FastExport(format!(
            "fast-export line is not utf-8: {error}"
        ))
    })?;
    let trimmed = raw.trim_end_matches('\n');

    if trimmed == "deleteall" {
        return Ok(Some("deleteall\n".to_string()));
    }

    if let Some(rest) = trimmed.strip_prefix("M ") {
        let mut parts = rest.splitn(3, ' ');
        let mode = parts.next().ok_or_else(|| malformed_op(trimmed))?;
        let data_ref = parts.next().ok_or_else(|| malformed_op(trimmed))?;
        let path = parts.next().ok_or_else(|| malformed_op(trimmed))?;
        match remap_path(path, mappings) {
            Some(mapped) => {
                stats.rewritten_ops += 1;
                return Ok(Some(format!("M {mode} {data_ref} {mapped}\n")));
            },
            None => {
                stats.dropped_ops += 1;
                return Ok(None);
            },
        }
    }

    if let Some(path) = trimmed.strip_prefix("D ") {
        match remap_path(path, mappings) {
            Some(mapped) => {
                stats.rewritten_ops += 1;
                return Ok(Some(format!("D {mapped}\n")));
            },
            None => {
                stats.dropped_ops += 1;
                return Ok(None);
            },
        }
    }

    if let Some(rest) = trimmed.strip_prefix("R ") {
        return rewrite_pair_op('R', rest, mappings, stats);
    }

    if let Some(rest) = trimmed.strip_prefix("C ") {
        return rewrite_pair_op('C', rest, mappings, stats);
    }

    Ok(None)
}

fn rewrite_pair_op(
    op: char,
    rest: &str,
    mappings: &[PathMapping],
    stats: &mut TransformStats,
) -> Result<Option<String>, CraneError> {
    let mut parts = rest.splitn(2, ' ');
    let source = parts.next().ok_or_else(|| malformed_op(rest))?;
    let destination = parts.next().ok_or_else(|| malformed_op(rest))?;

    let Some(mapped_source) = remap_path(source, mappings) else {
        stats.dropped_ops += 1;
        return Ok(None);
    };
    let Some(mapped_destination) = remap_path(destination, mappings) else {
        stats.dropped_ops += 1;
        return Ok(None);
    };

    stats.rewritten_ops += 1;
    Ok(Some(format!("{op} {mapped_source} {mapped_destination}\n")))
}

fn malformed_op(raw: &str) -> CraneError {
    CraneError::FastExport(format!("unsupported fast-export file op: {raw}"))
}

pub fn remap_path(
    path: &str,
    mappings: &[PathMapping],
) -> Option<String> {
    for mapping in mappings {
        if path == mapping.source {
            return Some(remap_exact_path(path, mapping));
        }
        if let Some(suffix) = path.strip_prefix(&mapping.source)
            && suffix.starts_with('/')
        {
            return Some(join_mapped_path(&mapping.destination, suffix));
        }
    }
    None
}

fn remap_exact_path(
    path: &str,
    mapping: &PathMapping,
) -> String {
    if !mapping.destination.is_empty() {
        return mapping.destination.clone();
    }

    path.rsplit('/').next().unwrap_or(path).to_string()
}

fn join_mapped_path(
    destination: &str,
    suffix: &str,
) -> String {
    let trimmed_suffix = suffix.trim_start_matches('/');
    if destination.is_empty() {
        return trimmed_suffix.to_string();
    }
    if trimmed_suffix.is_empty() {
        return destination.to_string();
    }

    format!("{destination}/{trimmed_suffix}")
}

fn parse_data_len(line: &[u8]) -> Result<Option<usize>, CraneError> {
    let raw = std::str::from_utf8(line).map_err(|error| {
        CraneError::FastExport(format!(
            "fast-export line is not utf-8: {error}"
        ))
    })?;
    let Some(rest) = raw.strip_prefix("data ") else {
        return Ok(None);
    };
    rest.trim_end_matches('\n')
        .parse::<usize>()
        .map(Some)
        .map_err(|error| {
            CraneError::FastExport(format!(
                "invalid fast-export data length `{}`: {error}",
                rest.trim()
            ))
        })
}

fn copy_exact<R, W>(
    reader: &mut R,
    writer: &mut W,
    len: usize,
) -> Result<(), CraneError>
where
    R: BufRead,
    W: Write,
{
    let mut remaining = len;
    while remaining > 0 {
        let buffer = reader.fill_buf().map_err(CraneError::Io)?;
        if buffer.is_empty() {
            return Err(CraneError::FastExport(
                "unexpected EOF while copying fast-export payload".to_string(),
            ));
        }

        let to_copy = remaining.min(buffer.len());
        writer
            .write_all(&buffer[..to_copy])
            .map_err(CraneError::Io)?;
        reader.consume(to_copy);
        remaining -= to_copy;
    }
    Ok(())
}

fn read_line<R>(
    reader: &mut R,
    buffer: &mut Vec<u8>,
) -> Result<bool, CraneError>
where
    R: BufRead,
{
    buffer.clear();
    let count = reader.read_until(b'\n', buffer).map_err(CraneError::Io)?;
    Ok(count > 0)
}
