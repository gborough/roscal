use std::{
    env,
    fs::File,
    io::{BufReader, Read, Write},
    path::{Path, PathBuf},
    vec,
};

// macro_rules! p {
//     ($($tokens: tt)*) => {
//         println!("cargo:warning={}", format!($($tokens)*))
//     }
// }

fn check_schema_diff() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = env::current_dir()?;

    let bu_path = format!(
        "{}/jsonschema/baseline_unified/1.0.0.json",
        current_dir.to_string_lossy()
    );

    let pu_path = format!(
        "{}/jsonschema/previous_unified/1.1.1.json",
        current_dir.to_string_lossy()
    );

    let tlu_path = format!(
        "{}/jsonschema/tracking_latest_unified/1.1.2.json",
        current_dir.to_string_lossy()
    );

    let bu = File::open(bu_path)?;
    let bu_reader = BufReader::new(bu);
    let pu = File::open(pu_path)?;
    let pu_reader = BufReader::new(pu);
    let tlu = File::open(tlu_path)?;
    let tlu_reader = BufReader::new(tlu);

    let bu_schema: serde_json::Value = serde_json::from_reader(bu_reader)?;
    let pu_schema: serde_json::Value = serde_json::from_reader(pu_reader)?;
    let tlu_schema: serde_json::Value = serde_json::from_reader(tlu_reader)?;

    assert_eq!(
        json_schema_diff::diff(bu_schema, pu_schema.clone())?,
        vec![]
    );
    assert_eq!(json_schema_diff::diff(pu_schema, tlu_schema)?, vec![]);

    Ok(())
}

fn read_file(
    path: impl AsRef<Path>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}

fn create_outout_path(
    path: &str,
    folder: &str,
    parsed: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = env::current_dir()?;
    let output_path =
        format!("{}/src/{}/{}", current_dir.to_string_lossy(), folder, path);
    let mut file = File::create(output_path)?;

    file.write_all(parsed.as_bytes())?;

    Ok(())
}

fn is_optional_type(path: &syn::Path) -> bool {
    path.segments[0].ident == "Option"
}

