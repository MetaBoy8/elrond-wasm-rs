pub fn contract_object_def() -> proc_macro2::TokenStream {
    quote! {
        pub struct ContractObj<A>
        where
            A: elrond_wasm::api::VMApi + Clone + 'static,
        {
            api: A,
        }
    }
}

pub fn impl_contract_base() -> proc_macro2::TokenStream {
    quote! {
        impl<A> elrond_wasm::contract_base::ContractBase for ContractObj<A>
        where
            A: elrond_wasm::api::VMApi + Clone + 'static
        {
            type Api = A;

            fn raw_vm_api(&self) -> Self::Api {
                self.api.clone()
            }
        }
    }
}

pub fn new_contract_object_fn() -> proc_macro2::TokenStream {
    quote! {
        pub fn contract_obj<A>(api: A) -> ContractObj<A>
        where
            A: elrond_wasm::api::VMApi + Clone + 'static,
        {
            ContractObj { api }
        }
    }
}

// TODO: explore auto-implementations of supertraits
#[allow(dead_code)]
pub fn impl_auto_impl() -> proc_macro2::TokenStream {
    quote! {
        impl<A> AutoImpl for ContractObj<A> where
            A: elrond_wasm::contract_base::ContractBase
                + elrond_wasm::api::ErrorApi
                + elrond_wasm::api::EndpointArgumentApi
                + elrond_wasm::api::EndpointFinishApi
                + elrond_wasm::api::ManagedTypeApi
                + Clone
                + 'static
        {
        }
    }
}

// pub fn impl_private_api() -> proc_macro2::TokenStream {
//     quote! {
//         impl<A> elrond_wasm::contract_base::ContractBase for ContractObj<A>
//         where
//             A: elrond_wasm::contract_base::ContractBase
//                 + elrond_wasm::api::ErrorApi
//                 + elrond_wasm::api::EndpointArgumentApi
//                 + elrond_wasm::api::EndpointFinishApi
//                 + elrond_wasm::api::ManagedTypeApi
//                 + Clone
//                 + 'static,
//         {
//             type ArgumentApi = A;
//             type CallbackClosureArgumentApi = A;
//             type FinishApi = A;

//             #[inline]
//             fn argument_api(&self) -> Self::Api {
//                 self.api.clone()
//             }

//             #[inline]
//             fn callback_closure_arg_api(&self) -> Self::CallbackClosureArgumentApi {
//                 self.api.clone()
//             }

//             #[inline]
//             fn finish_api(&self) -> Self::FinishApi {
//                 self.api.clone()
//             }
//         }
//     }
// }

pub fn impl_callable_contract() -> proc_macro2::TokenStream {
    quote! {
        impl<A> elrond_wasm::contract_base::CallableContract<A> for ContractObj<A>
        where
            A: elrond_wasm::api::VMApi + Clone + 'static
        {
            fn call(&self, fn_name: &[u8]) -> bool {
                EndpointWrappers::call(self, fn_name)
            }
            fn into_api(self: Box<Self>) -> A {
                self.api
            }
        }
    }
}

pub fn proxy_object_def() -> proc_macro2::TokenStream {
    quote! {
        pub struct Proxy<A>
        where
            A: elrond_wasm::api::VMApi + 'static,
        {
            pub api: A,
            pub address: elrond_wasm::types::ManagedAddress<A>,
        }

        impl<A> elrond_wasm::contract_base::ProxyObjApi for Proxy<A>
        where
            A: elrond_wasm::api::VMApi + 'static,
        {
            type Api = A;

            fn new_proxy_obj(api: A) -> Self {
                let zero_address = ManagedAddress::zero_address(api.clone());
                Proxy {
                    api,
                    address: zero_address,
                }
            }

            fn contract(mut self, address: ManagedAddress<Self::Api>) -> Self {
                self.address = address;
                self
            }

            #[inline]
            fn into_fields(self) -> (Self::Api, ManagedAddress<Self::Api>) {
                (self.api, self.address)
            }
        }
    }
}

pub fn callback_proxy_object_def() -> proc_macro2::TokenStream {
    quote! {
        pub struct CallbackProxyObj<A>
        where
            A: elrond_wasm::api::VMApi + 'static,
        {
            pub api: A,
        }

        impl<A> elrond_wasm::contract_base::CallbackProxyObjApi for CallbackProxyObj<A>
        where
            A: elrond_wasm::api::VMApi + 'static,
        {
            type Api = A;

            fn new_cb_proxy_obj(api: A) -> Self {
                CallbackProxyObj { api }
            }
            fn cb_call_api(self) -> Self::Api {
                self.api.clone()
            }
        }
    }
}
