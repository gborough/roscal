use derivative::Derivative;
use strum::EnumString;
use derive_builder::Builder;
use serde::{Serialize, Deserialize};
use crate::validation;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Catalog {
    #[serde(rename = "$schema")]
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    pub catalog: CatalogClass,
}

/// A structured, organized collection of control information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CatalogClass {
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub back_matter: Option<BackMatter>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub controls: Option<Vec<Control>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<ControlGroup>>,
    pub metadata: DocumentMetadata,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<Parameter>>,
    /// Provides a globally unique means to identify a given catalog instance.
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    #[derivative(PartialEq = "ignore")]
    pub uuid: String,
}

/// A collection of resources that may be referenced from within the OSCAL document instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct BackMatter {
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<Resource>>,
}

/// A resource associated with content in the containing document instance. A resource may be
/// directly included in the document using base64 encoding or may point to one or more
/// equivalent internet resources.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Resource {
    /// A resource encoded using the Base64 alphabet defined by RFC 2045.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base64: Option<Base64>,
    /// An optional citation consisting of end note text using structured markup.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub citation: Option<Citation>,
    /// An optional short summary of the resource used to indicate the purpose of the resource.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub description: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_ids: Option<Vec<DocumentIdentifier>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rlinks: Option<Vec<ResourceLink>>,
    /// An optional name given to the resource, which may be used by a tool for display and
    /// navigation.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub title: Option<String>,
    /// A unique identifier for a resource.
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    #[derivative(PartialEq = "ignore")]
    pub uuid: String,
}

/// A resource encoded using the Base64 alphabet defined by RFC 2045.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Base64 {
    /// Name of the file before it was encoded as Base64 to be embedded in a resource. This is
    /// the name that will be assigned to the file when the file is decoded.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_token_opt",
        deserialize_with = "validation::deser_token_opt"
    )]
    pub filename: Option<String>,
    /// A label that indicates the nature of a resource, as a data serialization or format.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(
        serialize_with = "validation::ser_base64",
        deserialize_with = "validation::deser_base64"
    )]
    pub value: String,
}

/// An optional citation consisting of end note text using structured markup.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Citation {
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
    /// A line of citation text.
    #[serde(deserialize_with = "validation::deser_markup")]
    pub text: String,
}

/// A reference to a local or remote resource, that has a specific relation to the containing
/// object.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Link {
    /// A resolvable URL reference to a resource.
    #[serde(
        serialize_with = "validation::ser_uri_ref",
        deserialize_with = "validation::deser_uri_ref"
    )]
    pub href: String,
    /// A label that indicates the nature of a resource, as a data serialization or format.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    /// Describes the type of relationship provided by the link's hypertext reference. This can
    /// be an indicator of the link's purpose.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_token_opt",
        deserialize_with = "validation::deser_token_opt"
    )]
    pub rel: Option<String>,
    /// In case where the href points to a back-matter/resource, this value will indicate the URI
    /// fragment to append to any rlink associated with the resource. This value MUST be URI
    /// encoded.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_fragment: Option<String>,
    /// A textual label to associate with the link, which may be used for presentation in a tool.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub text: Option<String>,
}

/// An attribute, characteristic, or quality of the containing object expressed as a
/// namespace qualified name/value pair.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Property {
    /// A textual label that provides a sub-type or characterization of the property's name.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_token_opt",
        deserialize_with = "validation::deser_token_opt"
    )]
    pub class: Option<String>,
    /// An identifier for relating distinct sets of properties.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_token_opt",
        deserialize_with = "validation::deser_token_opt"
    )]
    pub group: Option<String>,
    /// A textual label, within a namespace, that uniquely identifies a specific attribute,
    /// characteristic, or quality of the property's containing object.
    #[serde(
        serialize_with = "validation::ser_token",
        deserialize_with = "validation::deser_token"
    )]
    pub name: String,
    /// A namespace qualifying the property's name. This allows different organizations to
    /// associate distinct semantics with the same name.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_uri_opt",
        deserialize_with = "validation::deser_uri_opt"
    )]
    pub ns: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
    /// A unique identifier for a property.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_uuid_opt",
        deserialize_with = "validation::deser_uuid_opt"
    )]
    #[derivative(PartialEq = "ignore")]
    pub uuid: Option<String>,
    /// Indicates the value of the attribute, characteristic, or quality.
    pub value: String,
}

/// A document identifier qualified by an identifier scheme.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DocumentIdentifier {
    pub identifier: String,
    /// Qualifies the kind of document identifier using a URI. If the scheme is not provided the
    /// value of the element will be interpreted as a string of characters.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_uri_opt",
        deserialize_with = "validation::deser_uri_opt"
    )]
    pub scheme: Option<String>,
}

