//! 对齐: `cn.hutool.core.io.file.PathUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/file/PathUtil.java
//!
//! NIO `Path` 风格工具；纯 Channel/Selector API 保持 planned。
//! 部分路径操作委托 [`crate::FileUtil`]。

use crate::FileUtil;
use std::fs;
use std::io::{self, BufReader, Read, Write};
use std::path::{Component, Path, PathBuf};

/// 对齐 Java 类: `cn.hutool.core.io.file.PathUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct PathUtil;

impl PathUtil {
    /// 逻辑规范化路径，解析 `.` 与 `..`（对齐 Java `Path.normalize()` 语义）。
    pub fn normalize(path: &Path) -> PathBuf {
        let mut out = PathBuf::new();
        for component in path.components() {
            match component {
                Component::ParentDir => {
                    out.pop();
                }
                Component::CurDir => {}
                Component::RootDir | Component::Prefix(_) | Component::Normal(_) => {
                    out.push(component);
                }
            }
        }
        out
    }

    /// 对齐 Java: `PathUtil.toAbsNormal(Path)`
    pub fn to_abs_normal(path: &Path) -> PathBuf {
        let abs = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join(path)
        };
        Self::normalize(&abs)
    }

    /// 对齐 Java: `PathUtil.mkParentDirs(Path)`
    pub fn mk_parent_dirs(path: &Path) -> io::Result<()> {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }
        Ok(())
    }

    /// 对齐 Java: `PathUtil.mkdir(Path)`
    pub fn mkdir(path: &Path) -> io::Result<PathBuf> {
        fs::create_dir_all(path)?;
        Ok(path.to_path_buf())
    }

    /// 对齐 Java: `PathUtil.isDirectory(Path)` / `isDirectory(Path, boolean)`
    pub fn is_directory(path: &Path) -> bool {
        path.is_dir()
    }

    /// 对齐 Java: `PathUtil.isFile(Path)`
    pub fn is_file(path: &Path) -> bool {
        path.is_file()
    }

    /// 对齐 Java: `PathUtil.exists(Path, boolean)`
    pub fn exists(path: &Path) -> bool {
        path.exists()
    }

    /// 对齐 Java: `PathUtil.isSymlink(Path)`
    pub fn is_symlink(path: &Path) -> bool {
        FileUtil::is_symlink(&path.to_string_lossy())
    }

    /// 对齐 Java: `PathUtil.isDirEmpty(Path)`
    pub fn is_dir_empty(path: &Path) -> io::Result<bool> {
        FileUtil::is_dir_empty(&path.to_string_lossy())
    }

    /// 对齐 Java: `PathUtil.isExistsAndNotDirectory(Path)`
    pub fn is_exists_and_not_directory(path: &Path) -> bool {
        Self::exists(path) && !Self::is_directory(path)
    }

    /// 对齐 Java: `PathUtil.equals(Path, Path)`
    pub fn equals(a: &Path, b: &Path) -> bool {
        Self::normalize(a) == Self::normalize(b)
    }

    /// 对齐 Java: `PathUtil.isSub(Path, Path)` — child 是否在 parent 之下。
    pub fn is_sub(parent: &Path, child: &Path) -> bool {
        let p = Self::to_abs_normal(parent);
        let c = Self::to_abs_normal(child);
        c.starts_with(&p) && c != p
    }

    /// 对齐 Java: `PathUtil.getName(Path)`
    pub fn name(path: &Path) -> &str {
        FileUtil::name(path)
    }

    /// 对齐 Java: `PathUtil.getMimeType(String)`
    pub fn mime_type(path: &Path) -> &'static str {
        FileUtil::mime_type(&path.to_string_lossy())
    }

    /// 对齐 Java: `PathUtil.getPathEle(Path, int)` — 按索引取路径段。
    pub fn path_ele(path: &Path, index: isize) -> Option<PathBuf> {
        let comps: Vec<_> = path
            .components()
            .filter(|c| matches!(c, Component::Normal(_)))
            .collect();
        if comps.is_empty() {
            return None;
        }
        let i = if index < 0 {
            comps.len() as isize + index
        } else {
            index
        };
        if i < 0 || i as usize >= comps.len() {
            None
        } else {
            Some(PathBuf::from(comps[i as usize].as_os_str()))
        }
    }

    /// 对齐 Java: `PathUtil.getLastPathEle(Path)`
    pub fn last_path_ele(path: &Path) -> Option<PathBuf> {
        Self::path_ele(path, -1)
    }

    /// 对齐 Java: `PathUtil.subPath(Path, int, int)`
    pub fn sub_path(path: &Path, from: usize, to: usize) -> PathBuf {
        let comps: Vec<_> = path
            .components()
            .filter(|c| matches!(c, Component::Normal(_)))
            .collect();
        let end = to.min(comps.len());
        let start = from.min(end);
        let mut out = PathBuf::new();
        for c in &comps[start..end] {
            out.push(c.as_os_str());
        }
        out
    }

    /// 对齐 Java: `PathUtil.getAttributes(Path)` — 简化为文件大小。
    pub fn size(path: &Path) -> u64 {
        FileUtil::size(path)
    }

    /// 对齐 Java: `PathUtil.copyFile(Path, Path)`
    pub fn copy_file(src: &Path, target: &Path) -> io::Result<PathBuf> {
        let target_path =
            if Self::is_directory(target) {
                target.join(src.file_name().ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "empty source name")
                })?)
            } else {
                target.to_path_buf()
            };
        Self::mk_parent_dirs(&target_path)?;
        fs::copy(src, &target_path)?;
        Ok(target_path)
    }

    /// 对齐 Java: `PathUtil.copy(Path, Path)`
    pub fn copy(src: &Path, target: &Path) -> io::Result<PathBuf> {
        if Self::is_directory(src) {
            let nested =
                target.join(src.file_name().ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "empty source name")
                })?);
            Self::copy_content(src, &nested)
        } else {
            Self::copy_file(src, target)
        }
    }

    /// 对齐 Java: `PathUtil.copyContent(Path, Path)`（递归拷贝目录内容）。
    pub fn copy_content(src: &Path, target: &Path) -> io::Result<PathBuf> {
        if src.is_file() {
            return Self::copy_file(src, target);
        }
        fs::create_dir_all(target)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let from = entry.path();
            let to = target.join(entry.file_name());
            if from.is_dir() {
                Self::copy_content(&from, &to)?;
            } else {
                Self::copy_file(&from, &to)?;
            }
        }
        Ok(target.to_path_buf())
    }

    /// 对齐 Java: `PathUtil.del(Path)`
    pub fn del(path: &Path) -> io::Result<()> {
        FileUtil::delete(&path.to_string_lossy())
    }

    /// 对齐 Java: `PathUtil.rename(Path, String, boolean)`
    pub fn rename(path: &Path, new_name: &str) -> io::Result<PathBuf> {
        let parent = path.parent().unwrap_or_else(|| Path::new("."));
        let dest = parent.join(new_name);
        fs::rename(path, &dest)?;
        Ok(dest)
    }

    /// 对齐 Java: `PathUtil.move(Path, Path, boolean)`
    pub fn move_path(src: &Path, target: &Path) -> io::Result<PathBuf> {
        let dest =
            if Self::is_directory(target) {
                target.join(src.file_name().ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "empty source name")
                })?)
            } else {
                target.to_path_buf()
            };
        Self::mk_parent_dirs(&dest)?;
        fs::rename(src, &dest)?;
        Ok(dest)
    }

    /// 对齐 Java: `PathUtil.moveContent(Path, Path, boolean)`
    pub fn move_content(src: &Path, target: &Path) -> io::Result<PathBuf> {
        Self::copy_content(src, target)?;
        if src.is_dir() {
            fs::remove_dir_all(src)?;
        } else {
            fs::remove_file(src)?;
        }
        Ok(target.to_path_buf())
    }

    /// 对齐 Java: `PathUtil.loopFiles(Path)` — 非递归列出文件。
    pub fn loop_files(path: &Path) -> io::Result<Vec<PathBuf>> {
        let mut out = Vec::new();
        if !path.is_dir() {
            if path.is_file() {
                out.push(path.to_path_buf());
            }
            return Ok(out);
        }
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let p = entry.path();
            if p.is_file() {
                out.push(p);
            }
        }
        Ok(out)
    }

    /// 对齐 Java: `PathUtil.walkFiles(Path, FileVisitor)` — 递归收集全部文件。
    pub fn walk_files(path: &Path) -> io::Result<Vec<PathBuf>> {
        let mut out = Vec::new();
        Self::walk_files_into(path, &mut out)?;
        Ok(out)
    }

    fn walk_files_into(path: &Path, out: &mut Vec<PathBuf>) -> io::Result<()> {
        if path.is_file() {
            out.push(path.to_path_buf());
            return Ok(());
        }
        if !path.is_dir() {
            return Ok(());
        }
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let p = entry.path();
            if p.is_dir() {
                Self::walk_files_into(&p, out)?;
            } else if p.is_file() {
                out.push(p);
            }
        }
        Ok(())
    }

    /// 对齐 Java: `PathUtil.readBytes(Path)`
    pub fn read_bytes(path: &Path) -> io::Result<Vec<u8>> {
        fs::read(path)
    }

    /// 对齐 Java: `PathUtil.getInputStream(Path)` — 打开只读文件。
    pub fn open_read(path: &Path) -> io::Result<fs::File> {
        fs::File::open(path)
    }

    /// 对齐 Java: `PathUtil.getUtf8Reader(Path)` / `getReader`
    pub fn utf8_reader(path: &Path) -> io::Result<BufReader<fs::File>> {
        Ok(BufReader::new(fs::File::open(path)?))
    }

    /// 对齐 Java: `PathUtil.getOutputStream(Path)`
    pub fn open_write(path: &Path) -> io::Result<fs::File> {
        Self::mk_parent_dirs(path)?;
        fs::File::create(path)
    }

    /// 对齐 Java: `PathUtil.createTempFile`
    pub fn create_temp_file() -> io::Result<PathBuf> {
        FileUtil::create_temp_file()
    }

    /// 将 reader 内容写入 path（辅助 FileUtil.writeFromStream）。
    pub fn write_from_reader<R: Read>(path: &Path, reader: &mut R) -> io::Result<u64> {
        let mut file = Self::open_write(path)?;
        io::copy(reader, &mut file)
    }

    /// 将 path 内容写入 writer（辅助 FileUtil.writeToStream）。
    pub fn write_to_writer<W: Write>(path: &Path, writer: &mut W) -> io::Result<u64> {
        let mut file = Self::open_read(path)?;
        io::copy(&mut file, writer)
    }
}
