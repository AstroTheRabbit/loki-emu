
#[macro_export]
macro_rules! byte_field {
    (
        $(#[$struct_attr:meta])*
        $struct_vis:vis $struct_name:ident;
        $($field_vis:vis $field_name:ident: $length:expr),* $(,)?
    ) => {
        $(#[$struct_attr])*
        $struct_vis struct $struct_name {
            $(
                $field_vis $field_name: [u8; $length],
            )*
        }

        impl $struct_name {
            pub fn new_empty() -> Self {
                Self {
                    $(
                        $field_name: [0; $length],
                    )*
                }
            }
        }

        impl std::ops::Index<usize> for $struct_name {
            type Output = u8;

            fn index(&self, mut index: usize) -> &Self::Output {
                $(
                    if index < $length {
                        return &self.$field_name[index];
                    } else {
                        index -= $length;
                    }

                )*
                panic!("Index out of bounds: {}", index);
            }
        }

        impl std::ops::IndexMut<usize> for $struct_name {
            fn index_mut(&mut self, mut index: usize) -> &mut Self::Output {
                $(
                    if index < $length {
                        return &mut self.$field_name[index];
                    } else {
                        index -= $length;
                    }

                )*
                panic!("Index out of bounds: {}", index);
            }
        }

        impl From<[u8; 0 $(+ $length)*]> for $struct_name {
            fn from(value: [u8; 0 $(+ $length)*]) -> Self {
                let mut s = Self::new_empty();
                for i in 0..value.len() {
                    s[i] = value[i];
                }
                return s;
            }
        }
    };
}