/// A URL-based pointer to an external resource with an optional hash for verification and
/// change detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ResourceLink {
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hashes: Option<Vec<Hash>>,
    /// A resolvable URL pointing to the referenced resource.
    #[serde(
        serialize_with = "validation::ser_uri_ref",
        deserialize_with = "validation::deser_uri_ref"
    )]
    pub href: String,
    /// A label that indicates the nature of a resource, as a data serialization or format.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
}

/// A representation of a cryptographic digest generated over a resource using a specified
/// hash algorithm.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Hash {
    /// The digest method by which a hash is derived.
    pub algorithm: String,
    #[serde(
        serialize_with = "validation::ser_hash",
        deserialize_with = "validation::deser_hash"
    )]
    pub value: String,
}

/// A structured object representing a requirement or guideline, which when implemented will
/// reduce an aspect of risk related to an information system and its information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Control {
    /// A textual label that provides a sub-type or characterization of the control.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_token_opt",
        deserialize_with = "validation::deser_token_opt"
    )]
    pub class: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub controls: Option<Vec<Control>>,
    /// Identifies a control such that it can be referenced in the defining catalog and other
    /// OSCAL instances (e.g., profiles).
    #[serde(
        serialize_with = "validation::ser_token",
        deserialize_with = "validation::deser_token"
    )]
    pub id: String,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<Parameter>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<Part>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
    /// A name given to the control, which may be used by a tool for display and navigation.
    #[serde(deserialize_with = "validation::deser_markup")]
    pub title: String,
}

/// Parameters provide a mechanism for the dynamic assignment of value(s) in a control.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Parameter {
    /// A textual label that provides a characterization of the type, purpose, use or scope of
    /// the parameter.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_token_opt",
        deserialize_with = "validation::deser_token_opt"
    )]
    pub class: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Vec<Constraint>>,
    /// (deprecated) Another parameter invoking this one. This construct has been deprecated and
    /// should not be used.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_token_opt",
        deserialize_with = "validation::deser_token_opt"
    )]
    pub depends_on: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guidelines: Option<Vec<Guideline>>,
    /// A unique identifier for the parameter.
    #[serde(
        serialize_with = "validation::ser_token",
        deserialize_with = "validation::deser_token"
    )]
    pub id: String,
    /// A short, placeholder name for the parameter, which can be used as a substitute for a
    /// value if no value is assigned.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub label: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub select: Option<Selection>,
    /// Describes the purpose and use of a parameter.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub usage: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// A formal or informal expression of a constraint or test.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Constraint {
    /// A textual summary of the constraint to be applied.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub description: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tests: Option<Vec<ConstraintTest>>,
}

/// A test expression which is expected to be evaluated by a tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ConstraintTest {
    /// A formal (executable) expression of a constraint.
    pub expression: String,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
}

/// A prose statement that provides a recommendation for the use of a parameter.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Guideline {
    /// Prose permits multiple paragraphs, lists, tables etc.
    #[serde(deserialize_with = "validation::deser_markup")]
    pub prose: String,
}

/// Presenting a choice among alternatives.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Selection {
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub choice: Option<Vec<String>>,
    /// Describes the number of selections that must occur. Without this setting, only one value
    /// should be assumed to be permitted.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub how_many: Option<ParameterCardinality>,
}
/// Describes the number of selections that must occur. Without this setting, only one value
/// should be assumed to be permitted.
///
/// Name of the file before it was encoded as Base64 to be embedded in a resource. This is
/// the name that will be assigned to the file when the file is decoded.
///
/// A non-colonized name as defined by XML Schema Part 2: Datatypes Second Edition.
/// https://www.w3.org/TR/xmlschema11-2/#NCName.
///
/// A textual label that provides a sub-type or characterization of the property's name.
///
/// An identifier for relating distinct sets of properties.
///
/// A textual label, within a namespace, that uniquely identifies a specific attribute,
/// characteristic, or quality of the property's containing object.
///
/// A textual label that provides a sub-type or characterization of the control.
///
/// Identifies a control such that it can be referenced in the defining catalog and other
/// OSCAL instances (e.g., profiles).
///
/// A textual label that provides a characterization of the type, purpose, use or scope of
/// the parameter.
///
/// (deprecated) Another parameter invoking this one. This construct has been deprecated and
/// should not be used.
///
/// A unique identifier for the parameter.
///
/// An optional textual providing a sub-type or characterization of the part's name, or a
/// category to which the part belongs.
///
/// A unique identifier for the part.
///
/// A textual label that uniquely identifies the part's semantic type, which exists in a
/// value space qualified by the ns.
///
/// A textual label that provides a sub-type or characterization of the group.
///
/// Identifies the group for the purpose of cross-linking within the defining instance or
/// from other instances that reference the catalog.
///
/// A reference to a role performed by a party.
///
/// The type of action documented by the assembly, such as an approval.
///
/// A unique identifier for the role.
///
/// Describes the type of relationship provided by the link's hypertext reference. This can
/// be an indicator of the link's purpose.
///
/// Indicates the type of address.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
#[derive(EnumString, Derivative)]
#[derivative(PartialEq)]
pub enum ParameterCardinality {
    One,
    #[serde(rename = "one-or-more")]
    OneOrMore,
}

