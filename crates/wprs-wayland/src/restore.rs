use crate::backdrop::Original;
use crate::x11::{Rect, X11};
use x11rb::connection::Connection as _;
use x11rb::protocol::xproto::*;
use x11rb::wrapper::ConnectionExt as _;

impl X11 {
    fn exists(&self, pixmap: u32) -> bool {
        self.conn.get_geometry(pixmap).ok().is_some_and(|c| c.reply().is_ok())
    }

    fn remembered(&self) -> Option<Original> {
        *self.before.lock().unwrap_or_else(|e| e.into_inner())
    }

    pub(crate) fn remember(&self) {
        let mut before = self.before.lock().unwrap_or_else(|e| e.into_inner());
        if before.is_none() {
            match self.look() {
                Ok(original) => *before = Some(original),
                Err(e) => tracing::debug!("cannot remember the root window background: {e}"),
            }
        }
    }

    fn look(&self) -> anyhow::Result<Original> {
        let (c, a, root) = (&self.conn, &self.atoms, self.root);
        let rootpmap = self.property(root, a._XROOTPMAP_ID)?.first().copied();
        let esetroot = self.property(root, a.ESETROOT_PMAP_ID)?.first().copied();
        let geometry = c.get_geometry(root)?.reply()?;
        let (width, height) = (geometry.width, geometry.height);
        let snapshot = c.generate_id()?;
        c.create_pixmap(geometry.depth, snapshot, root, width, height)?;
        let all = [Rectangle { x: 0, y: 0, width, height }];
        c.poly_fill_rectangle(snapshot, self.gc, &all)?;
        c.copy_area(root, snapshot, self.gc, 0, 0, 0, 0, width, height)?;
        c.flush()?;
        Ok(Original { rootpmap, esetroot, snapshot: Some(snapshot) })
    }

    pub(crate) fn release_area(&self, area: Rect) {
        if let Err(e) = self.give_back(area) {
            tracing::debug!("cannot give a screen area back to the root window: {e}");
        }
    }

    fn give_back(&self, area: Rect) -> anyhow::Result<()> {
        let held = self.pixmap.lock().unwrap_or_else(|e| e.into_inner());
        let Some((ours, _)) = *held else { return Ok(()) };
        let (c, root) = (&self.conn, self.root);
        let before = self.remembered().unwrap_or_default();
        let (x, y, width, height) = (area.x as i16, area.y as i16, area.width as u16, area.height as u16);
        let source = before.snapshot.or(before.rootpmap).filter(|p| self.exists(*p));
        match source {
            Some(source) => {
                c.copy_area(source, ours, self.gc, x, y, x, y, width, height)?;
            }
            None => {
                c.poly_fill_rectangle(ours, self.gc, &[Rectangle { x, y, width, height }])?;
            }
        }
        c.clear_area(false, root, x, y, width, height)?;
        Ok(c.flush()?)
    }

    pub(crate) fn restore_root(&self) {
        if let Err(e) = self.hand_back() {
            tracing::debug!("cannot restore the root window background: {e}");
        }
    }

    fn hand_back(&self) -> anyhow::Result<()> {
        let before = self.before.lock().unwrap_or_else(|e| e.into_inner()).take();
        let ours = self.pixmap.lock().unwrap_or_else(|e| e.into_inner()).take().map(|(p, _)| p);
        let (c, a, root) = (&self.conn, &self.atoms, self.root);
        let Original { rootpmap, esetroot, snapshot } = before.unwrap_or_default();
        if let Some(ours) = ours {
            let current = self.property(root, a._XROOTPMAP_ID)?.first().copied();
            if current == Some(ours) {
                self.put_back(rootpmap, esetroot, snapshot)?;
            }
            c.kill_client(ours)?;
        }
        if let Some(snapshot) = snapshot {
            c.free_pixmap(snapshot)?;
        }
        Ok(c.flush()?)
    }

    fn put_back(
        &self,
        rootpmap: Option<u32>,
        esetroot: Option<u32>,
        snapshot: Option<u32>,
    ) -> anyhow::Result<()> {
        let (c, a, root) = (&self.conn, &self.atoms, self.root);
        let property = |atom, value: Option<u32>| match value.filter(|p| self.exists(*p)) {
            Some(pixmap) => {
                c.change_property32(PropMode::REPLACE, root, atom, AtomEnum::PIXMAP, &[pixmap]).map(drop)
            }
            None => c.delete_property(root, atom).map(drop),
        };
        let paint = match (rootpmap.filter(|p| self.exists(*p)), snapshot) {
            (Some(pixmap), _) | (None, Some(pixmap)) => {
                ChangeWindowAttributesAux::new().background_pixmap(pixmap)
            }
            (None, None) => ChangeWindowAttributesAux::new().background_pixel(self.black),
        };
        c.change_window_attributes(root, &paint)?;
        property(a._XROOTPMAP_ID, rootpmap)?;
        property(a.ESETROOT_PMAP_ID, esetroot)?;
        c.clear_area(false, root, 0, 0, 0, 0)?;
        Ok(())
    }
}
