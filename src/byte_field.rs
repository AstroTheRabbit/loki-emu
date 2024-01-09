#[macro_export]
macro_rules! sum_lengths {
    ($($length:expr),+) => {{
        let mut sum = 0;
        $(sum += $length;)+
        sum
    }};
}

#[macro_export]
macro_rules! byte_field {
    ($(#[$meta:meta])* $struct_name:ident; $($field:ident: $length:expr),+ $(,)?) => {
        use crate::sum_lengths;

        #[derive(Debug)]
        $(#[$meta])*
        pub struct $struct_name {
            $(
                pub $field: [u8; $length],
            )+
        }

        impl $struct_name {
            fn as_bytes(&self) -> &[u8] {
                unsafe {
                    let base_ptr = self as *const Self as *const u8;
                    std::slice::from_raw_parts(base_ptr, sum_lengths!($($length),+))
                }
            }

            fn as_mut_bytes(&mut self) -> &mut [u8] {
                unsafe {
                    let base_ptr = self as *mut Self as *mut u8;
                    std::slice::from_raw_parts_mut(base_ptr, sum_lengths!($($length),+))
                }
            }
        }

        impl std::ops::Index<usize> for $struct_name {
            type Output = u8;

            fn index(&self, index: usize) -> &Self::Output {
                &self.as_bytes()[index]
            }
        }

        impl std::ops::IndexMut<usize> for $struct_name {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.as_mut_bytes()[index]
            }
        }

        impl From<[u8; sum_lengths!($($length),+)]> for $struct_name {
            fn from(bytes: [u8; sum_lengths!($($length),+)]) -> Self {
                let mut offset = 0;
                $(
                    let $field = {
                        let start = offset;
                        offset += $length;
                        let end = offset;
                        let mut array = [0; $length];
                        array.copy_from_slice(&bytes[start..end]);
                        array
                    };
                )+
                $struct_name { $($field),+ }
            }
        }
    };
}
