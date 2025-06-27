use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::preceded,
    Parser,
};

use crate::{input::Input, intermediate::*};

use super::{common::*, constraint::constraint, error::ParserResult};

/// Tries to parse an ASN1 OCTET STRING
///
/// *`input` - [Input]-wrapped string slice to be matched against
///
/// `octet_string` will try to match an OCTET STRING declaration in the `input` string.
/// If the match succeeds, the lexer will consume the match and return the remaining string
/// and a wrapped `OctetString` value representing the ASN1 declaration.
/// If the match fails, the lexer will not consume the input and will return an error.
pub fn octet_string(input: Input<'_>) -> ParserResult<'_, ASN1Type> {
    map(
        preceded(skip_ws_and_comments(tag(OCTET_STRING)), opt(constraint)),
        |m| ASN1Type::OctetString(m.into()),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use crate::intermediate::{constraints::*, types::*, *};

    use super::octet_string;

    #[test]
    fn parses_unconfined_octetstring() {
        let sample = "  OCTET STRING".into();
        assert_eq!(
            octet_string(sample).unwrap().1,
            ASN1Type::OctetString(OctetString {
                constraints: vec![]
            })
        )
    }

    #[test]
    fn parses_strictly_constrained_octetstring() {
        let sample = "  OCTET STRING(SIZE (8))".into();
        assert_eq!(
            octet_string(sample).unwrap().1,
            ASN1Type::OctetString(OctetString {
                constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                    set: ElementOrSetOperation::Element(SubtypeElement::SizeConstraint(Box::new(
                        ElementOrSetOperation::Element(SubtypeElement::SingleValue {
                            value: ASN1Value::Integer(8),
                            extensible: false
                        })
                    ))),
                    extensible: false
                })]
            })
        )
    }

    #[test]
    fn parses_range_constrained_octetstring() {
        let sample = "  OCTET STRING -- even here?!?!? -- (SIZE (8 ..18))".into();
        assert_eq!(
            octet_string(sample).unwrap().1,
            ASN1Type::OctetString(OctetString {
                constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                    set: ElementOrSetOperation::Element(SubtypeElement::SizeConstraint(Box::new(
                        ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                            min: Some(ASN1Value::Integer(8)),
                            max: Some(ASN1Value::Integer(18)),
                            extensible: false
                        })
                    ))),
                    extensible: false
                })]
            })
        )
    }

    #[test]
    fn parses_strictly_constrained_extended_octetstring() {
        let sample = "  OCTET STRING (SIZE (2, ...))".into();
        assert_eq!(
            octet_string(sample).unwrap().1,
            ASN1Type::OctetString(OctetString {
                constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                    set: ElementOrSetOperation::Element(SubtypeElement::SizeConstraint(Box::new(
                        ElementOrSetOperation::Element(SubtypeElement::SingleValue {
                            value: ASN1Value::Integer(2),
                            extensible: true
                        })
                    ))),
                    extensible: false
                })]
            })
        )
    }

    #[test]
    fn parses_range_constrained_extended_octetstring() {
        let sample = "  OCTET STRING (SIZE (8 -- comment -- .. 18, ...))".into();
        assert_eq!(
            octet_string(sample).unwrap().1,
            ASN1Type::OctetString(OctetString {
                constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                    set: ElementOrSetOperation::Element(SubtypeElement::SizeConstraint(Box::new(
                        ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                            min: Some(ASN1Value::Integer(8)),
                            max: Some(ASN1Value::Integer(18)),
                            extensible: true
                        })
                    ))),
                    extensible: false
                })]
            })
        )
    }
}