/// An annotated, markup-based textual element of a control's or catalog group's definition,
/// or a child of another part.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Part {
    /// An optional textual providing a sub-type or characterization of the part's name, or a
    /// category to which the part belongs.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_token_opt",
        deserialize_with = "validation::deser_token_opt"
    )]
    pub class: Option<String>,
    /// A unique identifier for the part.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_token_opt",
        deserialize_with = "validation::deser_token_opt"
    )]
    pub id: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    /// A textual label that uniquely identifies the part's semantic type, which exists in a
    /// value space qualified by the ns.
    #[serde(
        serialize_with = "validation::ser_token",
        deserialize_with = "validation::deser_token"
    )]
    pub name: String,
    /// An optional namespace qualifying the part's name. This allows different organizations to
    /// associate distinct semantics with the same name.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_uri_opt",
        deserialize_with = "validation::deser_uri_opt"
    )]
    pub ns: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<Part>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
    /// Permits multiple paragraphs, lists, tables etc.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub prose: Option<String>,
    /// An optional name given to the part, which may be used by a tool for display and
    /// navigation.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub title: Option<String>,
}

/// A group of controls, or of groups of controls.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ControlGroup {
    /// A textual label that provides a sub-type or characterization of the group.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_token_opt",
        deserialize_with = "validation::deser_token_opt"
    )]
    pub class: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub controls: Option<Vec<Control>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<ControlGroup>>,
    /// Identifies the group for the purpose of cross-linking within the defining instance or
    /// from other instances that reference the catalog.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_token_opt",
        deserialize_with = "validation::deser_token_opt"
    )]
    pub id: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<Parameter>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<Part>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
    /// A name given to the group, which may be used by a tool for display and navigation.
    #[serde(deserialize_with = "validation::deser_markup")]
    pub title: String,
}

/// Provides information about the containing document, and defines concepts that are shared
/// across the document.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DocumentMetadata {
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Action>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_ids: Option<Vec<DocumentIdentifier>>,
    #[serde(
        serialize_with = "validation::ser_dttz",
        deserialize_with = "validation::deser_dttz"
    )]
    pub last_modified: String,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<Location>>,
    pub oscal_version: String,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parties: Option<Vec<Party>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub published: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub responsible_parties: Option<Vec<ResponsibleParty>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revisions: Option<Vec<RevisionHistoryEntry>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<Role>>,
    /// A name given to the document, which may be used by a tool for display and navigation.
    #[serde(deserialize_with = "validation::deser_markup")]
    pub title: String,
    pub version: String,
}

/// An action applied by a role within a given party to the content.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Action {
    /// The date and time when the action occurred.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_dttz_opt",
        deserialize_with = "validation::deser_dttz_opt"
    )]
    pub date: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub responsible_parties: Option<Vec<ResponsibleParty>>,
    /// Specifies the action type system used.
    pub system: String,
    /// The type of action documented by the assembly, such as an approval.
    #[serde(rename = "type")]
    #[serde(
        serialize_with = "validation::ser_token",
        deserialize_with = "validation::deser_token"
    )]
    pub action_type: String,
    /// A unique identifier that can be used to reference this defined action elsewhere in an
    /// OSCAL document. A UUID should be consistently used for a given location across revisions
    /// of the document.
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    #[derivative(PartialEq = "ignore")]
    pub uuid: String,
}

/// A reference to a set of persons and/or organizations that have responsibility for
/// performing the referenced role in the context of the containing object.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ResponsibleParty {
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[serde(
        serialize_with = "validation::ser_uuid_vec",
        deserialize_with = "validation::deser_uuid_vec"
    )]
    pub party_uuids: Vec<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
    /// A reference to a role performed by a party.
    #[serde(
        serialize_with = "validation::ser_token",
        deserialize_with = "validation::deser_token"
    )]
    pub role_id: String,
}

/// A physical point of presence, which may be associated with people, organizations, or
/// other concepts within the current or linked OSCAL document.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Location {
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_email_vec_opt",
        deserialize_with = "validation::deser_email_vec_opt"
    )]
    pub email_addresses: Option<Vec<String>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telephone_numbers: Option<Vec<TelephoneNumber>>,
    /// A name given to the location, which may be used by a tool for display and navigation.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub title: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_uri_vec_opt",
        deserialize_with = "validation::deser_uri_vec_opt"
    )]
    pub urls: Option<Vec<String>>,
    /// A unique ID for the location, for reference.
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    #[derivative(PartialEq = "ignore")]
    pub uuid: String,
}

