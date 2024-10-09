use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, one_of},
    combinator::{into, map, opt, recognize, value},
    multi::{many0, many1, separated_list0, separated_list1},
    sequence::{pair, preceded, terminated, tuple},
    IResult,
};

use crate::intermediate::{information_object::*, types::ObjectIdentifier, *};

use super::{
    asn1_type, asn1_value,
    common::{
        default, extension_marker, identifier, in_braces, in_brackets, optional_comma,
        optional_marker, skip_ws_and_comments, uppercase_identifier,
    },
    constraint::constraint,
    input::Input,
    into_inner,
};

/// Tries to parse an ASN1 TYPE-IDENTIFIER
///
/// *`input` - string slice to be matched against
///
/// ## _X681:_
/// _Annex A: The TYPE-IDENTIFIER information object class is defined as:
/// ```ignore
/// TYPE-IDENTIFIER ::= CLASS
/// {
/// 	&id OBJECT IDENTIFIER UNIQUE,
/// 	&Type
/// }
/// WITH SYNTAX {&Type IDENTIFIED BY &id}
/// ```
pub fn type_identifier(input: Input<'_>) -> IResult<Input<'_>, InformationObjectClass> {
    skip_ws_and_comments(value(
        InformationObjectClass {
            fields: vec![
                InformationObjectClassField {
                    identifier: ObjectFieldIdentifier::SingleValue("id".into()),
                    ty: Some(ASN1Type::ObjectIdentifier(ObjectIdentifier {
                        constraints: vec![],
                    })),
                    is_optional: false,
                    default: None,
                    is_unique: true,
                },
                InformationObjectClassField {
                    identifier: ObjectFieldIdentifier::MultipleValue("Type".into()),
                    ty: None,
                    is_optional: false,
                    default: None,
                    is_unique: false,
                },
            ],
            syntax: None,
        },
        tag(TYPE_IDENTIFIER),
    ))(input)
}

/// Tries to parse an ASN1 INSTANCE OF
///
/// *`input` - string slice to be matched against
///
/// ## _X680:_
/// _G.2.18: Use an instance-of to specify a type containing an object identifier field_
/// _and an open type value whose type is determined by the object identifier._
pub fn instance_of(input: Input<'_>) -> IResult<Input<'_>, ASN1Type> {
    map(
        preceded(
            tag(INSTANCE_OF),
            pair(
                skip_ws_and_comments(uppercase_identifier),
                skip_ws_and_comments(constraint),
            ),
        ),
        |(id, constraints)| {
            ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                parent: None,
                identifier: id.into(),
                constraints,
            })
        },
    )(input)
}

pub fn information_object_class(input: Input<'_>) -> IResult<Input<'_>, InformationObjectClass> {
    into(preceded(
        skip_ws_and_comments(tag(CLASS)),
        pair(
            in_braces(many0(terminated(
                skip_ws_and_comments(information_object_field),
                optional_comma,
            ))),
            opt(preceded(skip_ws_and_comments(tag(WITH_SYNTAX)), syntax)),
        ),
    ))(input)
}

pub fn information_object_field_reference(
    input: Input<'_>,
) -> IResult<Input<'_>, InformationObjectFieldReference> {
    into(tuple((
        skip_ws_and_comments(uppercase_identifier),
        many1(skip_ws_and_comments(preceded(
            char(DOT),
            skip_ws_and_comments(object_field_identifier),
        ))),
        opt(constraint),
    )))(input)
}

