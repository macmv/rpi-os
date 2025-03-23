use syn::{Ident, PathArguments, Token, parse::Parse, punctuated::Punctuated, token::Brace};

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
      if register.offset() < prev.offset() + prev.size() {
        return quote::quote_spanned!(register.offset.span() =>
          compile_error!("register overlaps with previous register");
        );
      }

      if register.offset() % register.size() != 0 {
        panic!("register {} is not aligned", register.name);
      }

      let padding = register.offset() - (prev.offset() + prev.size());

      if padding > 0 {
        let padding_name =
          Ident::new(&format!("_padding{}", padding_count), register.offset.span());
        padding_count += 1;

        let padding = padding as usize;
        fields.push(quote::quote! {
          #padding_name: [u8; #padding],
        });
      }
    };

    let name = &register.name;
    let ty = &register.ty;

    fields.push(quote::quote! {
      pub #name: #ty,
    });
  }

  let name = input.name;
  quote::quote! {
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
  offset: syn::LitInt,
  _arrow: Token![->],
  name:   syn::Ident,
  _colon: Token![:],
  ty:     syn::Type,
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
      offset: input.parse()?,
      _arrow: input.parse()?,
      name:   input.parse()?,
      _colon: input.parse()?,
      ty:     input.parse()?,
    })
  }
}

impl Register {
  pub fn offset(&self) -> u32 { self.offset.base10_parse().unwrap() }
  pub fn size(&self) -> u32 {
    match self.ty {
      syn::Type::Path(ref p) => match p.path.segments.last().unwrap().arguments {
        PathArguments::AngleBracketed(ref args) => match args.args.first().unwrap() {
          syn::GenericArgument::Type(ty) => match ty {
            syn::Type::Path(p) => {
              match p.path.segments.last().unwrap().ident.to_string().as_str() {
                "u8" => 1,
                "u16" => 2,
                "u32" => 4,
                "u64" => 8,
                _ => unimplemented!(),
              }
            }
            _ => unimplemented!(),
          },
          _ => unimplemented!(),
        },
        _ => unimplemented!(),
      },
      _ => unimplemented!(),
    }
  }
}
