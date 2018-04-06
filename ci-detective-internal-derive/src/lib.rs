#![recursion_limit = "128"]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

#[proc_macro_derive(CI, attributes(ci))]
pub fn derive_ci(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: syn::DeriveInput = syn::parse(input).unwrap();
    let ci: syn::Path = syn::parse_str("ci").unwrap();

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let data = if let syn::Data::Struct(data) = input.data {
        data
    } else {
        panic!("derive(CI) only works for structs")
    };

    let fields = if let syn::Fields::Named(fields) = data.fields {
        fields
    } else {
        panic!("derive(CI) only works with named fields")
    };

    let member_access = fields.named.iter().map(impl_member_access_fn);
    let field = fields.named.iter().filter_map(|field| field.ident);
    let field2 = fields.named.iter().filter_map(|field| field.ident);
    let requires_var = input
        .attrs
        .iter()
        .filter(|attr| attr.path == ci)
        .map(unwrap_ci_attribute)
        .filter(|meta| meta.name().as_ref() == "requires")
        .flat_map(|meta| {
            if let syn::Meta::List(meta) = meta {
                meta.nested
                    .into_iter()
                    .map(|meta| {
                        if let syn::NestedMeta::Meta(meta) = meta {
                            meta
                        } else {
                            panic!("#[ci(requires(...))] takes a list of name/value pairs")
                        }
                    })
                    .map(|meta| {
                        if let syn::Meta::NameValue(meta) = meta {
                            meta
                        } else {
                            panic!("#[ci(requires(...))] takes a list of name/value pairs")
                        }
                    })
                    .map(|meta| meta.ident)
            } else {
                panic!("#[ci(requires(...))] takes a list of name/value pairs")
            }
        });
    let requires_val = input
        .attrs
        .iter()
        .filter(|attr| attr.path == ci)
        .map(unwrap_ci_attribute)
        .filter(|meta| meta.name().as_ref() == "requires")
        .flat_map(|meta| {
            if let syn::Meta::List(meta) = meta {
                meta.nested
                    .into_iter()
                    .map(|meta| {
                        if let syn::NestedMeta::Meta(meta) = meta {
                            meta
                        } else {
                            panic!("#[ci(requires(...))] takes a list of name/value pairs")
                        }
                    })
                    .map(|meta| {
                        if let syn::Meta::NameValue(meta) = meta {
                            meta
                        } else {
                            panic!("#[ci(requires(...))] takes a list of name/value pairs")
                        }
                    })
                    .map(|meta| meta.lit)
            } else {
                panic!("#[ci(requires(...))] takes a list of name/value pairs")
            }
        });

    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            /// Construct a lazy instance of this CI's configuration.
            ///
            /// Returns None if the environment doesn't look like the CI's.
            /// Most CI has a set of environment variables which identify it,
            /// such as `TRAVIS=true` for Travis CI. Those are checked eagerly
            /// here.
            pub fn lazy() -> Option<Self> {
                if !(
                    #(::env(stringify!(#requires_var))? == #requires_val &&)*
                    true
                ) { return None; }
                Some(Self {#(#field: None,)*})
                // if !(
                //     //$(::env($require_environment_present).is_some() &&)*
                //     $(::env(stringify!($require_environment_var))? == $require_environment_val &&)*
                //     true
                // ) { return None; }
                // Some($CI {$(
                //     $member: ::LazyEnv::new(stringify!($member_env)),
                // )*})
            }

            pub(crate) fn load(&mut self) {
                #(let _ = self.#field2();)*
            }

            /// Construct an instance of this CI's configuration and load it eagerly.
            ///
            /// # Panics
            ///
            /// If any of the expected environment variables are not present.
            pub fn eager() -> Self {
                let mut ci = Self::lazy().expect(concat!(
                    "Environment does not look like ",
                    stringify!(#name),
                ));
                ci.load();
                ci
            }
        }

        impl #impl_generics #name #ty_generics #where_clause {
            #(#member_access)*
        }
    };
    expanded.into()
}

