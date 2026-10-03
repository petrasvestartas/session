pub fn gpu<'a>(rows: impl IntoIterator<Item = &'a crate::gpu_mesh::GpuMesh>) -> [u64; 4] {
    let mut geometry = HashSet::new();
    let mut count = [0; 4];
    for row in rows {
        if geometry.insert(Rc::as_ptr(&row.geometry)) {
            count[0] += 1; count[1] += row.geometry.allocated_bytes();
        }
        count[2] += 1; count[3] += row.settings_bytes();
    }
    count
}

#[cfg(test)]
mod tests {
