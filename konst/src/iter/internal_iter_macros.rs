use crate::iter::ConstIntoIter;

use core::marker::PhantomData;

#[doc(hidden)]
#[macro_export]
macro_rules! iterator_shared {
    (
        is_forward = $is_forward:ident,
        $(is_copy = $is_copy:ident,)?
        $(is_drop = $is_drop:ident,)?
        item = $Item:ty,
        iter_forward = $Self:ty,
        $(iter_reversed = $Rev:path,)?
        next($self:ident) $next_block:block,
        $(next_back $next_back_block:block,)?
        fields = $fields:tt,
    ) => {
        $crate::__iterator_shared! {
            is_forward = $is_forward,
            is_copy = ($($is_copy)? true),
            is_drop = ($($is_drop)? false),
            item = $Item,
            iter_forward = $Self,
            $(iter_reversed = $Rev,)?
            next($self) $next_block,
            $(next_back $next_back_block,)?
            fields = $fields,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __iterator_shared {
    (
        is_forward = $is_forward:ident,
        is_copy = ($is_copy:ident $($_is_copy:tt)*),
        is_drop = ($is_drop:ident $($_is_drop:tt)*),
        item = $Item:ty,
        iter_forward = $Self:ty,
        $(iter_reversed = $Rev:path,)?
        next($self:ident) $next_block:block,
        $(next_back $next_back_block:block,)?
        fields = $fields:tt,
    ) => {
        $crate::__choose_alt! {($is_copy) {
            /// Creates a clone of this iterator
            pub const fn copy(&self) -> Self {
                $crate::__iterator_shared_copy!{self $fields}
            }
        }}

        $(
            $crate::__iterator_shared_rev!{
                rev = $crate::__choose!($is_forward $Rev $Self),
                is_drop = $is_drop,
                $fields
            }
        )?

        /// Advances the iterator and returns the next value.
        #[track_caller]
        pub const fn next(&mut $self) -> Option<$Item> {
            $crate::__choose!{
                $is_forward
                $next_block
                $($next_back_block)?
            }
        }

        $(
            /// Removes and returns an element from the end of the iterator.
            #[track_caller]
            pub const fn next_back(&mut $self) -> Option<$Item> {
                $crate::__choose!{
                    $is_forward
                    $next_back_block
                    $next_block
                }
            }
        )?
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __iterator_shared_copy {
    ( $self:ident { $( $field:ident $(.$copy_method:ident())? ),* $(,)?} ) => ({
        Self {
            $($field: $self.$field $(.$copy_method())?,)*
        }
    })
}

#[doc(hidden)]
#[macro_export]
macro_rules! __iterator_shared_rev {
    (
        rev = $Rev:ty,
        is_drop = $is_drop:ident,
        { $( $field:ident $(.$copy_method:ident())? ),* $(,)?}
    ) => (
        /// Reverses the iterator
        pub const fn rev(self) -> $Rev {
            $crate::__choose_alt! {($is_drop) {
                $crate::destructure!{Self {$($field),*} = self}
            } else {
                let Self {$($field),*} = self;
            }}

            type Type<T> = T;
            Type::<$Rev> {$($field),*}
        }
    )
}

#[doc(hidden)]
#[inline(always)]
pub const fn __get_item_ty<Iter>(_: &Iter) -> PhantomData<(Iter, Iter::Item)>
where
    Iter: ConstIntoIter,
{
    PhantomData
}

#[doc(hidden)]
#[inline(always)]
pub const fn __assert_item_ty<Iter>(_: &Iter::Item, _: PhantomData<(Iter, Iter::Item)>)
where
    Iter: ConstIntoIter,
{
}

#[doc(hidden)]
#[inline(always)]
pub const fn __items_needs_drop<Iter>(_: &Iter) -> bool
where
    Iter: ConstIntoIter,
{
    Iter::ITEMS_NEED_DROP
}

#[doc(hidden)]
pub const fn __infer_option_ty<T>(_: &Option<T>, _: &T) {}
