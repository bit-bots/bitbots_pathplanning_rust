use map::{ObstacleMap, ObstacleMapConfig};
use obstacle::RoundObstacle;
use pyo3::prelude::*;

mod map;
mod obstacle;
mod planner;

#[pymodule]
fn bitbots_rust_nav(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RoundObstacle>()?;
    m.add_class::<ObstacleMap>()?;
    m.add_class::<ObstacleMapConfig>()?;
    Ok(())
}