/// A postal address for the location.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Address {
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addr_lines: Option<Vec<String>>,
    /// City, town or geographical region for the mailing address.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The ISO 3166-1 alpha-2 country code for the mailing address.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Postal or ZIP code for mailing address.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, province or analogous geographical region for a mailing address.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Indicates the type of address.
    #[serde(rename = "type")]
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_token_opt",
        deserialize_with = "validation::deser_token_opt"
    )]
    pub address_type: Option<String>,
}

/// A telephone service number as defined by ITU-T E.164.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct TelephoneNumber {
    pub number: String,
    /// Indicates the type of phone number.
    #[serde(rename = "type")]
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telephone_number_type: Option<String>,
}

/// An organization or person, which may be associated with roles or other concepts within
/// the current or linked OSCAL document.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Party {
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<Address>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_email_vec_opt",
        deserialize_with = "validation::deser_email_vec_opt"
    )]
    pub email_addresses: Option<Vec<String>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_ids: Option<Vec<PartyExternalIdentifier>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_uuids: Option<Vec<String>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_uuid_vec_opt",
        deserialize_with = "validation::deser_uuid_vec_opt"
    )]
    pub member_of_organizations: Option<Vec<String>>,
    /// The full name of the party. This is typically the legal name associated with the party.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
    /// A short common name, abbreviation, or acronym for the party.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telephone_numbers: Option<Vec<TelephoneNumber>>,
    /// A category describing the kind of party the object describes.
    #[serde(rename = "type")]
    pub party_type: PartyType,
    /// A unique identifier for the party.
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    #[derivative(PartialEq = "ignore")]
    pub uuid: String,
}

/// An identifier for a person or organization using a designated scheme. e.g. an Open
/// Researcher and Contributor ID (ORCID).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PartyExternalIdentifier {
    pub id: String,
    /// Indicates the type of external identifier.
    #[serde(
        serialize_with = "validation::ser_uri",
        deserialize_with = "validation::deser_uri"
    )]
    pub scheme: String,
}
/// A category describing the kind of party the object describes.
///
/// A label that indicates the nature of a resource, as a data serialization or format.
///
/// A non-empty string with leading and trailing whitespace disallowed. Whitespace is: U+9,
/// U+10, U+32 or [
/// ]+
///
/// In case where the href points to a back-matter/resource, this value will indicate the URI
/// fragment to append to any rlink associated with the resource. This value MUST be URI
/// encoded.
///
/// Indicates the value of the attribute, characteristic, or quality.
///
/// A formal (executable) expression of a constraint.
///
/// A parameter value or set of values.
///
/// A single line of an address.
///
/// City, town or geographical region for the mailing address.
///
/// The ISO 3166-1 alpha-2 country code for the mailing address.
///
/// Postal or ZIP code for mailing address.
///
/// State, province or analogous geographical region for a mailing address.
///
/// The OSCAL model version the document was authored against and will conform to as valid.
///
/// The full name of the party. This is typically the legal name associated with the party.
///
/// A short common name, abbreviation, or acronym for the party.
///
/// Used to distinguish a specific revision of an OSCAL document from other previous and
/// future versions.
///
/// A short common name, abbreviation, or acronym for the role.
///
/// The digest method by which a hash is derived.
///
/// Indicates the type of phone number.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
#[derive(EnumString, Derivative)]
#[derivative(PartialEq)]
pub enum PartyType {
    Organization,
    Person,
}

/// An entry in a sequential list of revisions to the containing document, expected to be in
/// reverse chronological order (i.e. latest first).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RevisionHistoryEntry {
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_dttz_opt",
        deserialize_with = "validation::deser_dttz_opt"
    )]
    pub last_modified: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oscal_version: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_dttz_opt",
        deserialize_with = "validation::deser_dttz_opt"
    )]
    pub published: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
    /// A name given to the document revision, which may be used by a tool for display and
    /// navigation.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub title: Option<String>,
    pub version: String,
}

/// Defines a function, which might be assigned to a party in a specific situation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Role {
    /// A summary of the role's purpose and associated responsibilities.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub description: Option<String>,
    /// A unique identifier for the role.
    #[serde(
        serialize_with = "validation::ser_token",
        deserialize_with = "validation::deser_token"
    )]
    pub id: String,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
    /// A short common name, abbreviation, or acronym for the role.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    /// A name given to the role, which may be used by a tool for display and navigation.
    pub title: String,
}
