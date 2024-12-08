use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{
    self, parse_macro_input, DeriveInput, Expr, ExprPath, GenericArgument, Lit, Meta,
    MetaNameValue, NestedMeta, Path, PathArguments, Type, TypePath,
};

#[derive(Default)]
struct Variant {
    pub name: String,
    pub tuple: Option<String>,
    pub accounts: Vec<Account>,
    // (name, type, generic type)
    pub args: Vec<(String, String, Option<String>)>,
}

#[derive(Debug)]
struct Account {
    pub name: String,
    pub optional: bool,
}

// Helper account attribute (reusing from shank annotation).
const ACCOUNT_ATTRIBUTE: &str = "account";
// Helper args attribute.
const ARGS_ATTRIBUTE: &str = "args";
// Name property in the account attribute.
const NAME_PROPERTY: &str = "name";
// Optional property in the account attribute.
const OPTIONAL_PROPERTY: &str = "optional";

#[proc_macro_derive(AccountContext, attributes(account, args))]
pub fn account_context_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let variants = if let syn::Data::Enum(syn::DataEnum { ref variants, .. }) = ast.data {
        let mut enum_variants = Vec::new();

        for v in variants {
            let tuple = if let syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) = &v.fields
            {
                match unnamed.first() {
                    Some(syn::Field {
                        ty:
                            Type::Path(TypePath {
                                path: Path { segments, .. },
                                ..
                            }),
                        ..
                    }) => Some(segments.first().unwrap().ident.to_string()),
                    _ => None,
                }
            } else {
                None
            };

            let mut variant = Variant {
                tuple,
                name: v.ident.to_string(),
                ..Default::default()
            };

            for a in &v.attrs {
                let mut attribute = String::new();
                let mut skip = true;

                for path in &a.path.segments {
                    let ident = path.ident.to_string();
                    if ident == ACCOUNT_ATTRIBUTE || ident == ARGS_ATTRIBUTE {
                        attribute = ident;
                        skip = false;
                        break;
                    }
                }

                if !skip {
                    if attribute == ACCOUNT_ATTRIBUTE {
                        match a.parse_meta() {
                            Ok(Meta::List(meta_list)) => {
                                let mut name = None;
                                let mut optional = false;

                                for nested in meta_list.nested.iter() {
                                    if let NestedMeta::Meta(meta) = nested {
                                        if let Some(nv) =
                                            extract_name_value(meta, NAME_PROPERTY).unwrap_or(None)
                                        {
                                            name = Some(nv);
                                        } else if is_path_matching(nested, OPTIONAL_PROPERTY) {
                                            optional = true;
                                        }
                                    }
                                }

                                let account_name = match name {
                                    Some(n) => n,
                                    None => {
                                        return syn::Error::new_spanned(
                                            a,
                                            format!(
                                                "Missing '{}' in #[account] attribute",
                                                NAME_PROPERTY
                                            ),
                                        )
                                        .to_compile_error()
                                        .into();
                                    }
                                };

                                variant.accounts.push(Account {
                                    name: account_name,
                                    optional,
                                });
                            }
                            Err(_) => {
                                return syn::Error::new_spanned(
                                    a,
                                    "Failed to parse #[account] attribute",
                                )
                                .to_compile_error()
                                .into();
                            }
                            _ => {
                                return syn::Error::new_spanned(
                                    a,
                                    "#[account] attribute must be a list",
                                )
                                .to_compile_error()
                                .into();
                            }
                        }
                    } else if attribute == ARGS_ATTRIBUTE {
                        match a.parse_args::<syn::ExprType>() {
                            Ok(args_tokens) => {
                                let name = match *args_tokens.expr {
                                    Expr::Path(ExprPath {
                                        path: Path { segments, .. },
                                        ..
                                    }) => segments.first().unwrap().ident.to_string(),
                                    _ => {
                                        return syn::Error::new_spanned(
                                            a,
                                            "#[args] requires an expression 'name: type'",
                                        )
                                        .to_compile_error()
                                        .into();
                                    }
                                };

                                let ty = match *args_tokens.ty {
                                    Type::Path(TypePath {
                                        path: Path { segments, .. },
                                        ..
                                    }) => {
                                        let segment = segments.first().unwrap();

                                        let generic_ty = match &segment.arguments {
                                            PathArguments::AngleBracketed(arguments) => {
                                                if let Some(GenericArgument::Type(Type::Path(ty))) =
                                                    arguments.args.first()
                                                {
                                                    Some(
                                                        ty.path
                                                            .segments
                                                            .first()
                                                            .unwrap()
                                                            .ident
                                                            .to_string(),
                                                    )
                                                } else {
                                                    None
                                                }
                                            }
                                            _ => None,
                                        };

                                        (segment.ident.to_string(), generic_ty)
                                    }
                                    _ => {
                                        return syn::Error::new_spanned(
                                            a,
                                            "#[args] requires an expression 'name: type'",
                                        )
                                        .to_compile_error()
                                        .into();
                                    }
                                };

                                variant.args.push((name, ty.0, ty.1));
                            }
                            Err(_) => {
                                return syn::Error::new_spanned(
                                    a,
                                    "Failed to parse #[args] attribute",
                                )
                                .to_compile_error()
                                .into();
                            }
                        }
                    }
                }
            }

            enum_variants.push(variant);
        }

        enum_variants
    } else {
        return syn::Error::new_spanned(ast, "AccountContext can only be derived for enums")
            .to_compile_error()
            .into();
    };

    let mut account_structs = generate_accounts(&variants);
    account_structs.extend(generate_builders(&variants));

    account_structs
}

