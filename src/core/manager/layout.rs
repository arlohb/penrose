use crate::{
    core::{
        config::Config,
        hooks::HookName,
        manager::{event::EventAction, util::pad_region},
        xconnection::XConn,
    },
    Result, WindowManager,
};

#[tracing::instrument(level = "trace", err)]
pub(super) fn layout_visible<X: XConn>(wm: &mut WindowManager<X>) -> Result<Vec<EventAction>> {
    wm.screens
        .visible_workspaces()
        .into_iter()
        .flat_map(|wix| apply_layout(wm, wix).transpose())
        .collect()
}

#[tracing::instrument(level = "debug", err)]
pub(super) fn apply_layout<X: XConn>(
    wm: &mut WindowManager<X>,
    wix: usize,
) -> Result<Option<EventAction>> {
    let (i, s) = match wm.screens.indexed_screen_for_workspace(wix) {
        Some((i, s)) => (i, s),
        None => return Ok(None),
    };

    let Config {
        show_bar,
        border_px,
        gap_px,
        ..
    } = wm.config;

    let (lc, aa) = wm.workspaces.get_arrange_actions(
        wix,
        s.region(show_bar),
        &wm.clients.clients_for_ids(&wm.workspaces[wix].client_ids()),
    )?;

    for (id, region) in aa.actions {
        trace!(id, ?region, "positioning client");
        if let Some(region) = region {
            let reg = pad_region(&region, lc.gapless, gap_px, border_px);
            wm.conn.position_client(id, reg, border_px, false)?;
            wm.clients.map_if_needed(id, &wm.conn)?;
        } else {
            wm.clients.unmap_if_needed(id, &wm.conn)?;
        }
    }

    for id in aa.floating {
        debug!(id, "mapping floating client above tiled");
        wm.conn.raise_client(id)?;
    }

    Ok(Some(EventAction::RunHook(HookName::LayoutApplied(wix, i))))
}