fn process_ast_common(ast: &mut syn::File) -> &mut syn::File {
    ast.items.insert(
        0,
        syn::parse_quote! {
            use derive_builder::Builder;
        },
    );

    ast.items.insert(
        0,
        syn::parse_quote! {
            use strum::EnumString;

        },
    );

    ast.items.insert(
        0,
        syn::parse_quote! {
            use derivative::Derivative;

        },
    );

    ast.items.insert(
        4,
        syn::parse_quote! {
            use crate::validation;
        },
    );

    ast.items.iter_mut().for_each(|ref mut item| {
        if let syn::Item::Struct(item_struct) = item {
            item_struct.attrs.insert(
                0,
                syn::parse_quote! {
                    #[derive(ToBeReplaced)]
                },
            );
            item_struct.attrs.extend(vec![
                syn::parse_quote! {
                    #[derive(Builder, Derivative)]
                },
                syn::parse_quote! {
                    #[builder(setter(into, strip_option))]
                },
                syn::parse_quote! {
                    #[derivative(PartialEq)]
                },
                syn::parse_quote! {
                    #[serde(deny_unknown_fields)]
                },
            ]);

            (&mut item_struct.fields)
                .into_iter()
                .for_each(|ref mut field| {
                    if let syn::Type::Path(typepath) = &field.ty {
                        if is_optional_type(&typepath.path) {
                            field.attrs.extend(vec![
                                syn::parse_quote!{#[builder(setter(into, strip_option), default)]},
                                syn::parse_quote!{#[serde(default, skip_serializing_if = "Option::is_none")]}])
                        }
                    }

                    if let Some(ident) = &mut field.ident {
                        if ident.to_string().as_str() == "remarks" {
                            field.attrs.push(syn::parse_quote! {
                                #[serde(
                                    deserialize_with = "validation::deser_markup_opt"
                                )]
                            })
                        } else if ident.to_string().as_str() == "uuid" {
                            if let syn::Type::Path(typepath) = &field.ty {
                                if is_optional_type(&typepath.path) {
                                    field.attrs.extend(vec![
                                        syn::parse_quote! {
                                            #[serde(
                                                serialize_with = "validation::ser_uuid_opt",
                                                deserialize_with = "validation::deser_uuid_opt"
                                            )]
                                        },
                                        syn::parse_quote! {
                                            #[derivative(PartialEq="ignore")]
                                        }
                                    ])
                                } else {
                                    field.attrs.extend(vec![
                                        syn::parse_quote! {
                                            #[serde(
                                                serialize_with = "validation::ser_uuid",
                                                deserialize_with = "validation::deser_uuid"
                                            )]
                                        },
                                        syn::parse_quote! {
                                            #[derivative(PartialEq="ignore")]
                                        }
                                    ])
                                }
                            }
                        } else if ident.to_string().as_str() == "activity_uuid"
                            || ident.to_string().as_str() == "actor_uuid"
                            || ident.to_string().as_str() == "component_uuid"
                            || ident.to_string().as_str() == "finding_uuid"
                            || ident.to_string().as_str() == "observation_uuid"
                            || ident.to_string().as_str() == "party_uuid"
                            || ident.to_string().as_str() == "response_uuid"
                            || ident.to_string().as_str() == "risk_uuid"
                            || ident.to_string().as_str() == "subject_placeholder_uuid"
                            || ident.to_string().as_str() == "subject_uuid"
                            || ident.to_string().as_str() == "subject_uuid"
                            || ident.to_string().as_str() == "task_uuid" {
                            field.attrs.push(syn::parse_quote! {
                                #[derivative(PartialEq="ignore")]
                            })
                        }
                    }
                });

            let ident = item_struct.ident.to_string().clone();

            if ident == "Action" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "date" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_dttz_opt",
                                            deserialize_with = "validation::deser_dttz_opt"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "action_type" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token",
                                            deserialize_with = "validation::deser_token"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "Activity" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "title" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup_opt"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "Address" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "address_type" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token_opt",
                                            deserialize_with = "validation::deser_token_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "AssessedControls" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "AssessmentPart" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "name" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token",
                                            deserialize_with = "validation::deser_token"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "ns" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uri_opt",
                                            deserialize_with = "validation::deser_uri_opt"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "class" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token_opt",
                                            deserialize_with = "validation::deser_token_opt"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "title" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "AssessmentPlatform" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "title" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "AssociatedActivity" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "activity_uuid" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uuid",
                                            deserialize_with = "validation::deser_uuid"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "Base64" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "filename" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token_opt",
                                            deserialize_with = "validation::deser_token_opt"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "value" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_base64",
                                            deserialize_with = "validation::deser_base64"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "Citation" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "text" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "Component" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "component_type" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token",
                                            deserialize_with = "validation::deser_token"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "title" || ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "purpose" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "Constraint" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(deserialize_with = "validation::deser_markup_opt")]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "ControlGroup" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "id" || ident.to_string().clone() == "class" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token_opt",
                                            deserialize_with = "validation::deser_token_opt"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "title" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "DocumentIdentifier" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "scheme" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uri_opt",
                                            deserialize_with = "validation::deser_uri_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "DocumentMetadata" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "title" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "last_modified" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_dttz",
                                            deserialize_with = "validation::deser_dttz"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "Facet" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "name" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token",
                                            deserialize_with = "validation::deser_token"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "system" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uri",
                                            deserialize_with = "validation::deser_uri"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "Finding" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "title" || ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "implementation_statement_uuid" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uuid_opt",
                                            deserialize_with = "validation::deser_uuid_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "FindingRelatedObservation" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "observation_uuid" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uuid",
                                            deserialize_with = "validation::deser_uuid"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "FrequencyCondition" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "period" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_positive_int",
                                            deserialize_with = "validation::deser_positive_int"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "Guideline" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "prose" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "Hash" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "value" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_hash",
                                            deserialize_with = "validation::deser_hash"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "IdentifiedSubject" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "subject_placeholder_uuid" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uuid",
                                            deserialize_with = "validation::deser_uuid"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "IdentifiedRisk" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "title" || ident.to_string().clone() == "description" || ident.to_string().clone() == "statement" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "status" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token",
                                            deserialize_with = "validation::deser_token"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "deadline" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_dttz_opt",
                                            deserialize_with = "validation::deser_dttz_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "IdentifiesTheSubject" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "subject_uuid" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uuid",
                                            deserialize_with = "validation::deser_uuid"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "ImplementedComponent" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "component_uuid" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uuid",
                                            deserialize_with = "validation::deser_uuid"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "ImportSystemSecurityPlan" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "href" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uri_ref",
                                            deserialize_with = "validation::deser_uri_ref"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "InventoryItem" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "Link" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "href" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uri_ref",
                                            deserialize_with = "validation::deser_uri_ref"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "rel" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token_opt",
                                            deserialize_with = "validation::deser_token_opt"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "text" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "Location" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "title" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup_opt"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "email_addresses" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_email_vec_opt",
                                            deserialize_with = "validation::deser_email_vec_opt"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "urls" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uri_vec_opt",
                                            deserialize_with = "validation::deser_uri_vec_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "LoggedBy" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "party_uuid" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uuid",
                                            deserialize_with = "validation::deser_uuid"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "role_id" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token_opt",
                                            deserialize_with = "validation::deser_token_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "MitigatingFactor" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "implementation_uuid" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uuid_opt",
                                            deserialize_with = "validation::deser_uuid_opt"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "Observation" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "title" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup_opt"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "collected" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_dttz",
                                            deserialize_with = "validation::deser_dttz"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "expires" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_dttz_opt",
                                            deserialize_with = "validation::deser_dttz_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "OriginatingActor" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "actor_uuid" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uuid",
                                            deserialize_with = "validation::deser_uuid"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "role_id" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token_opt",
                                            deserialize_with = "validation::deser_token_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "Parameter" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "id" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token",
                                            deserialize_with = "validation::deser_token"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "class" || ident.to_string().clone() == "depends_on" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token_opt",
                                            deserialize_with = "validation::deser_token_opt"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "label" || ident.to_string().clone() == "usage" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "Part" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "id" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token_opt",
                                            deserialize_with = "validation::deser_token_opt"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "name" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token",
                                            deserialize_with = "validation::deser_token"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "ns" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uri_opt",
                                            deserialize_with = "validation::deser_uri_opt"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "class" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token_opt",
                                            deserialize_with = "validation::deser_token_opt"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "title" || ident.to_string().clone() == "prose" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "Party" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "email_addresses" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_email_vec_opt",
                                            deserialize_with = "validation::deser_email_vec_opt"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "member_of_organizations" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uuid_vec_opt",
                                            deserialize_with = "validation::deser_uuid_vec_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "PartyExternalIdentifier" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "scheme" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uri",
                                            deserialize_with = "validation::deser_uri"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "PortRange" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "start" || ident.to_string().clone() == "end" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_non_neg_int_opt",
                                            deserialize_with = "validation::deser_non_neg_int_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "Privilege" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "Property" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "name" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token",
                                            deserialize_with = "validation::deser_token"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "ns" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uri_opt",
                                            deserialize_with = "validation::deser_uri_opt"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "class" || ident.to_string().clone() == "group" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token_opt",
                                            deserialize_with = "validation::deser_token_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "ReferencedControlObjectives" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "RelevantEvidence" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "href" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uri_ref_opt",
                                            deserialize_with = "validation::deser_uri_ref_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "RequiredAsset" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "title" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup_opt"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "Resource" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "title" || ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "ResourceLink" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "href" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uri_ref",
                                            deserialize_with = "validation::deser_uri_ref"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "ResponsibleParty" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "role_id" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token",
                                            deserialize_with = "validation::deser_token"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "party_uuids" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uuid_vec",
                                            deserialize_with = "validation::deser_uuid_vec"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "ResponsibleRole" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "role_id" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token",
                                            deserialize_with = "validation::deser_token"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "party_uuids" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uuid_vec_opt",
                                            deserialize_with = "validation::deser_uuid_vec_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "ReviewedControlsAndControlObjectives" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "RevisionHistoryEntry" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "title" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup_opt"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "published" || ident.to_string().clone() == "last_modified" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_dttz_opt",
                                            deserialize_with = "validation::deser_dttz_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "RiskLogEntry" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "title" || ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup_opt"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "start" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_dttz",
                                            deserialize_with = "validation::deser_dttz"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "end" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_dttz_opt",
                                            deserialize_with = "validation::deser_dttz_opt"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "status_change" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token_opt",
                                            deserialize_with = "validation::deser_token_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "RiskRelatedObservation" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "observation_uuid" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uuid",
                                            deserialize_with = "validation::deser_uuid"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "RiskResponse" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "lifecycle" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token",
                                            deserialize_with = "validation::deser_token"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "title" || ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "RiskResponseReference" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "response_uuid" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uuid",
                                            deserialize_with = "validation::deser_uuid"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "Role" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "id" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token",
                                            deserialize_with = "validation::deser_token"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "SelectAssessmentSubject" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "subject_uuid" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uuid",
                                            deserialize_with = "validation::deser_uuid"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "select_assessment_subject_type" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token",
                                            deserialize_with = "validation::deser_token"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "SelectObjective" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "objective_id" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token",
                                            deserialize_with = "validation::deser_token"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "ServiceProtocolInformation" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "title" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "SetParameterValue" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "param_id" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token",
                                            deserialize_with = "validation::deser_token"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "Step" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "title" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup_opt"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "SubjectOfAssessment" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "subject_of_assessment_type" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token",
                                            deserialize_with = "validation::deser_token"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "SystemIdentification" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "identifier_type" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uri_opt",
                                            deserialize_with = "validation::deser_uri_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "SystemUser" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "title" || ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup_opt"
                                        )]
                                    )
                                ])
                            } else if let Some(ident) = &mut field.ident {
                                if ident.to_string().clone() == "role_ids" {
                                    field.attrs.extend(vec![
                                        syn::parse_quote!(
                                            #[serde(
                                                deserialize_with = "validation::deser_token_vec_opt"
                                            )]
                                        )
                                    ])
                                }
                            }
                        }
                    },
                )
            } else if ident == "TargetClass" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "target_id" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token",
                                            deserialize_with = "validation::deser_token"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "title" || ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "Task" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "task_type" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token",
                                            deserialize_with = "validation::deser_token"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "title" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "TaskDependency" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "task_uuid" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uuid",
                                            deserialize_with = "validation::deser_uuid"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "TaskReference" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "task_uuid" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uuid",
                                            deserialize_with = "validation::deser_uuid"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "ThreatId" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "href" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uri_ref_opt",
                                            deserialize_with = "validation::deser_uri_ref_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "UsesComponent" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "component_uuid" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uuid",
                                            deserialize_with = "validation::deser_uuid"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            }
        } else if let syn::Item::Enum(item_enum) = item {
            item_enum.attrs.extend(vec![
                syn::parse_quote!(
                    #[non_exhaustive]
                ),
                syn::parse_quote!(
                    #[derive(EnumString, Derivative)]
                ),
                syn::parse_quote!(
                    #[derivative(PartialEq)]
                )
            ])
        }
    });

    ast
}

