#![feature(proc_macro_diagnostic)]
#![feature(slice_group_by)]

use proc_macro::TokenStream;
use syn::{LitStr, Ident, parse::{Parse, ParseStream, Result}, Token, parse_macro_input};
use quote::quote;




struct FormattedObject {
    body_map: Vec<(LitStr,Vec<Ident>)>,
    text_map: Vec<(LitStr,Vec<Ident>)>
}
/*
create_object! 
{
    * "  x  x  " GREEN "        "      *
    * "  xxxx  " RED   "        "      *
    * "xxxxxxxx" BLUE  " hello! " CYAN *
    * "xxxxxxxx" BLUE  "        "      *
    * "  x  x  " BLUE  "        "      *
}
*/

impl Parse for FormattedObject {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![*]) == true {
            let mut body_map: Vec<(LitStr,Vec<Ident>)> = vec![];
            let mut text_map: Vec<(LitStr,Vec<Ident>)> = vec![];
            let mut text_map_required = false;
            input.parse::<Token![*]>()?;
            let first_value: LitStr = input.parse()?;
            let first_string = first_value.value();
            let object_width = first_string.len();
            let first_string_is_valid = first_string.chars().all(|c| c.is_alphanumeric() | c.is_whitespace());
            let first_string_is_whitespace = first_string.chars().all(|c| c.is_whitespace());
            if first_string_is_valid == false {
                return Err(input.error("the strings for the body can only contain alphanumeric characters or whitespace"));
            }
            let mut first_body_colors: Vec<Ident> = vec![];
            if input.peek(Token![*]) == false && input.peek(LitStr) == false {
                if first_string_is_whitespace == true {
                    return Err(input.error("Can't assign a color to a whitespace string"));
                }
                loop {
                    first_body_colors.push(input.parse()?);
                    if input.peek(Token![*]) | input.peek(LitStr) {
                        break;
                    }
                }
            }
            else {
                if first_string_is_whitespace == false {
                    return Err(input.error("expected a variable of type `Color` on the first body string because it is not all whitespace"));
                }
            }
            body_map.push((first_value,first_body_colors));
            if input.peek(LitStr) {
                text_map_required = true;
                let value: LitStr = input.parse()?;
                let string = value.value();
                let width = string.len();
                if width != object_width {
                    return Err(input.error("the text strings for the body strings must be the same length as the very first body string"));
                }
                let is_whitespace = string.chars().all(|c| c.is_whitespace());
                let mut text_colors: Vec<Ident> = vec![];
                if input.peek(Token![*]) == false {
                    if is_whitespace == true {
                        return Err(input.error("Can't assign a color to a whitespace string"));
                    }
                    loop {
                        text_colors.push(input.parse()?);
                        if input.peek(Token![*]) {
                            break;
                        }
                    }
                }
                else {
                    if is_whitespace == false {
                        return Err(input.error("expected a variable of type `Color` on the first text string because it is not all whitespace"));
                    }
                }
                text_map.push((value,text_colors));
            }
            input.parse::<Token![*]>()?;
            while input.peek(Token![*]) == true {
                input.parse::<Token![*]>()?;
                let value: LitStr = input.parse()?;
                let string = value.value();
                let width = string.len();
                if width != object_width {
                    return Err(input.error("the text strings for the body strings must be the same length as the very first body string"));
                }
                let is_valid = string.chars().all(|c| c.is_alphanumeric() | c.is_whitespace());
                let is_whitespace = string.chars().all(|c| c.is_whitespace());
                if is_valid == false {
                    return Err(input.error("the strings for the body can only contain alphanumeric characters or whitespace"));
                }
                let mut body_colors: Vec<Ident> = vec![];
                if input.peek(Token![*]) == false && input.peek(LitStr) == false {
                    if is_whitespace == true {
                        return Err(input.error("Can't assign a color to a whitespace string"));
                    }
                    loop {
                        body_colors.push(input.parse()?);
                        if input.peek(Token![*]) | input.peek(LitStr) {
                            break;
                        }
                    }
                }
                else {
                    if is_whitespace == false {
                        return Err(input.error("expected a variable of type `Color` on the body string because it is not all whitespace"));
                    }
                }
                body_map.push((value,body_colors));
                if text_map_required {
                    let value: LitStr = input.parse()?;
                    let string = value.value();
                    let width = string.len();
                    if width != object_width {
                        return Err(input.error("the text strings for the body strings must be the same length as the very first body string"));
                    }
                    let is_whitespace = string.chars().all(|c| c.is_whitespace());
                    let mut text_colors: Vec<Ident> = vec![];
                    if input.peek(Token![*]) == false {
                        if is_whitespace == true {
                            return Err(input.error("Can't assign a color to a whitespace string"));
                        }
                        loop {
                            text_colors.push(input.parse()?);
                            if input.peek(Token![*]) {
                                break;
                            }
                        }
                    }
                    else {
                        if is_whitespace == false {
                            return Err(input.error("expected a variable of type `Color` on the first text string because it is not all whitespace"));
                        }
                    }
                    text_map.push((value,text_colors));
                }
                input.parse::<Token![*]>()?;
            }
            return Ok(FormattedObject {
                body_map,
                text_map
            });
        }
        else {
            Err(input.error("expected a delimiter of `*` here. Did you forget how to create an object using this syntax?\n [] - optional \n create_object! { * $LITSTR $COLOR [$LITSTR $COLOR] * }"))
        }
    }
}

