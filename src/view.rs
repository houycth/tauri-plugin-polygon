use crate::error::*;
use crate::polygon::*;
use crate::statics::*;
use crate::utils;

pub(crate) fn register(id: PolygonId) -> Result<()> {
    let mut ids = REGISTERED_IDS.get().ok_or(Error::NotInitialized)?.write()?;
    if ids.contains(&id) {
        return Err(Error::PolygonExists(id));
    }
    ids.insert(id.clone());

    let polygon = Polygon::default(&id);

    let mut registered = REGISTERED_POLYGON
        .get()
        .ok_or(Error::NotInitialized)?
        .write()?;
    registered.insert(id, polygon);

    Ok(())
}

pub(crate) fn register_all(ids: Vec<PolygonId>) -> Result<()> {
    let polygons = ids
        .iter()
        .map(|id| Polygon::default(id))
        .collect::<Vec<Polygon>>();

    let mut registered = REGISTERED_POLYGON
        .get()
        .ok_or(Error::NotInitialized)?
        .write()?;

    for polygon in polygons {
        registered.insert(polygon.id().into(), polygon);
    }

    let mut registered_ids = REGISTERED_IDS.get().ok_or(Error::NotInitialized)?.write()?;

    for id in ids.iter() {
        registered_ids.insert(id.into());
    }

    Ok(())
}

pub(crate) fn remove(id: &str) -> Result<()> {
    let mut ids = REGISTERED_IDS.get().ok_or(Error::NotInitialized)?.write()?;
    if !ids.contains(&id.to_string()) {
        return Err(Error::PolygonNotFound(id.to_string()));
    }

    let mut registered = REGISTERED_POLYGON
        .get()
        .ok_or(Error::NotInitialized)?
        .write()?;

    if let Some(polygon) = registered.remove(id) {
        polygon.distroy();
    }

    ids.retain(|i| i != id);

    Ok(())
}

pub(crate) fn clear() -> Result<()> {
    let mut registered = REGISTERED_POLYGON
        .get()
        .ok_or(Error::NotInitialized)?
        .write()?;

    for (_, polygon) in registered.iter_mut() {
        polygon.distroy();
    }

    registered.clear();

    let mut ids = REGISTERED_IDS.get().ok_or(Error::NotInitialized)?.write()?;
    ids.clear();

    Ok(())
}

pub(crate) fn update(id: &str, points: &[(f64, f64)]) -> Result<()> {
    if points.len() < 3 {
        return Err(Error::PointsNotEnough(points.len()));
    }

    let registered = REGISTERED_POLYGON
        .get()
        .ok_or(Error::NotInitialized)?
        .read()?;

    if let Some(polygon) = registered.get(id) {
        polygon.set_points(points);
        Ok(())
    } else {
        Err(Error::PolygonNotFound(id.to_string()))
    }
}

pub(crate) fn hide(id: &str) -> Result<()> {
    let registered = REGISTERED_POLYGON
        .get()
        .ok_or(Error::NotInitialized)?
        .read()?;

    if let Some(polygon) = registered.get(id) {
        polygon.hide();
        Ok(())
    } else {
        Err(Error::PolygonNotFound(id.to_string()))
    }
}

pub(crate) fn show(id: &str) -> Result<()> {
    let registered = REGISTERED_POLYGON
        .get()
        .ok_or(Error::NotInitialized)?
        .read()?;

    if let Some(polygon) = registered.get(id) {
        polygon.show();
        Ok(())
    } else {
        Err(Error::PolygonNotFound(id.to_string()))
    }
}

pub(crate) fn cursor_in() -> Result<Vec<PolygonId>> {
    let mut v = Vec::new();
    let registered = REGISTERED_POLYGON
        .get()
        .ok_or(Error::NotInitialized)?
        .read()?;

    for (id, polygon) in registered.iter() {
        if polygon.cursor_in() {
            v.push(id.clone());
        }
    }
    Ok(v)
}

pub(crate) fn pos_contained(polygon: &Polygon, x: f64, y: f64) -> bool {
    if polygon.display() {
        utils::is_point_in_polygon(&polygon.points(), (x, y))
    } else {
        false
    }
}