/// Helper function to extract a string value from a Meta based on the property name.
fn extract_name_value(meta: &Meta, property: &str) -> Result<Option<String>, syn::Error> {
    if let Meta::NameValue(MetaNameValue { path, lit, .. }) = meta {
        if let Some(ident) = path.get_ident() {
            if ident == property {
                if let Lit::Str(lit_str) = lit {
                    return Ok(Some(lit_str.value()));
                } else {
                    return Err(syn::Error::new_spanned(
                        lit,
                        format!("Expected a string literal for '{}'", property),
                    ));
                }
            }
        }
    }
    Ok(None)
}

/// Helper function to check if a NestedMeta matches the given property name.
fn is_path_matching(nested_meta: &NestedMeta, property: &str) -> bool {
    if let NestedMeta::Meta(Meta::Path(path)) = nested_meta {
        if let Some(ident) = path.get_ident() {
            return ident == property;
        }
    }
    false
}

/// Generates a struct for each enum variant.
fn generate_accounts(variants: &[Variant]) -> TokenStream {
    let variant_structs = variants.iter().map(|variant| {
        let name = syn::parse_str::<syn::Ident>(&variant.name).unwrap();
        let struct_fields = variant.accounts.iter().map(|account| {
            let account_name = syn::parse_str::<syn::Ident>(&format!("{}_info", account.name)).unwrap();
            if account.optional {
                quote! {
                    pub #account_name: Option<&'a pinocchio::account_info::AccountInfo>
                }
            } else {
                quote! {
                    pub #account_name: &'a pinocchio::account_info::AccountInfo
                }
            }
        });

        let account_fields = variant.accounts.iter().enumerate().map(|(index, account)| {
            let account_name = syn::parse_str::<syn::Ident>(&format!("{}_info", account.name)).unwrap();

            if account.optional {
                quote! {
                    #account_name: if accounts[#index].key() == &crate::ID { None } else { Some(&accounts[#index]) }
                }
            } else {
                quote! {
                    #account_name: &accounts[#index]
                }
            }
        });

        let expected = variant.accounts.len();

        quote! {
            pub struct #name<'a> {
                #(#struct_fields,)*
            }
            impl<'a> #name<'a> {
                pub fn to_context(accounts: &'a [pinocchio::account_info::AccountInfo]) -> Result<Context<'a, Self>, pinocchio::program_error::ProgramError> {
                    if accounts.len() < #expected {
                        return Err(pinocchio::program_error::ProgramError::NotEnoughAccountKeys);
                    }
                    Ok(Context {
                        accounts: Self {
                            #(#account_fields,)*
                        },
                        remaining_accounts: &accounts[#expected..],
                    })
                }
            }
        }
    });

    TokenStream::from(quote! {
        #(#variant_structs)*
    })
}

