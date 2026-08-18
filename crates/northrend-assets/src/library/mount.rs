use std::{
    cmp::Ordering,
    fs,
    path::{Path, PathBuf},
};

use super::AssetError;

pub(super) fn discover(path: &Path, base_archives: &[&str]) -> Result<Vec<PathBuf>, AssetError> {
    let mut archives = Vec::new();

    visit(path, &mut archives)?;
    archives.sort_unstable_by(|left, right| {
        compare(left, right, base_archives).then_with(|| path_name(left).cmp(&path_name(right)))
    });

    Ok(archives)
}

fn visit(path: &Path, archives: &mut Vec<PathBuf>) -> Result<(), AssetError> {
    let entries = fs::read_dir(path).map_err(|source| AssetError::Scan {
        path: path.into(),
        source,
    })?;

    for entry in entries {
        let entry = entry.map_err(|source| AssetError::Scan {
            path: path.into(),
            source,
        })?;
        let file_type = entry.file_type().map_err(|source| AssetError::Scan {
            path: entry.path(),
            source,
        })?;
        let entry_path = entry.path();

        if file_type.is_dir() {
            visit(&entry_path, archives)?;
        } else if file_type.is_file() && is_archive(&entry_path) && !is_backup(&entry_path) {
            archives.push(entry_path);
        }
    }

    Ok(())
}

fn compare(left: &Path, right: &Path, base_archives: &[&str]) -> Ordering {
    let left_name = file_name(left);
    let right_name = file_name(right);
    let left_base = base_order(&left_name, base_archives);
    let right_base = base_order(&right_name, base_archives);

    match (left_base, right_base) {
        (Some(left), Some(right)) => return left.cmp(&right),
        (Some(_), None) => return Ordering::Less,
        (None, Some(_)) => return Ordering::Greater,
        (None, None) => {}
    }

    let left_patch = patch_suffix(&left_name);
    let right_patch = patch_suffix(&right_name);

    match (left_patch, right_patch) {
        (Some(left), Some(right)) => compare_patch(left, right),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => left_name.cmp(&right_name),
    }
}

fn compare_patch(left: &str, right: &str) -> Ordering {
    let left_order = patch_order(left);
    let right_order = patch_order(right);

    left_order
        .cmp(&right_order)
        .then_with(|| match left_order {
            1 => compare_number(left, right),
            _ => left.cmp(right),
        })
        .then_with(|| left.cmp(right))
}

fn compare_number(left: &str, right: &str) -> Ordering {
    let left = left.trim_start_matches('0');
    let right = right.trim_start_matches('0');

    left.len().cmp(&right.len()).then_with(|| left.cmp(right))
}

fn patch_order(suffix: &str) -> u8 {
    if suffix.is_empty() {
        0
    } else if suffix.bytes().all(|byte| byte.is_ascii_digit()) {
        1
    } else if suffix.len() == 1 && suffix.as_bytes()[0].is_ascii_alphabetic() {
        2
    } else {
        3
    }
}

fn patch_suffix(name: &str) -> Option<&str> {
    let stem = name.strip_suffix(".mpq")?;

    if stem == "patch" {
        Some("")
    } else {
        stem.strip_prefix("patch-")
    }
}

fn base_order(name: &str, base_archives: &[&str]) -> Option<usize> {
    base_archives
        .iter()
        .position(|archive| archive.eq_ignore_ascii_case(name))
}

fn is_archive(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("mpq"))
}

fn is_backup(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.eq_ignore_ascii_case("backup.mpq"))
}

fn file_name(path: &Path) -> String {
    path.file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_ascii_lowercase()
}

fn path_name(path: &Path) -> String {
    path.to_string_lossy().to_ascii_lowercase()
}