fn process_ast_assessment_plan(ast: &mut syn::File) -> String {
    let ast = process_ast_common(ast);

    ast.items.iter_mut().for_each(|ref mut item| {
        if let syn::Item::Struct(item_struct) = item {
            let ident = item_struct.ident.to_string().clone();

            if ident == "AssessmentSpecificControlObjective" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "control_id" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token",
                                            deserialize_with = "validation::deser_token"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "SelectControl" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "control_id" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token",
                                            deserialize_with = "validation::deser_token"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "statement_ids" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token_vec_opt",
                                            deserialize_with = "validation::deser_token_vec_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            }
        }
    });

    prettyplease::unparse(ast).replace("#[derive(ToBeReplaced)]", "")
}

fn process_ast_assessment_results(ast: &mut syn::File) -> String {
    let ast = process_ast_common(ast);

    ast.items.iter_mut().for_each(|ref mut item| {
        if let syn::Item::Struct(item_struct) = item {
            let ident = item_struct.ident.to_string().clone();

            if ident == "AssessmentLogEntry" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "title" || ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup_opt"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "start" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_dttz",
                                            deserialize_with = "validation::deser_dttz"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "end" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_dttz_opt",
                                            deserialize_with = "validation::deser_dttz_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "AssessmentResult" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "title" || ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "start" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_dttz",
                                            deserialize_with = "validation::deser_dttz"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "end" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_dttz_opt",
                                            deserialize_with = "validation::deser_dttz_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "AssociatedRisk" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "risk_uuid" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uuid",
                                            deserialize_with = "validation::deser_uuid"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "ImportAssessmentPlan" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "href" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uri_ref",
                                            deserialize_with = "validation::deser_uri_ref"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "SelectControl" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "control_id" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token",
                                            deserialize_with = "validation::deser_token"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "statement_ids" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token_vec_opt",
                                            deserialize_with = "validation::deser_token_vec_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            }
        }
    });

    prettyplease::unparse(ast).replace("#[derive(ToBeReplaced)]", "")
}