fn generate_builders(variants: &[Variant]) -> TokenStream {
    let mut default_pubkeys = HashMap::new();
    default_pubkeys.insert(
        "system_program".to_string(),
        syn::parse_str::<syn::ExprPath>("pinocchio_system::ID").unwrap(),
    );
    default_pubkeys.insert(
        "spl_token_program".to_string(),
        syn::parse_str::<syn::ExprPath>("crate::utils::SPL_TOKEN_ID").unwrap(),
    );
    default_pubkeys.insert(
        "spl_ata_program".to_string(),
        syn::parse_str::<syn::ExprPath>("crate::utils::SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID")
            .unwrap(),
    );
    default_pubkeys.insert(
        "sysvar_instructions".to_string(),
        syn::parse_str::<syn::ExprPath>("crate::utils::SYSVAR_ID").unwrap(),
    );
    default_pubkeys.insert(
        "authorization_rules_program".to_string(),
        syn::parse_str::<syn::ExprPath>("mpl_token_auth_rules::ID").unwrap(),
    );

    let variant_structs = variants.iter().map(|variant| {
        let name = syn::parse_str::<syn::Ident>(&variant.name).unwrap();
        let builder_name = syn::parse_str::<syn::Ident>(&format!("{}Builder", name)).unwrap();

        // Struct accounts
        let struct_accounts = variant.accounts.iter().map(|account| {
            let account_name = syn::parse_str::<syn::Ident>(&account.name).unwrap();
            if account.optional {
                quote! {
                    pub #account_name: Option<pinocchio::pubkey::Pubkey>
                }
            } else {
                quote! {
                    pub #account_name: pinocchio::pubkey::Pubkey
                }
            }
        });

        // Struct args
        let struct_args = variant.args.iter().map(|(name, ty, generic_ty)| {
            let ident_ty = syn::parse_str::<syn::Ident>(ty).unwrap();
            let arg_ty = if let Some(genetic_ty) = generic_ty {
                let arg_generic_ty = syn::parse_str::<syn::Ident>(genetic_ty).unwrap();
                quote! { #ident_ty<#arg_generic_ty> }
            } else {
                quote! { #ident_ty }
            };
            let arg_name = syn::parse_str::<syn::Ident>(name).unwrap();

            quote! {
                pub #arg_name: #arg_ty
            }
        });

        // Builder accounts
        let builder_accounts = variant.accounts.iter().map(|account| {
            let account_name = syn::parse_str::<syn::Ident>(&account.name).unwrap();
            quote! {
                pub #account_name: Option<pinocchio::pubkey::Pubkey>
            }
        });

        // Builder args
        let builder_args = variant.args.iter().map(|(name, ty, generic_ty)| {
            let ident_ty = syn::parse_str::<syn::Ident>(ty).unwrap();
            let arg_ty = if let Some(genetic_ty) = generic_ty {
                let arg_generic_ty = syn::parse_str::<syn::Ident>(genetic_ty).unwrap();
                quote! { Option<#ident_ty<#arg_generic_ty>> }
            } else {
                quote! { Option<#ident_ty> }
            };
            let arg_name = syn::parse_str::<syn::Ident>(name).unwrap();

            quote! {
                pub #arg_name: Option<#arg_ty>
            }
        });

        // Builder initialization
        let builder_initialize_accounts = variant.accounts.iter().map(|account| {
            let account_name = syn::parse_str::<syn::Ident>(&account.name).unwrap();
            quote! {
                #account_name: None
            }
        });

        let builder_initialize_args = variant.args.iter().map(|(name, _ty, _generi_ty)| {
            let arg_name = syn::parse_str::<syn::Ident>(name).unwrap();
            quote! {
                #arg_name: None
            }
        });

        // Builder account setter methods
        let builder_accounts_methods = variant.accounts.iter().map(|account| {
            let account_name = syn::parse_str::<syn::Ident>(&account.name).unwrap();
            quote! {
                pub fn #account_name(&mut self, #account_name: pinocchio::pubkey::Pubkey) -> &mut Self {
                    self.#account_name = Some(#account_name);
                    self
                }
            }
        });

        // Builder args setter methods
        let builder_args_methods = variant.args.iter().map(|(name, ty, generic_ty)| {
            let ident_ty = syn::parse_str::<syn::Ident>(ty).unwrap();
            let arg_ty = if let Some(genetic_ty) = generic_ty {
                let arg_generic_ty = syn::parse_str::<syn::Ident>(genetic_ty).unwrap();
                quote! { #ident_ty<#arg_generic_ty> }
            } else {
                quote! { #ident_ty }
            };
            let arg_name = syn::parse_str::<syn::Ident>(name).unwrap();

            quote! {
                pub fn #arg_name(&mut self, #arg_name: #arg_ty) -> &mut Self {
                    self.#arg_name = Some(#arg_name);
                    self
                }
            }
        });

        // Required accounts
        let required_accounts = variant.accounts.iter().map(|account| {
            let account_name = syn::parse_str::<syn::Ident>(&account.name).unwrap();

            if account.optional {
                quote! {
                    #account_name: self.#account_name
                }
            } else if default_pubkeys.contains_key(&account.name) {
                let pubkey = default_pubkeys.get(&account.name).unwrap();
                quote! {
                    #account_name: self.#account_name.unwrap_or(#pubkey)
                }
            } else {
                quote! {
                    #account_name: self.#account_name.ok_or(pinocchio::program_error::ProgramError::InvalidArgument)?
                }
            }
        });

        // Required args
        let required_args = variant.args.iter().map(|(name, _ty, _generic_ty)| {
            let arg_name = syn::parse_str::<syn::Ident>(name).unwrap();
            quote! {
                #arg_name: self.#arg_name.clone().ok_or(pinocchio::program_error::ProgramError::InvalidArgument)?
            }
        });

        // Args parameter list
        let args = if let Some(args) = &variant.tuple {
            let arg_ty = syn::parse_str::<syn::Ident>(args).unwrap();
            quote! { &mut self, args: #arg_ty }
        } else {
            quote! { &mut self }
        };

        // Instruction args
        let instruction_args = if variant.tuple.is_some() {
            quote! { pub args: #args, }
        } else {
            quote! {}
        };

        // Required instruction args
        let required_instruction_args = if variant.tuple.is_some() {
            quote! { args, }
        } else {
            quote! {}
        };

        // Builder new method
        let builder_new = quote! {
            pub fn new() -> Box<#builder_name> {
                Box::new(#builder_name {
                    #(#builder_initialize_accounts,)*
                    #(#builder_initialize_args,)*
                })
            }
        };

        // Builder build method
        let builder_build = quote! {
            pub fn build(#args) -> Result<Box<#name>, pinocchio::program_error::ProgramError> {
                Ok(Box::new(#name {
                    #(#required_accounts,)*
                    #(#required_args,)*
                    #required_instruction_args
                }))
            }
        };

        quote! {
            pub struct #name {
                #(#struct_accounts,)*
                #(#struct_args,)*
                #instruction_args
            }

            pub struct #builder_name {
                #(#builder_accounts,)*
                #(#builder_args,)*
            }

            impl #builder_name {
                #builder_new

                #(#builder_accounts_methods)*

                #(#builder_args_methods)*

                #builder_build
            }
        }
    });

    TokenStream::from(quote! {
        pub mod builders {
            use super::*;

            #(#variant_structs)*
        }
    })
}
