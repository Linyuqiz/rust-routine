pub(crate) fn sum(vec: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for value in vec {
        // 防止数据溢出计算出错误的结果
        if u32::MAX - sum < *value {
            return None;
        }
        sum += value
    }
    return Some(sum);
}