#[proc_macro]
pub fn create_object(input: TokenStream) -> TokenStream {
    let FormattedObject {
        body_map,
        text_map
    } = parse_macro_input!(input as FormattedObject);
    /* 
        create_object! {
            * "  x  x  " GREEN "        "      *
            * "  xxxx  " RED   "        "      *
            * "xxxxxxxx" BLUE  " hello! " CYAN *
            * "xxxxxxxx" BLUE  "        "      *
            * "  x  x  " BLUE  "        "      *
        }
        let x = AnsiiString::new();
        x.push(AnsiiChunk::Void)
    */

    let mut final_quote = Vec::<proc_macro2::TokenStream>::new();

    final_quote.push(
        quote! {
            let mut _ansii_string = AnsiiString::new();
        }
    );

    for (chunk,bank) in body_map.into_iter().zip(text_map.into_iter()) {
        let body = chunk.0;
        let color_map = chunk.1;
        let text = bank.0;
        let text_string = text.value();
        let text_color_map = bank.1;
        let body_string = body.value();
        let wrapped_color_letter = body_string.chars().find(|c| c.is_alphanumeric() );
        assert!(wrapped_color_letter.is_some(),"whitespace strings not yet implemented");
        let color_letter = wrapped_color_letter.unwrap();
        assert!(body_string.chars().all(|c| c.is_whitespace() | (c == color_letter) ),"multiple color letters not yet implemented");
        body_string.chars().zip(text_string.chars()).collect::<Vec<_>>().group_by(|c1,c2| c1.0 == c2.0).for_each(|c| if c[0].0 == color_letter {
            let x: String = c.into_iter().map(|c| c.1).collect();
            let body_color = &color_map[0];
            let text_color = if text_color_map.len() > 0 {
                &text_color_map[0]
            } else {
                &color_map[0]
            };
            final_quote.push(
                quote! {
                    _ansii_string += AnsiiChunk::Text(#x.to_string(),Color::make_rgbc(#body_color.rgb48(),#text_color.rgb38()));
                }
            )
        } else {
            let x: String = c.into_iter().map(|c| c.1).collect();
            final_quote.push(
                quote! {
                    _ansii_string += AnsiiChunk::Void(#x.to_string(),Color::None);
                }
            )
        });
        final_quote.push(
            quote! {
                _ansii_string += AnsiiChunk::Text("\n".to_string(),Color::None);
            }
        )
    }

    final_quote.push(
        quote! {
            _ansii_string
        }
    );
    
    let expanded = quote! {
        {
            #( #final_quote )*
        }
    };

    TokenStream::from(expanded)
}
