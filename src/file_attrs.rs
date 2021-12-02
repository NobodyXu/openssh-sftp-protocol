use super::{constants, extensions::Extensions, seq_iter::SeqIter, visitor::impl_visitor};

use serde::Serialize;

#[derive(Debug, Default, Clone, Serialize)]
pub struct FileAttrs {
    flags: u32,

    /// present only if flag SSH_FILEXFER_ATTR_SIZE
    size: Option<u64>,

    /// present only if flag SSH_FILEXFER_ATTR_UIDGID
    ///
    /// Stores uid and gid.
    id: Option<(u32, u32)>,

    /// present only if flag SSH_FILEXFER_ATTR_PERMISSIONS
    permissions: Option<u32>,

    /// present only if flag SSH_FILEXFER_ATTR_ACMODTIME
    ///
    /// Stores atime and mtime.
    time: Option<(u32, u32)>,

    /// present only if flag SSH_FILEXFER_ATTR_EXTENDED
    extensions: Option<Extensions>,
}

impl FileAttrs {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_size(&mut self, size: u64) {
        self.flags |= constants::SSH_FILEXFER_ATTR_SIZE;
        self.size = Some(size);
    }

    pub fn set_uid(&mut self, uid: u32, gid: u32) {
        self.flags |= constants::SSH_FILEXFER_ATTR_UIDGID;
        self.id = Some((uid, gid));
    }

    pub fn set_permissions(&mut self, permissions: u32) {
        self.flags |= constants::SSH_FILEXFER_ATTR_PERMISSIONS;
        self.permissions = Some(permissions);
    }

    pub fn set_time(&mut self, atime: u32, mtime: u32) {
        self.flags |= constants::SSH_FILEXFER_ATTR_ACMODTIME;
        self.time = Some((atime, mtime));
    }

    pub fn set_extensions(&mut self, extensions: Extensions) {
        self.flags |= constants::SSH_FILEXFER_ATTR_EXTENDED;
        self.extensions = Some(extensions);
    }

    pub fn get_size(&self) -> Option<u64> {
        self.size
    }

    /// Return uid and gid
    pub fn get_id(&self) -> Option<(u32, u32)> {
        self.id
    }

    pub fn get_permissions(&self) -> Option<u32> {
        self.permissions
    }

    /// Return atime and mtime
    pub fn get_time(&self) -> Option<(u32, u32)> {
        self.time
    }

    pub fn get_extensions(&self) -> &Option<Extensions> {
        &self.extensions
    }

    pub fn get_extensions_mut(&mut self) -> &mut Option<Extensions> {
        &mut self.extensions
    }
}

impl_visitor!(FileAttrs, FileAttrVisitor, "File attributes", seq, {
    let mut iter = SeqIter::new(seq);
    let mut attrs = FileAttrs::default();

    let flags = iter.get_next()?;

    attrs.flags = flags;

    if (flags & constants::SSH_FILEXFER_ATTR_SIZE) != 0 {
        attrs.size = Some(iter.get_next()?);
    }
    if (flags & constants::SSH_FILEXFER_ATTR_UIDGID) != 0 {
        attrs.id = Some((iter.get_next()?, iter.get_next()?));
    }
    if (flags & constants::SSH_FILEXFER_ATTR_PERMISSIONS) != 0 {
        attrs.permissions = Some(iter.get_next()?);
    }
    if (flags & constants::SSH_FILEXFER_ATTR_ACMODTIME) != 0 {
        attrs.time = Some((iter.get_next()?, iter.get_next()?));
    }
    if (flags & constants::SSH_FILEXFER_ATTR_EXTENDED) != 0 {
        attrs.extensions = Some(iter.get_next()?);
    }

    Ok(attrs)
});
