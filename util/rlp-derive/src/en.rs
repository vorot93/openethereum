// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// This file is part of Parity Ethereum.

// Parity Ethereum is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Ethereum is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Ethereum.  If not, see <http://www.gnu.org/licenses/>.

use syn;
use proc_macro2::{TokenStream, Span};

pub fn impl_encodable(ast: &syn::DeriveInput) -> TokenStream {
	let body = if let syn::Data::Struct(s) = &ast.data {
		s
	} else {
		panic!("#[derive(RlpEncodable)] is only defined for structs.")
	};

	let stmts: Vec<_> = body.fields.iter().enumerate().map(encodable_field_map).collect();
	let name = &ast.ident;

	let stmts_len = stmts.len();
	let stmts_len = quote! { #stmts_len };
	let dummy_const = syn::Ident::new(&format!("_IMPL_RLP_ENCODABLE_FOR_{}", name), Span::call_site());
	let impl_block = quote! {
		impl rlp::Encodable for #name {
			fn rlp_append(&self, stream: &mut rlp::RlpStream) {
				stream.begin_list(#stmts_len);
				#(#stmts)*
			}
		}
	};

	quote! {
		#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
		const #dummy_const: () = {
			extern crate rlp;
			#impl_block
		};
	}
}

pub fn impl_encodable_wrapper(ast: &syn::DeriveInput) -> TokenStream {
	let body = {
		if let syn::Data::Struct(s) = &ast.data {
			s
		} else {
			panic!("#[derive(RlpEncodableWrapper)] is only defined for structs.");
		}
	};

	let stmt = {
		let fields: Vec<_> = body.fields.iter().collect();
		if fields.len() == 1 {
			let field = fields.first().expect("fields.len() == 1; qed");
			encodable_field(0, field)
		} else {
			panic!("#[derive(RlpEncodableWrapper)] is only defined for structs with one field.")
		}
	};

	let name = &ast.ident;

	let dummy_const = syn::Ident::new(&format!("_IMPL_RLP_ENCODABLE_FOR_{}", name), Span::call_site());
	let impl_block = quote! {
		impl rlp::Encodable for #name {
			fn rlp_append(&self, stream: &mut rlp::RlpStream) {
				#stmt
			}
		}
	};

	quote! {
		#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
		const #dummy_const: () = {
			extern crate rlp;
			#impl_block
		};
	}
}

fn encodable_field_map(tuple: (usize, &syn::Field)) -> TokenStream {
	encodable_field(tuple.0, tuple.1)
}

fn encodable_field(index: usize, field: &syn::Field) -> TokenStream {
	let ident = if let Some(ident) = field.ident.as_ref() {
		quote! { #ident }
	} else {
		let index = syn::Index::from(index);
		quote! { #index }
	};

	let id = quote! { self.#ident };

	match &field.ty {
		syn::Type::Path(path) => {
			let top_segment = path.path.segments.first().expect("there must be at least 1 segment");
			let ident = &top_segment.value().ident;
			if &ident.to_string() == "Vec" {
				let inner_ident = match &top_segment.value().arguments {
					syn::PathArguments::AngleBracketed(angle) => {
						let ty = angle.args.first().expect("Vec has only one angle bracketed type; qed");
						match &**ty.value() {
							syn::GenericArgument::Type(syn::Type::Path(path)) => &path.path.segments.first().expect("there must be at least 1 segment").value().ident,
							_ => panic!("rlp_derive not supported"),
						}
					},
					_ => unreachable!("Vec has only one angle bracketed type; qed"),
				};
				quote! { stream.append_list::<#inner_ident, _>(&#id); }
			} else {
				quote! { stream.append(&#id); }
			}
		},
		_ => panic!("rlp_derive not supported"),
	}
}
