pub enum WorldResource {}
pub trait HostWorldResource {
    fn new(&mut self) -> wasmtime::component::Resource<WorldResource>;
    fn foo(&mut self, self_: wasmtime::component::Resource<WorldResource>) -> ();
    fn static_foo(&mut self) -> ();
    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<WorldResource>,
    ) -> wasmtime::Result<()>;
}
pub struct TheWorld {
    interface1: exports::foo::foo::uses_resource_transitively::Guest,
    some_world_func2: wasmtime::component::Func,
}
pub trait TheWorldImports: HostWorldResource {
    fn some_world_func(&mut self) -> wasmtime::component::Resource<WorldResource>;
}
const _: () = {
    #[allow(unused_imports)]
    use wasmtime::component::__internal::anyhow;
    impl TheWorld {
        pub fn add_to_linker<T, U>(
            linker: &mut wasmtime::component::Linker<T>,
            get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
        ) -> wasmtime::Result<()>
        where
            U: foo::foo::resources::Host + foo::foo::long_use_chain1::Host
                + foo::foo::long_use_chain2::Host + foo::foo::long_use_chain3::Host
                + foo::foo::long_use_chain4::Host
                + foo::foo::transitive_interface_with_resource::Host + TheWorldImports,
        {
            foo::foo::resources::add_to_linker(linker, get)?;
            foo::foo::long_use_chain1::add_to_linker(linker, get)?;
            foo::foo::long_use_chain2::add_to_linker(linker, get)?;
            foo::foo::long_use_chain3::add_to_linker(linker, get)?;
            foo::foo::long_use_chain4::add_to_linker(linker, get)?;
            foo::foo::transitive_interface_with_resource::add_to_linker(linker, get)?;
            Self::add_root_to_linker(linker, get)?;
            Ok(())
        }
        pub fn add_root_to_linker<T, U>(
            linker: &mut wasmtime::component::Linker<T>,
            get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
        ) -> wasmtime::Result<()>
        where
            U: TheWorldImports,
        {
            let mut linker = linker.root();
            linker
                .resource(
                    "world-resource",
                    wasmtime::component::ResourceType::host::<WorldResource>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        HostWorldResource::drop(
                            get(store.data_mut()),
                            wasmtime::component::Resource::new_own(rep),
                        )
                    },
                )?;
            linker
                .func_wrap(
                    "[constructor]world-resource",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = get(caller.data_mut());
                        let r = HostWorldResource::new(host);
                        Ok((r,))
                    },
                )?;
            linker
                .func_wrap(
                    "[method]world-resource.foo",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<WorldResource>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostWorldResource::foo(host, arg0);
                        Ok(r)
                    },
                )?;
            linker
                .func_wrap(
                    "[static]world-resource.static-foo",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = get(caller.data_mut());
                        let r = HostWorldResource::static_foo(host);
                        Ok(r)
                    },
                )?;
            linker
                .func_wrap(
                    "some-world-func",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = get(caller.data_mut());
                        let r = TheWorldImports::some_world_func(host);
                        Ok((r,))
                    },
                )?;
            Ok(())
        }
        /// Instantiates the provided `module` using the specified
        /// parameters, wrapping up the result in a structure that
        /// translates between wasm and the host.
        pub fn instantiate<T>(
            mut store: impl wasmtime::AsContextMut<Data = T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<T>,
        ) -> wasmtime::Result<(Self, wasmtime::component::Instance)> {
            let instance = linker.instantiate(&mut store, component)?;
            Ok((Self::new(store, &instance)?, instance))
        }
        /// Instantiates a pre-instantiated module using the specified
        /// parameters, wrapping up the result in a structure that
        /// translates between wasm and the host.
        pub fn instantiate_pre<T>(
            mut store: impl wasmtime::AsContextMut<Data = T>,
            instance_pre: &wasmtime::component::InstancePre<T>,
        ) -> wasmtime::Result<(Self, wasmtime::component::Instance)> {
            let instance = instance_pre.instantiate(&mut store)?;
            Ok((Self::new(store, &instance)?, instance))
        }
        /// Low-level creation wrapper for wrapping up the exports
        /// of the `instance` provided in this structure of wasm
        /// exports.
        ///
        /// This function will extract exports from the `instance`
        /// defined within `store` and wrap them all up in the
        /// returned structure which can be used to interact with
        /// the wasm module.
        pub fn new(
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<Self> {
            let mut store = store.as_context_mut();
            let mut exports = instance.exports(&mut store);
            let mut __exports = exports.root();
            let interface1 = exports::foo::foo::uses_resource_transitively::Guest::new(
                &mut __exports
                    .instance("foo:foo/uses-resource-transitively")
                    .ok_or_else(|| {
                        anyhow::anyhow!(
                            "exported instance `foo:foo/uses-resource-transitively` not present"
                        )
                    })?,
            )?;
            let some_world_func2 = *__exports
                .typed_func::<
                    (),
                    (wasmtime::component::Resource<WorldResource>,),
                >("some-world-func2")?
                .func();
            Ok(TheWorld {
                interface1,
                some_world_func2,
            })
        }
        pub fn call_some_world_func2<S: wasmtime::AsContextMut>(
            &self,
            mut store: S,
        ) -> wasmtime::Result<wasmtime::component::Resource<WorldResource>> {
            let callee = unsafe {
                wasmtime::component::TypedFunc::<
                    (),
                    (wasmtime::component::Resource<WorldResource>,),
                >::new_unchecked(self.some_world_func2)
            };
            let (ret0,) = callee.call(store.as_context_mut(), ())?;
            callee.post_return(store.as_context_mut())?;
            Ok(ret0)
        }
        pub fn foo_foo_uses_resource_transitively(
            &self,
        ) -> &exports::foo::foo::uses_resource_transitively::Guest {
            &self.interface1
        }
    }
};
pub mod foo {
    pub mod foo {
        #[allow(clippy::all)]
        pub mod resources {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::anyhow;
            pub enum Bar {}
            pub trait HostBar {
                fn new(&mut self) -> wasmtime::component::Resource<Bar>;
                fn static_a(&mut self) -> u32;
                fn method_a(&mut self, self_: wasmtime::component::Resource<Bar>) -> u32;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<Bar>,
                ) -> wasmtime::Result<()>;
            }
            #[derive(wasmtime::component::ComponentType)]
            #[derive(wasmtime::component::Lift)]
            #[derive(wasmtime::component::Lower)]
            #[component(record)]
            pub struct NestedOwn {
                #[component(name = "nested-bar")]
                pub nested_bar: wasmtime::component::Resource<Bar>,
            }
            impl core::fmt::Debug for NestedOwn {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("NestedOwn")
                        .field("nested-bar", &self.nested_bar)
                        .finish()
                }
            }
            const _: () = {
                assert!(
                    4 == < NestedOwn as wasmtime::component::ComponentType >::SIZE32
                );
                assert!(
                    4 == < NestedOwn as wasmtime::component::ComponentType >::ALIGN32
                );
            };
            #[derive(wasmtime::component::ComponentType)]
            #[derive(wasmtime::component::Lift)]
            #[derive(wasmtime::component::Lower)]
            #[component(record)]
            pub struct NestedBorrow {
                #[component(name = "nested-bar")]
                pub nested_bar: wasmtime::component::Resource<Bar>,
            }
            impl core::fmt::Debug for NestedBorrow {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("NestedBorrow")
                        .field("nested-bar", &self.nested_bar)
                        .finish()
                }
            }
            const _: () = {
                assert!(
                    4 == < NestedBorrow as wasmtime::component::ComponentType >::SIZE32
                );
                assert!(
                    4 == < NestedBorrow as wasmtime::component::ComponentType >::ALIGN32
                );
            };
            pub type SomeHandle = wasmtime::component::Resource<Bar>;
            const _: () = {
                assert!(
                    4 == < SomeHandle as wasmtime::component::ComponentType >::SIZE32
                );
                assert!(
                    4 == < SomeHandle as wasmtime::component::ComponentType >::ALIGN32
                );
            };
            pub trait Host: HostBar {
                fn bar_own_arg(&mut self, x: wasmtime::component::Resource<Bar>) -> ();
                fn bar_borrow_arg(
                    &mut self,
                    x: wasmtime::component::Resource<Bar>,
                ) -> ();
                fn bar_result(&mut self) -> wasmtime::component::Resource<Bar>;
                fn tuple_own_arg(
                    &mut self,
                    x: (wasmtime::component::Resource<Bar>, u32),
                ) -> ();
                fn tuple_borrow_arg(
                    &mut self,
                    x: (wasmtime::component::Resource<Bar>, u32),
                ) -> ();
                fn tuple_result(&mut self) -> (wasmtime::component::Resource<Bar>, u32);
                fn option_own_arg(
                    &mut self,
                    x: Option<wasmtime::component::Resource<Bar>>,
                ) -> ();
                fn option_borrow_arg(
                    &mut self,
                    x: Option<wasmtime::component::Resource<Bar>>,
                ) -> ();
                fn option_result(
                    &mut self,
                ) -> Option<wasmtime::component::Resource<Bar>>;
                fn result_own_arg(
                    &mut self,
                    x: Result<wasmtime::component::Resource<Bar>, ()>,
                ) -> ();
                fn result_borrow_arg(
                    &mut self,
                    x: Result<wasmtime::component::Resource<Bar>, ()>,
                ) -> ();
                fn result_result(
                    &mut self,
                ) -> Result<wasmtime::component::Resource<Bar>, ()>;
                fn list_own_arg(
                    &mut self,
                    x: wasmtime::component::__internal::Vec<
                        wasmtime::component::Resource<Bar>,
                    >,
                ) -> ();
                fn list_borrow_arg(
                    &mut self,
                    x: wasmtime::component::__internal::Vec<
                        wasmtime::component::Resource<Bar>,
                    >,
                ) -> ();
                fn list_result(
                    &mut self,
                ) -> wasmtime::component::__internal::Vec<
                    wasmtime::component::Resource<Bar>,
                >;
                fn record_own_arg(&mut self, x: NestedOwn) -> ();
                fn record_borrow_arg(&mut self, x: NestedBorrow) -> ();
                fn record_result(&mut self) -> NestedOwn;
                fn func_with_handle_typedef(&mut self, x: SomeHandle) -> ();
            }
            pub fn add_to_linker<T, U>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wasmtime::Result<()>
            where
                U: Host,
            {
                let mut inst = linker.instance("foo:foo/resources")?;
                inst.resource(
                    "bar",
                    wasmtime::component::ResourceType::host::<Bar>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        HostBar::drop(
                            get(store.data_mut()),
                            wasmtime::component::Resource::new_own(rep),
                        )
                    },
                )?;
                inst.func_wrap(
                    "[constructor]bar",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = get(caller.data_mut());
                        let r = HostBar::new(host);
                        Ok((r,))
                    },
                )?;
                inst.func_wrap(
                    "[static]bar.static-a",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = get(caller.data_mut());
                        let r = HostBar::static_a(host);
                        Ok((r,))
                    },
                )?;
                inst.func_wrap(
                    "[method]bar.method-a",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<Bar>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostBar::method_a(host, arg0);
                        Ok((r,))
                    },
                )?;
                inst.func_wrap(
                    "bar-own-arg",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<Bar>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = Host::bar_own_arg(host, arg0);
                        Ok(r)
                    },
                )?;
                inst.func_wrap(
                    "bar-borrow-arg",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<Bar>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = Host::bar_borrow_arg(host, arg0);
                        Ok(r)
                    },
                )?;
                inst.func_wrap(
                    "bar-result",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = get(caller.data_mut());
                        let r = Host::bar_result(host);
                        Ok((r,))
                    },
                )?;
                inst.func_wrap(
                    "tuple-own-arg",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): ((wasmtime::component::Resource<Bar>, u32),)|
                    {
                        let host = get(caller.data_mut());
                        let r = Host::tuple_own_arg(host, arg0);
                        Ok(r)
                    },
                )?;
                inst.func_wrap(
                    "tuple-borrow-arg",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): ((wasmtime::component::Resource<Bar>, u32),)|
                    {
                        let host = get(caller.data_mut());
                        let r = Host::tuple_borrow_arg(host, arg0);
                        Ok(r)
                    },
                )?;
                inst.func_wrap(
                    "tuple-result",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = get(caller.data_mut());
                        let r = Host::tuple_result(host);
                        Ok((r,))
                    },
                )?;
                inst.func_wrap(
                    "option-own-arg",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (Option<wasmtime::component::Resource<Bar>>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = Host::option_own_arg(host, arg0);
                        Ok(r)
                    },
                )?;
                inst.func_wrap(
                    "option-borrow-arg",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (Option<wasmtime::component::Resource<Bar>>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = Host::option_borrow_arg(host, arg0);
                        Ok(r)
                    },
                )?;
                inst.func_wrap(
                    "option-result",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = get(caller.data_mut());
                        let r = Host::option_result(host);
                        Ok((r,))
                    },
                )?;
                inst.func_wrap(
                    "result-own-arg",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (Result<wasmtime::component::Resource<Bar>, ()>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = Host::result_own_arg(host, arg0);
                        Ok(r)
                    },
                )?;
                inst.func_wrap(
                    "result-borrow-arg",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (Result<wasmtime::component::Resource<Bar>, ()>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = Host::result_borrow_arg(host, arg0);
                        Ok(r)
                    },
                )?;
                inst.func_wrap(
                    "result-result",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = get(caller.data_mut());
                        let r = Host::result_result(host);
                        Ok((r,))
                    },
                )?;
                inst.func_wrap(
                    "list-own-arg",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (
                            arg0,
                        ): (
                            wasmtime::component::__internal::Vec<
                                wasmtime::component::Resource<Bar>,
                            >,
                        )|
                    {
                        let host = get(caller.data_mut());
                        let r = Host::list_own_arg(host, arg0);
                        Ok(r)
                    },
                )?;
                inst.func_wrap(
                    "list-borrow-arg",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (
                            arg0,
                        ): (
                            wasmtime::component::__internal::Vec<
                                wasmtime::component::Resource<Bar>,
                            >,
                        )|
                    {
                        let host = get(caller.data_mut());
                        let r = Host::list_borrow_arg(host, arg0);
                        Ok(r)
                    },
                )?;
                inst.func_wrap(
                    "list-result",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = get(caller.data_mut());
                        let r = Host::list_result(host);
                        Ok((r,))
                    },
                )?;
                inst.func_wrap(
                    "record-own-arg",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (NestedOwn,)|
                    {
                        let host = get(caller.data_mut());
                        let r = Host::record_own_arg(host, arg0);
                        Ok(r)
                    },
                )?;
                inst.func_wrap(
                    "record-borrow-arg",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (NestedBorrow,)|
                    {
                        let host = get(caller.data_mut());
                        let r = Host::record_borrow_arg(host, arg0);
                        Ok(r)
                    },
                )?;
                inst.func_wrap(
                    "record-result",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = get(caller.data_mut());
                        let r = Host::record_result(host);
                        Ok((r,))
                    },
                )?;
                inst.func_wrap(
                    "func-with-handle-typedef",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (SomeHandle,)|
                    {
                        let host = get(caller.data_mut());
                        let r = Host::func_with_handle_typedef(host, arg0);
                        Ok(r)
                    },
                )?;
                Ok(())
            }
        }
        #[allow(clippy::all)]
        pub mod long_use_chain1 {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::anyhow;
            pub enum A {}
            pub trait HostA {
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<A>,
                ) -> wasmtime::Result<()>;
            }
            pub trait Host: HostA {}
            pub fn add_to_linker<T, U>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wasmtime::Result<()>
            where
                U: Host,
            {
                let mut inst = linker.instance("foo:foo/long-use-chain1")?;
                inst.resource(
                    "a",
                    wasmtime::component::ResourceType::host::<A>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        HostA::drop(
                            get(store.data_mut()),
                            wasmtime::component::Resource::new_own(rep),
                        )
                    },
                )?;
                Ok(())
            }
        }
        #[allow(clippy::all)]
        pub mod long_use_chain2 {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::anyhow;
            pub type A = super::super::super::foo::foo::long_use_chain1::A;
            pub trait Host {}
            pub fn add_to_linker<T, U>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wasmtime::Result<()>
            where
                U: Host,
            {
                let mut inst = linker.instance("foo:foo/long-use-chain2")?;
                Ok(())
            }
        }
        #[allow(clippy::all)]
        pub mod long_use_chain3 {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::anyhow;
            pub type A = super::super::super::foo::foo::long_use_chain2::A;
            pub trait Host {}
            pub fn add_to_linker<T, U>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wasmtime::Result<()>
            where
                U: Host,
            {
                let mut inst = linker.instance("foo:foo/long-use-chain3")?;
                Ok(())
            }
        }
        #[allow(clippy::all)]
        pub mod long_use_chain4 {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::anyhow;
            pub type A = super::super::super::foo::foo::long_use_chain3::A;
            pub trait Host {
                fn foo(&mut self) -> wasmtime::component::Resource<A>;
            }
            pub fn add_to_linker<T, U>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wasmtime::Result<()>
            where
                U: Host,
            {
                let mut inst = linker.instance("foo:foo/long-use-chain4")?;
                inst.func_wrap(
                    "foo",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = get(caller.data_mut());
                        let r = Host::foo(host);
                        Ok((r,))
                    },
                )?;
                Ok(())
            }
        }
        #[allow(clippy::all)]
        pub mod transitive_interface_with_resource {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::anyhow;
            pub enum Foo {}
            pub trait HostFoo {
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<Foo>,
                ) -> wasmtime::Result<()>;
            }
            pub trait Host: HostFoo {}
            pub fn add_to_linker<T, U>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wasmtime::Result<()>
            where
                U: Host,
            {
                let mut inst = linker
                    .instance("foo:foo/transitive-interface-with-resource")?;
                inst.resource(
                    "foo",
                    wasmtime::component::ResourceType::host::<Foo>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        HostFoo::drop(
                            get(store.data_mut()),
                            wasmtime::component::Resource::new_own(rep),
                        )
                    },
                )?;
                Ok(())
            }
        }
    }
}
pub mod exports {
    pub mod foo {
        pub mod foo {
            #[allow(clippy::all)]
            pub mod uses_resource_transitively {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::anyhow;
                pub type Foo = super::super::super::super::foo::foo::transitive_interface_with_resource::Foo;
                pub struct Guest {
                    handle: wasmtime::component::Func,
                }
                impl Guest {
                    pub fn new(
                        __exports: &mut wasmtime::component::ExportInstance<'_, '_>,
                    ) -> wasmtime::Result<Guest> {
                        let handle = *__exports
                            .typed_func::<
                                (wasmtime::component::Resource<Foo>,),
                                (),
                            >("handle")?
                            .func();
                        Ok(Guest { handle })
                    }
                    pub fn call_handle<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                        arg0: wasmtime::component::Resource<Foo>,
                    ) -> wasmtime::Result<()> {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (wasmtime::component::Resource<Foo>,),
                                (),
                            >::new_unchecked(self.handle)
                        };
                        let () = callee.call(store.as_context_mut(), (arg0,))?;
                        callee.post_return(store.as_context_mut())?;
                        Ok(())
                    }
                }
            }
        }
    }
}