fn process_ast_poam(ast: &mut syn::File) -> String {
    let ast = process_ast_common(ast);

    ast.items.iter_mut().for_each(|ref mut item| {
        if let syn::Item::Struct(item_struct) = item {
            let ident = item_struct.ident.to_string().clone();

            if ident == "FindingRelatedRisk" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "risk_uuid" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uuid",
                                            deserialize_with = "validation::deser_uuid"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "PoaMItem" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "title" || ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "PoamItemRelatedObservation" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "observation_uuid" || ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uuid",
                                            deserialize_with = "validation::deser_uuid"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "PoamItemRelatedRisk" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "risk_uuid" || ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uuid",
                                            deserialize_with = "validation::deser_uuid"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "RelatedFinding" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "finding_uuid" || ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uuid",
                                            deserialize_with = "validation::deser_uuid"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            }
        }
    });

    prettyplease::unparse(ast).replace("#[derive(ToBeReplaced)]", "")
}

fn process_ast_catalog(ast: &mut syn::File) -> String {
    let ast = process_ast_common(ast);

    ast.items.iter_mut().for_each(|ref mut item| {
        if let syn::Item::Struct(item_struct) = item {
            let ident = item_struct.ident.to_string().clone();

            if ident == "Control" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "id" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token",
                                            deserialize_with = "validation::deser_token"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "title" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "class" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token_opt",
                                            deserialize_with = "validation::deser_token_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            }
        }
    });

    prettyplease::unparse(ast).replace("#[derive(ToBeReplaced)]", "")
}

