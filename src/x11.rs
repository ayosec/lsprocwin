use std::collections::{hash_map::Entry, HashMap};
use std::error::Error;

use xcb::{Atom, Window};

/// Value to indicate that the window is in all desktops.
///
/// https://specifications.freedesktop.org/wm-spec/latest/ar01s05.html#idm46075117284064
const ALL_DESKTOPS: u32 = 0xFFFFFFFF;

/// Collect values from `_NET_WM_DESKTOP`
pub fn window_desktops(
    windows: impl Iterator<Item = Window>,
) -> Result<HashMap<Window, Option<u32>>, Box<dyn Error>> {
    let (conn, _) = xcb::Connection::connect(None)?;

    let propname = xcb::intern_atom(&conn, false, "_NET_WM_DESKTOP")
        .get_reply()?
        .atom();
    let format = xcb::intern_atom(&conn, false, "CARDINAL")
        .get_reply()?
        .atom();

    let mut desktops = HashMap::new();
    for window in windows {
        if let Entry::Vacant(entry) = desktops.entry(window) {
            let desktop = match get_prop(&conn, window, propname, format) {
                Ok(self::ALL_DESKTOPS) => None,
                Ok(d) => Some(d),
                Err(_) => continue,
            };

            entry.insert(desktop);
        }
    }

    Ok(desktops)
}

fn get_prop<T>(
    conn: &xcb::Connection,
    window: Window,
    prop: Atom,
    format: Atom,
) -> Result<T, Box<dyn Error>>
where
    T: Copy,
{
    let size = std::mem::size_of::<T>() as u32;

    let reply =
        xcb::xproto::get_property(&conn, false, window, prop, format, 0, size).get_reply()?;

    if reply.value_len() > 0 {
        Ok(reply.value()[0])
    } else {
        Err(format!("Expected {} bytes, found {}", size, reply.value_len()).into())
    }
}
