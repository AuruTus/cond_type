pub trait CondTrait<const COND: bool, T, F> {
    type CType;
    fn get_cval(self) -> Self::CType;
}

impl<T, F> CondTrait<true, T, F> for (T, F) {
    type CType = T;
    fn get_cval(self) -> Self::CType {
        self.0
    }
}

impl<T, F> CondTrait<false, T, F> for (T, F) {
    type CType = F;
    fn get_cval(self) -> Self::CType {
        self.1
    }
}

pub type CondType<const COND: bool, T, F> = <(T, F) as CondTrait<COND, T, F>>::CType;

pub fn get_cond_val<const COND: bool, T, F>(val_t: T, val_f: F) -> CondType<COND, T, F>
where
    (T, F): CondTrait<COND, T, F>,
{
    (val_t, val_f).get_cval()
}

macro_rules! choose {
    ($cond:expr, $val_t:expr, $val_f:expr) => {
        $crate::get_cond_val::<{ $cond }, _, _>($val_t, $val_f)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sized_struct() {
        struct ST;
        let _a = get_cond_val::<true, _, _>(ST {}, 1);
        let _a = get_cond_val::<false, _, _>(ST {}, 1);
    }

    #[test]
    fn test_unsized_struct() {
        trait Trt {
            fn test_dyn(&self);
        }

        struct ST;
        impl Trt for ST {
            fn test_dyn(&self) {}
        }

        let b: Box<dyn Trt> = Box::new(ST {});
        let c = "123456";
        let _a = get_cond_val::<true, _, _>(b, c);
        let _a = get_cond_val::<false, _, _>(_a, c);
    }

    #[test]
    fn test_const_expr() {
        struct ST;
        let _a = choose!(1 < 2, ST {}, 2);
        let _a = choose!(2 > 1, _a, ST {});
    }
}
