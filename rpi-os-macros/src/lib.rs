use quote::quote;
use syn::{Ident, Token, parse::Parse, punctuated::Punctuated, token::Brace};

#[proc_macro]
pub fn reg_struct(body: proc_macro::TokenStream) -> proc_macro::TokenStream {
  reg_struct_impl(syn::parse_macro_input!(body as Registers)).into()
}

fn reg_struct_impl(input: Registers) -> proc_macro2::TokenStream {
  let mut fields = vec![];
  let mut padding_count = 0;

  for (i, register) in input.registers.iter().enumerate() {
    if i > 0 {
      let prev = &input.registers[i - 1];
      if register.offset < prev.offset + prev.size {
        return quote::quote_spanned!(register.name.span() =>
          compile_error!("register overlaps with previous register");
        );
      }

      if register.offset % register.size != 0 {
        panic!("register {} is not aligned", register.name);
      }

      let padding = register.offset - (prev.offset + prev.size);

      if padding > 0 {
        let padding_name = Ident::new(&format!("_padding{}", padding_count), register.name.span());
        padding_count += 1;

        let padding = padding as usize;
        fields.push(quote! {
          #padding_name: [u8; #padding],
        });
      }
    };

    let name = &register.name;
    let ty = &register.ty;
    let arg = match register.alias {
      Some((_, ref arg)) => quote!(#arg),
      None => match register.size {
        1 => quote! { u8 },
        2 => quote! { u16 },
        4 => quote! { u32 },
        8 => quote! { u64 },
        _ => unimplemented!(),
      },
    };

    fields.push(quote! {
      pub #name: #ty<#arg>,
    });
  }

  let name = input.name;
  quote! {
    #[repr(C)]
    pub struct #name {
      #(#fields)*
    }
  }
}
struct Registers {
  _struct:   Token![struct],
  name:      syn::Ident,
  _brace:    Brace,
  registers: Punctuated<Register, Token![,]>,
}

struct Register {
  offset:       u32,
  _arrow:       Token![->],
  name:         syn::Ident,
  _colon:       Token![:],
  ty:           syn::Ident,
  _open_brace:  Token![<],
  size:         u32,
  alias:        Option<(Token![=], syn::Path)>,
  _close_brace: Token![>],
}

impl Parse for Registers {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let registers;

    Ok(Registers {
      _struct:   input.parse()?,
      name:      input.parse()?,
      _brace:    syn::braced!(registers in input),
      registers: registers.parse_terminated(Register::parse, Token![,])?,
    })
  }
}

impl Parse for Register {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    Ok(Register {
      offset:       input.parse::<syn::LitInt>()?.base10_parse().unwrap(),
      _arrow:       input.parse()?,
      name:         input.parse()?,
      _colon:       input.parse()?,
      ty:           input.parse()?,
      _open_brace:  input.parse()?,
      size:         match input.parse::<Ident>()?.to_string().as_str() {
        "u8" => 1,
        "u16" => 2,
        "u32" => 4,
        "u64" => 8,
        _ => unimplemented!(),
      },
      alias:        if input.peek(Token![=]) {
        Some((input.parse()?, input.parse()?))
      } else {
        None
      },
      _close_brace: input.parse()?,
    })
  }
}