pub fn information_object(input: Input<'_>) -> IResult<Input<'_>, InformationObjectFields> {
    in_braces(alt((
        default_syntax_information_object,
        custom_syntax_information_object,
    )))(input)
}

pub fn object_set(input: Input<'_>) -> IResult<Input<'_>, ObjectSet> {
    into(in_braces(tuple((
        separated_list0(
            skip_ws_and_comments(alt((tag(PIPE), tag(UNION)))),
            skip_ws_and_comments(alt((
                into(information_object),
                into(skip_ws_and_comments(identifier)),
            ))),
        ),
        opt(skip_ws_and_comments(preceded(
            opt(char(COMMA)),
            extension_marker,
        ))),
        opt(skip_ws_and_comments(preceded(
            char(COMMA),
            separated_list1(
                skip_ws_and_comments(alt((tag(PIPE), tag(UNION)))),
                skip_ws_and_comments(alt((
                    into(information_object),
                    into(skip_ws_and_comments(identifier)),
                ))),
            ),
        ))),
    ))))(input)
}

fn custom_syntax_information_object(
    input: Input<'_>,
) -> IResult<Input<'_>, InformationObjectFields> {
    map(
        skip_ws_and_comments(many1(skip_ws_and_comments(alt((
            value(SyntaxApplication::Comma, char(COMMA)),
            map(asn1_type, |m| match m {
                ASN1Type::ElsewhereDeclaredType(t) => SyntaxApplication::LiteralOrTypeReference(t),
                t => SyntaxApplication::TypeReference(t),
            }),
            map(asn1_value, SyntaxApplication::ValueReference),
            map(object_set, SyntaxApplication::ObjectSetDeclaration),
            map(syntax_literal, |m| SyntaxApplication::Literal(m.into())),
        ))))),
        InformationObjectFields::CustomSyntax,
    )(input)
}

fn default_syntax_information_object(
    input: Input<'_>,
) -> IResult<Input<'_>, InformationObjectFields> {
    map(
        many1(terminated(
            skip_ws_and_comments(alt((
                into(pair(
                    single_value_field_id,
                    skip_ws_and_comments(asn1_value),
                )),
                into(pair(multiple_value_field_id, object_set)),
                into(pair(
                    multiple_value_field_id,
                    skip_ws_and_comments(asn1_type),
                )),
            ))),
            optional_comma,
        )),
        InformationObjectFields::DefaultSyntax,
    )(input)
}

