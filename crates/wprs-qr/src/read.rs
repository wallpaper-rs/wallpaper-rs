pub fn read(luma: &[u8], width: usize, height: usize, stride: usize) -> Option<String> {
    let last = height.checked_sub(1)?.checked_mul(stride)?.checked_add(width)?;
    if width == 0 || stride < width || luma.len() < last {
        return None;
    }
    let mut image = rqrr::PreparedImage::prepare_from_greyscale(width, height, |x, y| luma[y * stride + x]);
    image.detect_grids().iter().find_map(|grid| grid.decode().ok().map(|(_, text)| text))
}