fn unwrap_ci_attribute(attr: &syn::Attribute) -> syn::Meta {
    assert_eq!(attr.path, syn::parse_str("ci").unwrap());
    let meta = attr.interpret_meta()
        .unwrap_or_else(|| panic!("malformed #[ci] attribute `{}`", quote!(#attr)));
    let meta = if let syn::Meta::List(meta) = meta {
        meta
    } else {
        panic!("#[ci] attribute must use scoping #[ci(...)]")
    };
    let meta = if meta.nested.len() == 1 {
        meta.nested.into_pairs().next().unwrap().into_value()
    } else {
        panic!("#[ci] takes exactly one argument")
    };
    let meta = if let syn::NestedMeta::Meta(meta) = meta {
        meta
    } else {
        panic!("malformed argument to #[ci] `{}`", quote!(#meta))
    };
    meta
}

fn impl_member_access_fn(member: &syn::Field) -> quote::Tokens {
    let ci: syn::Path = syn::parse_str("ci").unwrap();

    let ident = member
        .ident
        .unwrap_or_else(|| panic!("derive(CI) only supports named fields"));
    let ty = {
        if let syn::Type::Path(ref ty) = member.ty {
            let ty = ty.path.segments.first().unwrap().into_value();
            if ty.ident.as_ref() == "Option" {
                if let syn::PathArguments::AngleBracketed(ref arguments) = ty.arguments {
                    if let &syn::GenericArgument::Type(ref ty) =
                        arguments.args.first().unwrap().into_value()
                    {
                        ty
                    } else {
                        panic!("CI members must be Option<...>");
                    }
                } else {
                    panic!("CI members must be Option<...>");
                }
            } else {
                panic!("CI members must be Option<...>");
            }
        } else {
            panic!("CI members must be Option<...>");
        }
    };

    // TODO(RUST 1.26): Remove the PathBuf hack as now PathBuf is ParseStr
    let require_path_buf_hack = if let &syn::Type::Path(ref ty) = ty {
        ty.path.segments.last().unwrap().into_value().ident.as_ref() == "PathBuf"
    } else {
        false
    };

    let docs = member.attrs.iter().filter(|attr| {
        attr.path
            .segments
            .first()
            .unwrap()
            .into_value()
            .ident
            .as_ref() == "doc"
    });
    let expected = member
        .attrs
        .iter()
        .filter(|attr| attr.path == ci)
        .map(unwrap_ci_attribute)
        .filter(|meta| {
            if let &syn::Meta::Word(ref word) = meta {
                word.as_ref() == "expected"
            } else {
                false
            }
        })
        .count();
    let env = {
        let mut env_attr = member
            .attrs
            .iter()
            .filter(|attr| attr.path == ci)
            .map(unwrap_ci_attribute)
            .filter_map(|meta| {
                if let syn::Meta::NameValue(meta) = meta {
                    Some(meta.lit)
                } else {
                    None
                }
            });
        let env = env_attr.next().unwrap_or_else(|| {
            panic!(
                "member `{}` does not have a `#[ci(env = ...)]` attribute",
                ident
            )
        });
        assert!(
            env_attr.next().is_none(),
            "member `{}` has a duplicate `#[ci(env = ...)]` attribute",
            ident
        );
        env
    };

    if expected > 1 {
        panic!("multiple #[ci(expected)] on `{}`", ident)
    } else if expected == 1 {
        if require_path_buf_hack {
            quote! {
                #(#docs)*
                pub fn #ident(&mut self) -> &#ty {
                    if self.#ident.is_none() {
                        self.#ident = ::env(#env).map(Into::into);
                    }
                    self.#ident.as_ref().unwrap_or_else(|| panic!(
                        "Environment variable {} expected to parse to {} but failed; was {:?}",
                        #env, stringify!(#ty), ::env(#env),
                    ))
                }
            }
        } else {
            quote! {
                #(#docs)*
                pub fn #ident(&mut self) -> &#ty {
                    if self.#ident.is_none() {
                        self.#ident = ::env(#env).and_then(|it| it.parse().ok());
                    }
                    self.#ident.as_ref().unwrap_or_else(|| panic!(
                        "Environment variable {} expected to parse to {} but failed; was {:?}",
                        #env, stringify!(#ty), ::env(#env),
                    ))
                }
            }
        }
    } else {
        if require_path_buf_hack {
            quote! {
                #(#docs)*
                pub fn #ident(&mut self) -> &#ty {
                    if self.#ident.is_none() {
                        self.#ident = ::env(#env).map(Into::into);
                    }
                    self.#ident.as_ref()
                }
            }
        } else {
            quote! {
                #(#docs)*
                pub fn #ident(&mut self) -> Option<&#ty> {
                    if self.#ident.is_none() {
                        self.#ident = ::env(#env).and_then(|it| it.parse().ok());
                    }
                    self.#ident.as_ref()
                }
            }
        }
    }
}