fn process_ast_profile(ast: &mut syn::File) -> String {
    let ast = process_ast_common(ast);

    ast.items.iter_mut().for_each(|ref mut item| {
        if let syn::Item::Struct(item_struct) = item {
            let ident = item_struct.ident.to_string().clone();

            if ident == "Addition" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "by_id" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token_opt",
                                            deserialize_with = "validation::deser_token_opt"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "title" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "Alteration" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "control_id"  {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token",
                                            deserialize_with = "validation::deser_token"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "ImportResource" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "href" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uri_ref",
                                            deserialize_with = "validation::deser_uri_ref"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "Removal" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "by_name" || ident.to_string().clone() == "by_class" || ident.to_string().clone() == "by_id" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token_opt",
                                            deserialize_with = "validation::deser_token_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "SelectControl" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "with_ids" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token_vec_opt",
                                            deserialize_with = "validation::deser_token_vec_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            }
        }
    });

    prettyplease::unparse(ast).replace("#[derive(ToBeReplaced)]", "")
}

fn process_ast_component_definition(ast: &mut syn::File) -> String {
    let ast = process_ast_common(ast);

    ast.items.iter_mut().for_each(|ref mut item| {
        if let syn::Item::Struct(item_struct) = item {
            let ident = item_struct.ident.to_string().clone();

            if ident == "Capability" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(deserialize_with = "validation::deser_markup")]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "ControlImplementation" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "control_id" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token",
                                            deserialize_with = "validation::deser_token"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "ControlImplementationSet" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(deserialize_with = "validation::deser_markup")]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "ControlStatementImplementation" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "statement_id" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token",
                                            deserialize_with = "validation::deser_token"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "ImportComponentDefinition" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "href" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uri_ref",
                                            deserialize_with = "validation::deser_uri_ref"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "IncorporatesComponent" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "component_uuid" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uuid",
                                            deserialize_with = "validation::deser_uuid"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            }
        }
    });

    prettyplease::unparse(ast).replace("#[derive(ToBeReplaced)]", "")
}

