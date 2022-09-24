macro_rules! iterator_shared {
    (
        is_forward = $is_forward:ident,
        item = $Item:ty,
        iter_forward = $Self:ty,
        iter_reversed = $Rev:path,
        next = $next:ident,
        next_back = $next_back:ident,
        fields = {$($field:ident),*},
    ) => {
        /// Creates a clone of this iterator
        pub const fn copy(&self) -> Self {
            let Self{$($field),*} = *self;
            Self{$($field),*}
        }

        /// Reverses the iterator
        pub const fn rev(self) -> $Rev {
            let Self{$($field),*} = self;
            $Rev {$($field),*}
        }

        /// Advances the iterator and returns the next value.
        pub const fn next(mut self) -> Option<($Item, Self)> {
            macro_select!{$is_forward, $next, $next_back, self}
        }

        /// Removes and returns an element from the end of the iterator.
        pub const fn next_back(mut self) -> Option<($Item, Self)> {
            macro_select!{$is_forward, $next_back, $next, self}
        }
    };
}

macro_rules! macro_select {
    (true, $iftrue:ident, $iffalse:ident, $($args:tt)*) => {
        $iftrue!{$($args)*}
    };
    (false, $iftrue:ident, $iffalse:ident, $($args:tt)*) => {
        $iffalse!{$($args)*}
    };
}
