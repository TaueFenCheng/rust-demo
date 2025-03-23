pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // cargo test -- --ignored
    // 运行被忽略的测试

    #[test]
    #[ignore] // 忽略某个测试
    // #[should_panic] // 是否需要触发恐慌
    // #[should_panic(expected="error")] // 是否需要触发恐慌
    fn test_multiple(){
        let a = add(2,3);
        assert_eq!(a,5)   
        // assert_ !(a,5) // 不相等
    }
}