fn process_ast_ssp(ast: &mut syn::File) -> String {
    let ast = process_ast_common(ast);

    ast.items.iter_mut().for_each(|ref mut item| {
        if let syn::Item::Struct(item_struct) = item {
            let ident = item_struct.ident.to_string().clone();

            if ident == "ComponentControlImplementation" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "component_uuid" || ident.to_string().clone() == "class" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uuid",
                                            deserialize_with = "validation::deser_uuid"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "ControlImplementationResponsibility" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "provided_uuid" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uuid_opt",
                                            deserialize_with = "validation::deser_uuid_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "Diagram" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "caption" || ident.to_string().clone() == "class" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "Export" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "ImpactLevel" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "adjustment_justification" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(deserialize_with = "validation::deser_markup_opt")]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "ImportProfile" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "href" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uri_ref",
                                            deserialize_with = "validation::deser_uri_ref"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "InheritedControlImplementation" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "provided_uuid" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uuid_opt",
                                            deserialize_with = "validation::deser_uuid_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "LeveragedAuthorization" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "title" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "party_uuid" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uuid",
                                            deserialize_with = "validation::deser_uuid"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "date_authorized" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_date",
                                            deserialize_with = "validation::deser_date"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "ProvidedControlImplementation" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "SatisfiedControlImplementationResponsibility" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "responsibility_uuid" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_uuid_opt",
                                            deserialize_with = "validation::deser_uuid_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "SpecificControlStatement" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "statement_id" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_token",
                                            deserialize_with = "validation::deser_token"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "SystemCharacteristics" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup"
                                        )]
                                    )
                                ])
                            } else if ident.to_string().clone() == "date_authorized" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            serialize_with = "validation::ser_date_opt",
                                            deserialize_with = "validation::deser_date_opt"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            } else if ident == "ControlImplementation" {
                (&mut item_struct.fields).into_iter().for_each(
                    |ref mut field| {
                        if let Some(ident) = &mut field.ident {
                            if ident.to_string().clone() == "description" {
                                field.attrs.extend(vec![
                                    syn::parse_quote!(
                                        #[serde(
                                            deserialize_with = "validation::deser_markup"
                                        )]
                                    )
                                ])
                            }
                        }
                    },
                )
            }
        }
    });

    prettyplease::unparse(ast).replace("#[derive(ToBeReplaced)]", "")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=jsonschema/");

    let current_dir = env::current_dir()?;
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let assessment_plan_path = format!(
        "{}/jsonschema/tracking_latest_models/oscal_assessment-plan_schema.json",
        current_dir.to_string_lossy()
    );
    let assessment_plan_out_path =
        format!("{}/assessment_plan.rs", out_path.to_string_lossy());

    let assessment_results_path = format!(
        "{}/jsonschema/tracking_latest_models/oscal_assessment-results_schema.json",
        current_dir.to_string_lossy()
    );
    let assessment_results_out_path =
        format!("{}/assessment_results.rs", out_path.to_string_lossy());

    let poam_path = format!(
        "{}/jsonschema/tracking_latest_models/oscal_poam_schema.json",
        current_dir.to_string_lossy()
    );
    let poam_out_path = format!("{}/poam.rs", out_path.to_string_lossy());

    let catalog_path = format!(
        "{}/jsonschema/tracking_latest_models/oscal_catalog_schema.json",
        current_dir.to_string_lossy()
    );
    let catalog_out_path = format!("{}/catalog.rs", out_path.to_string_lossy());

    let profile_path = format!(
        "{}/jsonschema/tracking_latest_models/oscal_profile_schema.json",
        current_dir.to_string_lossy()
    );
    let profile_out_path = format!("{}/profile.rs", out_path.to_string_lossy());

    let component_definition_path = format!(
        "{}/jsonschema/tracking_latest_models/oscal_component_schema.json",
        current_dir.to_string_lossy()
    );
    let component_definition_out_path =
        format!("{}/component_definition.rs", out_path.to_string_lossy());

    let ssp_path = format!(
        "{}/jsonschema/tracking_latest_models/oscal_ssp_schema.json",
        current_dir.to_string_lossy()
    );
    let ssp_path_out_path = format!("{}/ssp.rs", out_path.to_string_lossy());

    check_schema_diff()?;

    std::process::Command::new("quicktype")
        .args([
            &assessment_plan_path,
            "--src-lang",
            "schema",
            "--top-level",
            "AssessmentPlan",
            "--derive-debug",
            "--derive-clone",
            "--leading-comments",
            "--density",
            "dense",
            "--visibility",
            "public",
            "-o",
            &assessment_plan_out_path,
        ])
        .status()
        .expect("quicktype not found");

    std::process::Command::new("quicktype")
        .args([
            &assessment_results_path,
            "--src-lang",
            "schema",
            "--top-level",
            "AssessmentResults",
            "--derive-debug",
            "--derive-clone",
            "--leading-comments",
            "--density",
            "dense",
            "--visibility",
            "public",
            "-o",
            &assessment_results_out_path,
        ])
        .status()
        .expect("quicktype not found");

    std::process::Command::new("quicktype")
        .args([
            &poam_path,
            "--src-lang",
            "schema",
            "--top-level",
            "PlanOfActionAndMilestones",
            "--derive-debug",
            "--derive-clone",
            "--leading-comments",
            "--density",
            "dense",
            "--visibility",
            "public",
            "-o",
            &poam_out_path,
        ])
        .status()
        .expect("quicktype not found");

    std::process::Command::new("quicktype")
        .args([
            &catalog_path,
            "--src-lang",
            "schema",
            "--top-level",
            "Catalog",
            "--derive-debug",
            "--derive-clone",
            "--leading-comments",
            "--density",
            "dense",
            "--visibility",
            "public",
            "-o",
            &catalog_out_path,
        ])
        .status()
        .expect("quicktype not found");

    std::process::Command::new("quicktype")
        .args([
            &profile_path,
            "--src-lang",
            "schema",
            "--top-level",
            "Profile",
            "--derive-debug",
            "--derive-clone",
            "--leading-comments",
            "--density",
            "dense",
            "--visibility",
            "public",
            "-o",
            &profile_out_path,
        ])
        .status()
        .expect("quicktype not found");

    std::process::Command::new("quicktype")
        .args([
            &component_definition_path,
            "--src-lang",
            "schema",
            "--top-level",
            "ComponentDefinition",
            "--derive-debug",
            "--derive-clone",
            "--leading-comments",
            "--density",
            "dense",
            "--visibility",
            "public",
            "-o",
            &component_definition_out_path,
        ])
        .status()
        .expect("quicktype not found");

    std::process::Command::new("quicktype")
        .args([
            &ssp_path,
            "--src-lang",
            "schema",
            "--top-level",
            "SystemSecurityPlan",
            "--derive-debug",
            "--derive-clone",
            "--leading-comments",
            "--density",
            "dense",
            "--visibility",
            "public",
            "-o",
            &ssp_path_out_path,
        ])
        .status()
        .expect("quicktype not found");

    let assessment_plan = read_file(assessment_plan_out_path)?;
    let assessment_results = read_file(assessment_results_out_path)?;
    let poam = read_file(poam_out_path)?;
    let catalog = read_file(catalog_out_path)?;
    let profile = read_file(profile_out_path)?;
    let component_definition = read_file(component_definition_out_path)?;
    let ssp = read_file(ssp_path_out_path)?;

    let mut assessment_plan_ast = syn::parse_file(&assessment_plan)?;
    let mut assessment_results_ast = syn::parse_file(&assessment_results)?;
    let mut poam_ast = syn::parse_file(&poam)?;
    let mut catalog_ast = syn::parse_file(&catalog)?;
    let mut profile_ast = syn::parse_file(&profile)?;
    let mut component_definition_ast = syn::parse_file(&component_definition)?;
    let mut ssp_ast = syn::parse_file(&ssp)?;

    let assessment_plan_str =
        process_ast_assessment_plan(&mut assessment_plan_ast);
    let assessment_results_str =
        process_ast_assessment_results(&mut assessment_results_ast);
    let poam_str = process_ast_poam(&mut poam_ast);
    let catalog_str = process_ast_catalog(&mut catalog_ast);
    let profile_str = process_ast_profile(&mut profile_ast);
    let component_definition_str =
        process_ast_component_definition(&mut component_definition_ast);
    let ssp_str = process_ast_ssp(&mut ssp_ast);

    create_outout_path(
        "assessment_plan.rs",
        "assessment",
        assessment_plan_str,
    )?;
    create_outout_path(
        "assessment_results.rs",
        "assessment",
        assessment_results_str,
    )?;
    create_outout_path("poam.rs", "assessment", poam_str)?;
    create_outout_path("catalog.rs", "control", catalog_str)?;
    create_outout_path("profile.rs", "control", profile_str)?;
    create_outout_path(
        "component_definition.rs",
        "implementation",
        component_definition_str,
    )?;
    create_outout_path("ssp.rs", "implementation", ssp_str)?;

    Ok(())
}
