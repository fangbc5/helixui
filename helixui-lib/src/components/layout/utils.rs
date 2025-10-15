pub mod controlled {
    use dioxus::prelude::*;

    #[derive(Props, PartialEq, Clone)]
    pub struct Controlled<T: Clone + PartialEq + 'static> {
        #[props(optional)]
        pub value: Option<T>,
        #[props(optional)]
        pub default_value: Option<T>,
        #[props(optional)]
        pub on_change: Option<EventHandler<T>>,
    }

    impl<T: Clone + PartialEq + 'static> Controlled<T> {
        pub fn resolve<'a>(&self, inner: &'a T) -> T {
            self.value.clone().unwrap_or_else(|| inner.clone())
        }
        pub fn set_and_emit(&self, next: T, set_inner: impl FnOnce(T)) {
            if self.value.is_some() {
                if let Some(cb) = &self.on_change {
                    cb.call(next);
                }
            } else {
                set_inner(next.clone());
                if let Some(cb) = &self.on_change {
                    cb.call(next);
                }
            }
        }
    }
}