fn information_object_field(input: Input<'_>) -> IResult<Input<'_>, InformationObjectClassField> {
    into(tuple((
        skip_ws_and_comments(object_field_identifier),
        opt(skip_ws_and_comments(asn1_type)),
        opt(into_inner(skip_ws_and_comments(tag(UNIQUE)))),
        optional_marker,
        default,
    )))(input)
}

fn object_field_identifier(input: Input<'_>) -> IResult<Input<'_>, ObjectFieldIdentifier> {
    alt((single_value_field_id, multiple_value_field_id))(input)
}

fn single_value_field_id(input: Input<'_>) -> IResult<Input<'_>, ObjectFieldIdentifier> {
    map(
        into_inner(recognize(tuple((
            char(AMPERSAND),
            one_of("abcdefghijklmnopqrstuvwxyz"),
            many0(alt((preceded(char('-'), alphanumeric1), alphanumeric1))),
        )))),
        |s| ObjectFieldIdentifier::SingleValue(String::from(s)),
    )(input)
}

fn multiple_value_field_id(input: Input<'_>) -> IResult<Input<'_>, ObjectFieldIdentifier> {
    map(
        into_inner(recognize(tuple((
            char(AMPERSAND),
            one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
            many0(alt((preceded(char('-'), alphanumeric1), alphanumeric1))),
        )))),
        |m| ObjectFieldIdentifier::MultipleValue(String::from(m)),
    )(input)
}

fn syntax(input: Input<'_>) -> IResult<Input<'_>, Vec<SyntaxExpression>> {
    in_braces(many1(syntax_token_or_group_spec))(input)
}

fn syntax_token_or_group_spec(input: Input<'_>) -> IResult<Input<'_>, SyntaxExpression> {
    alt((
        map(syntax_token, SyntaxExpression::Required),
        map(syntax_optional_group, SyntaxExpression::Optional),
    ))(input)
}

fn syntax_optional_group(input: Input<'_>) -> IResult<Input<'_>, Vec<SyntaxExpression>> {
    in_brackets(skip_ws_and_comments(many1(syntax_token_or_group_spec)))(input)
}

fn syntax_token(input: Input<'_>) -> IResult<Input<'_>, SyntaxToken> {
    skip_ws_and_comments(alt((
        map(syntax_literal, SyntaxToken::from),
        map(object_field_identifier, SyntaxToken::from),
        map(
            into_inner(tag(COMMA.to_string().as_str())),
            SyntaxToken::from,
        ),
    )))(input)
}

fn syntax_literal(input: Input<'_>) -> IResult<Input<'_>, &str> {
    uppercase_identifier(input)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::intermediate::types::*;

    use crate::lexer::information_object_class::{information_object_class, object_set};
    use crate::lexer::top_level_type_declaration;

    use super::*;

    #[test]
    fn parses_information_object_class() {
        assert_eq!(
            information_object_class(
                r#"CLASS
      {&operationCode CHOICE {local INTEGER,
      global OCTET STRING}
      UNIQUE,
      &ArgumentType,
      &ResultType,
      &Errors ERROR OPTIONAL }"#
                    .into()
            )
            .unwrap()
            .1,
            InformationObjectClass {
                syntax: None,
                fields: vec![
                    InformationObjectClassField {
                        identifier: ObjectFieldIdentifier::SingleValue("&operationCode".into()),
                        ty: Some(ASN1Type::Choice(Choice {
                            extensible: None,
                            options: vec![
                                ChoiceOption {
                                    is_recursive: false,
                                    name: "local".into(),
                                    tag: None,
                                    ty: ASN1Type::Integer(Integer {
                                        constraints: vec![],
                                        distinguished_values: None,
                                    }),
                                    constraints: vec![]
                                },
                                ChoiceOption {
                                    is_recursive: false,
                                    name: "global".into(),
                                    tag: None,
                                    ty: ASN1Type::OctetString(OctetString {
                                        constraints: vec![],
                                    }),
                                    constraints: vec![]
                                }
                            ],
                            constraints: vec![]
                        })),
                        is_optional: false,
                        is_unique: true,
                        default: None
                    },
                    InformationObjectClassField {
                        identifier: ObjectFieldIdentifier::MultipleValue("&ArgumentType".into()),
                        ty: None,
                        is_optional: false,
                        is_unique: false,
                        default: None
                    },
                    InformationObjectClassField {
                        identifier: ObjectFieldIdentifier::MultipleValue("&ResultType".into()),
                        ty: None,
                        is_optional: false,
                        is_unique: false,
                        default: None
                    },
                    InformationObjectClassField {
                        identifier: ObjectFieldIdentifier::MultipleValue("&Errors".into()),
                        ty: Some(ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                            parent: None,
                            constraints: vec![],
                            identifier: "ERROR".into()
                        })),
                        is_optional: true,
                        is_unique: false,
                        default: None
                    }
                ]
            }
        )
    }

    #[test]
    fn parses_simple_object_set() {
        assert_eq!(
            object_set(r#"{My-ops}"#.into()).unwrap().1,
            ObjectSet {
                values: vec![ObjectSetValue::Reference("My-ops".into())],
                extensible: None
            }
        )
    }

    #[test]
    fn parses_extended_value_set() {
        assert_eq!(
            object_set(r#"{My-ops | Other-ops, ...}"#.into()).unwrap().1,
            ObjectSet {
                values: vec![
                    ObjectSetValue::Reference("My-ops".into()),
                    ObjectSetValue::Reference("Other-ops".into())
                ],
                extensible: Some(2)
            }
        )
    }

    #[test]
    fn parses_inline_declaration_value_set() {
        assert_eq!(
            object_set(
                r#"{
                {&errorCode asn-val-security-failure, &ParameterType BOOLEAN} |
                {&errorCode asn-val-unknown-order},
                ...
            }"#
                .into()
            )
            .unwrap()
            .1,
            ObjectSet {
                values: vec![
                    ObjectSetValue::Inline(InformationObjectFields::DefaultSyntax(vec![
                        InformationObjectField::FixedValueField(FixedValueField {
                            identifier: "&errorCode".to_string(),
                            value: ASN1Value::ElsewhereDeclaredValue {
                                identifier: "asn-val-security-failure".into(),
                                parent: None
                            }
                        }),
                        InformationObjectField::TypeField(TypeField {
                            identifier: "&ParameterType".into(),
                            ty: ASN1Type::Boolean(Boolean {
                                constraints: vec![]
                            })
                        })
                    ])),
                    ObjectSetValue::Inline(InformationObjectFields::DefaultSyntax(vec![
                        InformationObjectField::FixedValueField(FixedValueField {
                            identifier: "&errorCode".into(),
                            value: ASN1Value::ElsewhereDeclaredValue {
                                identifier: "asn-val-unknown-order".into(),
                                parent: None
                            }
                        })
                    ])),
                ],
                extensible: Some(2)
            }
        )
    }

    #[test]
    fn parses_information_object_class_with_custom_syntax() {
        assert_eq!(
            information_object_class(
                r#"CLASS{
            &itsaidCtxRef ItsAidCtxRef UNIQUE,
            &ContextInfo OPTIONAL
            }
            WITH SYNTAX {&ContextInfo IDENTIFIED BY &itsaidCtxRef}"#
                    .into()
            )
            .unwrap()
            .1,
            InformationObjectClass {
                fields: vec![
                    InformationObjectClassField {
                        identifier: ObjectFieldIdentifier::SingleValue("&itsaidCtxRef".into()),
                        ty: Some(ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                            parent: None,
                            identifier: "ItsAidCtxRef".into(),
                            constraints: vec![]
                        })),
                        is_optional: false,
                        default: None,
                        is_unique: true
                    },
                    InformationObjectClassField {
                        identifier: ObjectFieldIdentifier::MultipleValue("&ContextInfo".into()),
                        ty: None,
                        is_optional: true,
                        default: None,
                        is_unique: false
                    }
                ],
                syntax: Some(InformationObjectSyntax {
                    expressions: vec![
                        SyntaxExpression::Required(SyntaxToken::Field(
                            ObjectFieldIdentifier::MultipleValue("&ContextInfo".into())
                        )),
                        SyntaxExpression::Required(SyntaxToken::Literal("IDENTIFIED".into())),
                        SyntaxExpression::Required(SyntaxToken::Literal("BY".into())),
                        SyntaxExpression::Required(SyntaxToken::Field(
                            ObjectFieldIdentifier::SingleValue("&itsaidCtxRef".into())
                        ))
                    ]
                })
            }
        )
    }

    #[test]
    fn parses_information_object_with_custom_syntax() {
        println!(
            "{:?}",
            information_object_class(
                r#"CLASS {&id    BilateralDomain UNIQUE,
            &Type
}WITH SYNTAX {&Type,
     IDENTIFIED BY &id
}"#
                .into()
            )
            .unwrap()
        )
    }

    #[test]
    fn parses_top_level_information_object_field_ref() {
        assert_eq!(
            top_level_type_declaration(r#"AttributeValue ::= OPEN.&Type"#.into())
                .unwrap()
                .1,
            ToplevelTypeDefinition {
                comments: "".into(),
                tag: None,
                name: "AttributeValue".into(),
                ty: ASN1Type::InformationObjectFieldReference(InformationObjectFieldReference {
                    class: "OPEN".into(),
                    field_path: vec![ObjectFieldIdentifier::MultipleValue("&Type".into())],
                    constraints: vec![]
                }),
                parameterization: None,
                index: None
            }
        )
    }

    #[test]
    fn tld_test() {
        println!(
            "{:?}",
            super::information_object_class(
                r#"CLASS {
                    &InitiatingMessage				,
                    &SuccessfulOutcome							OPTIONAL,
                    &UnsuccessfulOutcome						OPTIONAL,
                    &procedureCode				ProcedureCode	UNIQUE,
                    &criticality				Criticality	DEFAULT ignore
                }
                
                WITH SYNTAX {
                    INITIATING MESSAGE			&InitiatingMessage
                    [SUCCESSFUL OUTCOME			&SuccessfulOutcome]
                    [UNSUCCESSFUL OUTCOME		&UnsuccessfulOutcome]
                    PROCEDURE CODE				&procedureCode
                    [CRITICALITY				&criticality]
                }"#
                .into()
            )
        )
    }
}
