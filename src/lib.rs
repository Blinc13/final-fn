#![no_std]

#[macro_export]
macro_rules! final_fn {
    ($code:block) => {
        let r#final: final_fn::FinalFunc<_> = (|| $code).into();
    };
    ($code:expr) => {
        let r#final: final_fn::FinalFunc<_> = (|| $code).into();
    };
}



pub struct FinalFunc<T>
where
    T: Fn(),
{
    func: T,
}

impl<T> Drop for FinalFunc<T>
where
    T: Fn(),
{
    fn drop(&mut self) {
        (self.func)()
    }
}

impl<T> From<T> for FinalFunc<T>
where
    T: Fn(),
{
    fn from(value: T) -> Self {
        Self { func: value }
    }
}
