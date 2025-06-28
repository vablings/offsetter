#![no_std]
pub extern crate paste;

#[macro_export]
/// Macro to place fields at specified offsets
macro_rules! offset {
    // Guarded munched exprs
    // output
    (@guard ($current_offset:expr) -> {$(#[$attr:meta])* $vis:vis struct $name:ident $(($amount:expr, $vis_field:vis $id:ident: $ty:ty))*}) =>    {

        $crate::paste::paste! {
            #[repr(C, packed)]
            $(#[$attr])* $vis struct $name { $([<_pad $id>]: [u8;$amount], $vis_field $id: $ty),* }
        }

    };

    // add more fields
    (@guard ($current_offset:expr, $offset:literal $vis_field:vis $id:ident: $ty:ty $(,)?) -> {$($output:tt)*}) => {
        offset!(@guard ($offset + core::mem::size_of::<$ty>()) -> {$($output)* ($offset - ($current_offset), $vis_field $id: $ty)});
    };

    (@guard ($current_offset:expr, $offset:literal $vis_field:vis $id:ident: $ty:ty, $($next:tt)+) -> {$($output:tt)*}) => {
        offset!(@guard ($offset + core::mem::size_of::<$ty>(), $($next)+) -> {$($output)* ($offset - ($current_offset), $vis_field $id: $ty)});
    };

    // Entry points
    ($(#[$attr:meta])* $vis:vis struct $struct_name:ident {$($input:tt)*}) => {
        offset!(@guard (0, $($input)*) -> {$(#[$attr])* $vis struct $struct_name});
        $crate::offset_checker!($struct_name {$($input)*});
    };
}

#[macro_export]
macro_rules! offset_debug {
    // Final struct
    (@build ($current_offset:expr) -> { $(#[$attr:meta])* $vis:vis struct $name:ident $(($pad_amount:expr, $vis_field:vis $id:ident : $ty:ty))* }) => {
        $crate::paste::paste! {
            #[repr(C, packed)]
            $(#[$attr])* $vis struct $name {
                $([<_pad_ $id>]: [u8; $pad_amount], $vis_field $id : $ty),*
            }
        }
    };

    // Add fields (offset must be in parentheses for clean parsing)
    (@build ($current_offset:expr, ($offset:expr) $vis_field:vis $id:ident : $ty:ty $(,)?) -> { $($output:tt)* }) => {
        offset_debug!(@build ($offset + core::mem::size_of::<$ty>()) -> {
            $($output)* ($offset - $current_offset, $vis_field $id : $ty)
        });
    };

    (@build ($current_offset:expr, ($offset:expr) $vis_field:vis $id:ident : $ty:ty, $($rest:tt)+) -> { $($output:tt)* }) => {
        offset_debug!(@build ($offset + core::mem::size_of::<$ty>(), $($rest)+) -> {
            $($output)* ($offset - $current_offset, $vis_field $id : $ty)
        });
    };

    // Entry
    ($(#[$attr:meta])* $vis:vis struct $name:ident { $($body:tt)* }) => {
        offset_debug!(@build (0, $($body)*) -> { $(#[$attr])* $vis struct $name });
    };
}

#[cfg(feature = "checked")]
#[macro_export]
macro_rules! offset_checker {
        ($struct_name:ident {$($offset:literal $vis_field:vis $id:ident: $ty:ty),* $(,)?}) => {
            $(const _: ()  = assert!(core::mem::offset_of!($struct_name, $id) == $offset);)*
        };

}

#[cfg(not(feature = "checked"))]
#[macro_export]
macro_rules! offset_checker {
        ($struct_name:ident {$($offset:literal $vis_field:vis $id:ident: $ty:ty),* $(,)?}) => {
        };

}
