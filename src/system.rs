use crate::world::{
    resource::{Fetch, FetchMut, Resource},
    World,
};

pub trait System<'a> {
    type SystemData: SystemData<'a>;

    fn run_now(&mut self, world: &'a World) {
        self.run(SystemData::fetch(world))
    }

    fn run(&mut self, data: Self::SystemData);
}

pub trait SystemData<'a> {
    fn fetch(world: &'a World) -> Self;
}

impl<'a, T: Resource> SystemData<'a> for Fetch<'a, T> {
    fn fetch(world: &'a World) -> Self {
        world.resources.fetch().unwrap_or_else(|| {
            panic!(
                "{}: the resource does not exist.",
                std::any::type_name::<T>()
            )
        })
    }
}
impl<'a, T: Resource> SystemData<'a> for FetchMut<'a, T> {
    fn fetch(world: &'a World) -> Self {
        world.resources.fetch_mut().unwrap_or_else(|| {
            panic!(
                "{}: the resource does not exist.",
                std::any::type_name::<T>()
            )
        })
    }
}

mod impl_data {
    use super::SystemData;
    use crate::world::World;

    macro_rules! __impl_data {
        ($($elem:ident),*) => {
            impl<'a, $($elem),* ,> SystemData<'a> for ($($elem),*)
            where
                $($elem: SystemData<'a>),*
            {
                fn fetch(world: &'a World) -> Self {
                    ($(<$elem>::fetch(world)),*)
                }
            }
        };
    }
    macro_rules! impl_sys_data {
        ($p0:ident, $p1:ident) => {
            __impl_data! {$p0, $p1}
        };
        ($p0:ident, $($p:ident),*) => {
            __impl_data! {$p0, $($p),*}
            impl_sys_data!{$($p),*}
        };
    }

    impl_sys_data! {P15, P14, P13, P12, P11, P10, P9, P8, P7, P6, P5, P4, P3, P2, P1, P0}
}
