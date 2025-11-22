pub trait EulerSim {
    // number of grid cells, even though most impls will store velocities at edges
    fn grid_size(&self) -> (usize, usize);
    fn velocity(&self, coord: (usize, usize)) -> Option<(f64, f64)>;
}