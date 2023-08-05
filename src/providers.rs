//! Some common tile map providers.

use crate::mercator::TileId;

/// <https://www.openstreetmap.org/about>
pub fn openstreetmap(tile_id: TileId) -> String {
    format!(
        "https://tile.openstreetmap.org/{}/{}/{}.png",
        tile_id.zoom, tile_id.x, tile_id.y
    )
}

pub fn geoportal(tile_id: TileId) -> String {
    format!(
        "https://mapy.geoportal.gov.pl/wss/service/PZGIK/ORTO/WMTS/StandardResolution?\
        &SERVICE=WMTS\
        &REQUEST=GetTile\
        &VERSION=1.0.0\
        &LAYER=ORTOFOTOMAPA\
        &STYLE=default\
        &TILEMATRIXSET=EPSG:3857\
        &TILEMATRIX=EPSG:3857:{}\
        &TILEROW={}\
        &TILECOL={}\
        &FORMAT=image/png\
        &rfh=1",
        tile_id.zoom, tile_id.y, tile_id.x
    )
}
