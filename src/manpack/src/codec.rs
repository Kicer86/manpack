
#[no_mangle]
pub extern "C" fn compress(count: u64, data: &mut [u32]) -> Vec<u32>
{
    return data.to_vec();
}
