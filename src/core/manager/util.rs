use crate::{
    core::{
        data_types::Region,
        xconnection::{XClientConfig, XState, Xid},
    },
    Result,
};

pub(super) fn pad_region(region: &Region, gapless: bool, gap_px: u32, border_px: u32) -> Region {
    let gpx = if gapless { 0 } else { gap_px };
    let padding = 2 * (border_px + gpx);
    let (x, y, w, h) = region.values();

    // Check that the resulting size would not be zero or negative
    // Do not allow zero-size as this is chosen by the WM
    if w <= padding || h <= padding {
        warn!("not padding region to avoid integer underflow");
        return *region;
    }

    Region::new(x + gpx, y + gpx, w - padding, h - padding)
}

pub(super) fn position_floating_client<X>(
    conn: &X,
    id: Xid,
    screen_region: Region,
    border_px: u32,
) -> Result<()>
where
    X: XClientConfig + XState,
{
    let default_position = conn.client_geometry(id)?;
    let (mut x, mut y, w, h) = default_position.values();
    let (sx, sy, _, _) = screen_region.values();
    x = if x < sx { sx } else { x };
    y = if y < sy { sy } else { y };

    // Check that the resulting size would not be negative
    // Allow zero-size here as it is chosen by the client
    let reg = if w >= 2 * border_px && h >= 2 * border_px {
        Region::new(
            x + border_px,
            y + border_px,
            w - (2 * border_px),
            h - (2 * border_px),
        )
    } else {
        warn!("floating client too small {}", id);
        Region::new(x, y, w, h)
    };

    Ok(conn.position_client(id, reg, border_px, false)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_helpers::*;

    #[test]
    fn pad_region_centered() {
        let r = Region::new(0, 0, 200, 100);
        let g = 10;
        let b = 3;
        assert_eq!(pad_region(&r, false, g, b), Region::new(10, 10, 174, 74));
        assert_eq!(pad_region(&r, true, g, b), Region::new(0, 0, 194, 94));
    }

    #[test]
    fn pad_region_tiny() {
        let r = Region::new(0, 0, 3, 3);
        let g = 10;
        let b = 3;
        assert_eq!(pad_region(&r, false, g, b), r);
        assert_eq!(pad_region(&r, true, g, b), r);
    }

    #[test]
    fn position_floating() {
        let conn = TestXConn::new(1, vec![], vec![]);
        conn.position_client(0, Region::new(0, 0, 400, 300), 2, false)
            .unwrap();

        position_floating_client(&conn, 0, Region::default(), 2).unwrap();

        assert_eq!(
            conn.client_geometry(0).unwrap(),
            Region::new(2, 2, 396, 296)
        );
    }

    #[test]
    fn position_floating_tiny() {
        let conn = TestXConn::new(1, vec![], vec![]);
        conn.position_client(0, Region::new(0, 0, 4, 3), 2, false)
            .unwrap();

        position_floating_client(&conn, 0, Region::default(), 2).unwrap();

        assert_eq!(conn.client_geometry(0).unwrap(), Region::new(0, 0, 4, 3));
    }
}
