use derivative::Derivative;
use strum::EnumString;
use derive_builder::Builder;
use serde::{Serialize, Deserialize};
use crate::validation;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SystemSecurityPlan {
    #[serde(rename = "$schema")]
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    pub system_security_plan: SystemSecurityPlanSsp,
}

/// A system security plan, such as those described in NIST SP 800-18.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SystemSecurityPlanSsp {
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub back_matter: Option<BackMatter>,
    pub control_implementation: ControlImplementation,
    pub import_profile: ImportProfile,
    pub metadata: DocumentMetadata,
    pub system_characteristics: SystemCharacteristics,
    pub system_implementation: SystemImplementation,
    /// A machine-oriented, globally unique identifier with cross-instance scope that can be used
    /// to reference this system security plan (SSP) elsewhere in this or other OSCAL instances.
    /// The locally defined UUID of the SSP can be used to reference the data item locally or
    /// globally (e.g., in an imported OSCAL instance).This UUID should be assigned per-subject,
    /// which means it should be consistently used to identify the same subject across revisions
    /// of the document.
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

/// Describes how the system satisfies a set of controls.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ControlImplementation {
    /// A statement describing important things to know about how this set of control
    /// satisfaction documentation is approached.
    #[serde(deserialize_with = "validation::deser_markup")]
    pub description: String,
    pub implemented_requirements: Vec<ControlBasedRequirement>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_parameters: Option<Vec<SetParameterValue>>,
}

/// Describes how the system satisfies the requirements of an individual control.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ControlBasedRequirement {
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub by_components: Option<Vec<ComponentControlImplementation>>,
    /// A reference to a control with a corresponding id value. When referencing an externally
    /// defined control, the Control Identifier Reference must be used in the context of the
    /// external / imported OSCAL instance (e.g., uri-reference).
    pub control_id: String,
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
    pub responsible_roles: Option<Vec<ResponsibleRole>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_parameters: Option<Vec<SetParameterValue>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statements: Option<Vec<SpecificControlStatement>>,
    /// A machine-oriented, globally unique identifier with cross-instance scope that can be used
    /// to reference this control requirement elsewhere in this or other OSCAL instances. The
    /// locally defined UUID of the control requirement can be used to reference the data item
    /// locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned
    /// per-subject, which means it should be consistently used to identify the same subject
    /// across revisions of the document.
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    #[derivative(PartialEq = "ignore")]
    pub uuid: String,
}

/// Defines how the referenced component implements a set of controls.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ComponentControlImplementation {
    /// A machine-oriented identifier reference to the component that is implemeting a given
    /// control.
    #[derivative(PartialEq = "ignore")]
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    pub component_uuid: String,
    /// An implementation statement that describes how a control or a control statement is
    /// implemented within the referenced system component.
    pub description: String,
    /// Identifies content intended for external consumption, such as with leveraged
    /// organizations.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub export: Option<Export>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub implementation_status: Option<ImplementationStatus>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inherited: Option<Vec<InheritedControlImplementation>>,
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
    pub responsible_roles: Option<Vec<ResponsibleRole>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub satisfied: Option<Vec<SatisfiedControlImplementationResponsibility>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_parameters: Option<Vec<SetParameterValue>>,
    /// A machine-oriented, globally unique identifier with cross-instance scope that can be used
    /// to reference this by-component entry elsewhere in this or other OSCAL instances. The
    /// locally defined UUID of the by-component entry can be used to reference the data item
    /// locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned
    /// per-subject, which means it should be consistently used to identify the same subject
    /// across revisions of the document.
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    #[derivative(PartialEq = "ignore")]
    pub uuid: String,
}

/// Identifies content intended for external consumption, such as with leveraged
/// organizations.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Export {
    /// An implementation statement that describes the aspects of the control or control
    /// statement implementation that can be available to another system leveraging this system.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub description: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provided: Option<Vec<ProvidedControlImplementation>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub responsibilities: Option<Vec<ControlImplementationResponsibility>>,
}

/// Describes a capability which may be inherited by a leveraging system.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ProvidedControlImplementation {
    /// An implementation statement that describes the aspects of the control or control
    /// statement implementation that can be provided to another system leveraging this system.
    #[serde(deserialize_with = "validation::deser_markup")]
    pub description: String,
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
    pub responsible_roles: Option<Vec<ResponsibleRole>>,
    /// A machine-oriented, globally unique identifier with cross-instance scope that can be used
    /// to reference this provided entry elsewhere in this or other OSCAL instances. The locally
    /// defined UUID of the provided entry can be used to reference the data item locally or
    /// globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject,
    /// which means it should be consistently used to identify the same subject across revisions
    /// of the document.
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    #[derivative(PartialEq = "ignore")]
    pub uuid: String,
}

/// A reference to a role with responsibility for performing a function relative to the
/// containing object, optionally associated with a set of persons and/or organizations that
/// perform that role.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ResponsibleRole {
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_uuid_vec_opt",
        deserialize_with = "validation::deser_uuid_vec_opt"
    )]
    pub party_uuids: Option<Vec<String>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
    /// A human-oriented identifier reference to a role performed.
    #[serde(
        serialize_with = "validation::ser_token",
        deserialize_with = "validation::deser_token"
    )]
    pub role_id: String,
}

/// Describes a control implementation responsibility imposed on a leveraging system.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ControlImplementationResponsibility {
    /// An implementation statement that describes the aspects of the control or control
    /// statement implementation that a leveraging system must implement to satisfy the control
    /// provided by a leveraged system.
    pub description: String,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
    /// A machine-oriented identifier reference to an inherited control implementation that a
    /// leveraging system is inheriting from a leveraged system.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_uuid_opt",
        deserialize_with = "validation::deser_uuid_opt"
    )]
    pub provided_uuid: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub responsible_roles: Option<Vec<ResponsibleRole>>,
    /// A machine-oriented, globally unique identifier with cross-instance scope that can be used
    /// to reference this responsibility elsewhere in this or other OSCAL instances. The locally
    /// defined UUID of the responsibility can be used to reference the data item locally or
    /// globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject,
    /// which means it should be consistently used to identify the same subject across revisions
    /// of the document.
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    #[derivative(PartialEq = "ignore")]
    pub uuid: String,
}

/// Indicates the degree to which the a given control is implemented.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ImplementationStatus {
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
    /// Identifies the implementation status of the control or control objective.
    pub state: String,
}

/// Describes a control implementation inherited by a leveraging system.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct InheritedControlImplementation {
    /// An implementation statement that describes the aspects of a control or control statement
    /// implementation that a leveraging system is inheriting from a leveraged system.
    pub description: String,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
    /// A machine-oriented identifier reference to an inherited control implementation that a
    /// leveraging system is inheriting from a leveraged system.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_uuid_opt",
        deserialize_with = "validation::deser_uuid_opt"
    )]
    pub provided_uuid: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub responsible_roles: Option<Vec<ResponsibleRole>>,
    /// A machine-oriented, globally unique identifier with cross-instance scope that can be used
    /// to reference this inherited entry elsewhere in this or other OSCAL instances. The locally
    /// defined UUID of the inherited control implementation can be used to reference the data
    /// item locally or globally (e.g., in an imported OSCAL instance). This UUID should be
    /// assigned per-subject, which means it should be consistently used to identify the same
    /// subject across revisions of the document.
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    #[derivative(PartialEq = "ignore")]
    pub uuid: String,
}

/// Describes how this system satisfies a responsibility imposed by a leveraged system.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SatisfiedControlImplementationResponsibility {
    /// An implementation statement that describes the aspects of a control or control statement
    /// implementation that a leveraging system is implementing based on a requirement from a
    /// leveraged system.
    pub description: String,
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
    /// A machine-oriented identifier reference to a control implementation that satisfies a
    /// responsibility imposed by a leveraged system.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_uuid_opt",
        deserialize_with = "validation::deser_uuid_opt"
    )]
    pub responsibility_uuid: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub responsible_roles: Option<Vec<ResponsibleRole>>,
    /// A machine-oriented, globally unique identifier with cross-instance scope that can be used
    /// to reference this satisfied control implementation entry elsewhere in this or other OSCAL
    /// instances. The locally defined UUID of the control implementation can be used to
    /// reference the data item locally or globally (e.g., in an imported OSCAL instance). This
    /// UUID should be assigned per-subject, which means it should be consistently used to
    /// identify the same subject across revisions of the document.
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    #[derivative(PartialEq = "ignore")]
    pub uuid: String,
}

/// Identifies the parameter that will be set by the enclosed value.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SetParameterValue {
    /// A human-oriented reference to a parameter within a control, who's catalog has been
    /// imported into the current implementation context.
    #[serde(
        serialize_with = "validation::ser_token",
        deserialize_with = "validation::deser_token"
    )]
    pub param_id: String,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
    pub values: Vec<String>,
}

/// Identifies which statements within a control are addressed.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SpecificControlStatement {
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub by_components: Option<Vec<ComponentControlImplementation>>,
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
    pub responsible_roles: Option<Vec<ResponsibleRole>>,
    /// A human-oriented identifier reference to a control statement.
    #[serde(
        serialize_with = "validation::ser_token",
        deserialize_with = "validation::deser_token"
    )]
    pub statement_id: String,
    /// A machine-oriented, globally unique identifier with cross-instance scope that can be used
    /// to reference this control statement elsewhere in this or other OSCAL instances. The UUID
    /// of the control statement in the source OSCAL instance is sufficient to reference the data
    /// item locally or globally (e.g., in an imported OSCAL instance).
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    #[derivative(PartialEq = "ignore")]
    pub uuid: String,
}

/// Used to import the OSCAL profile representing the system's control baseline.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ImportProfile {
    /// A resolvable URL reference to the profile or catalog to use as the system's control
    /// baseline.
    #[serde(
        serialize_with = "validation::ser_uri_ref",
        deserialize_with = "validation::deser_uri_ref"
    )]
    pub href: String,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
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
/// A target-level of availability for the system, based on the sensitivity of information
/// within the system.
///
/// A target-level of confidentiality for the system, based on the sensitivity of information
/// within the system.
///
/// A target-level of integrity for the system, based on the sensitivity of information
/// within the system.
///
/// The overall information system sensitivity categorization, such as defined by FIPS-199.
///
/// The prescribed base (Confidentiality, Integrity, or Availability) security impact level.
///
/// The selected (Confidentiality, Integrity, or Availability) security impact level.
///
/// A human-oriented, globally unique identifier qualified by the given identification system
/// used, such as NIST SP 800-60. This identifier has cross-instance scope and can be used to
/// reference this system elsewhere in this or other OSCAL instances. This id should be
/// assigned per-subject, which means it should be consistently used to identify the same
/// subject across revisions of the document.
///
/// The full name of the system.
///
/// A short name for the system, such as an acronym, that is suitable for display in a data
/// table or summary list.
///
/// The common name of the protocol, which should be the appropriate "service name" from the
/// IANA Service Name and Transport Protocol Port Number Registry.
///
/// Describes a function performed for a given authorized privilege by this user class.
///
/// A short common name, abbreviation, or acronym for the user.
///
/// The digest method by which a hash is derived.
///
/// Indicates the type of phone number.
///
/// A category describing the purpose of the component.
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

/// Contains the characteristics of the system, such as its name, purpose, and security
/// impact level.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SystemCharacteristics {
    pub authorization_boundary: AuthorizationBoundary,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_flow: Option<DataFlow>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_date_opt",
        deserialize_with = "validation::deser_date_opt"
    )]
    pub date_authorized: Option<String>,
    /// A summary of the system.
    #[serde(deserialize_with = "validation::deser_markup")]
    pub description: String,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_architecture: Option<NetworkArchitecture>,
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
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_impact_level: Option<SecurityImpactLevel>,
    /// The overall information system sensitivity categorization, such as defined by FIPS-199.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_sensitivity_level: Option<String>,
    pub status: SystemCharacteristicsStatus,
    pub system_ids: Vec<SystemIdentification>,
    pub system_information: SystemInformation,
    /// The full name of the system.
    pub system_name: String,
    /// A short name for the system, such as an acronym, that is suitable for display in a data
    /// table or summary list.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_name_short: Option<String>,
}

/// A description of this system's authorization boundary, optionally supplemented by
/// diagrams that illustrate the authorization boundary.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AuthorizationBoundary {
    /// A summary of the system's authorization boundary.
    pub description: String,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagrams: Option<Vec<Diagram>>,
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
}

/// A graphic that provides a visual representation the system, or some aspect of it.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Diagram {
    /// A brief caption to annotate the diagram.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub caption: Option<String>,
    /// A summary of the diagram.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
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
    /// A machine-oriented, globally unique identifier with cross-instance scope that can be used
    /// to reference this diagram elsewhere in this or other OSCAL instances. The locally defined
    /// UUID of the diagram can be used to reference the data item locally or globally (e.g., in
    /// an imported OSCAL instance). This UUID should be assigned per-subject, which means it
    /// should be consistently used to identify the same subject across revisions of the document.
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    #[derivative(PartialEq = "ignore")]
    pub uuid: String,
}

/// A description of the logical flow of information within the system and across its
/// boundaries, optionally supplemented by diagrams that illustrate these flows.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DataFlow {
    /// A summary of the system's data flow.
    pub description: String,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagrams: Option<Vec<Diagram>>,
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
}

/// A description of the system's network architecture, optionally supplemented by diagrams
/// that illustrate the network architecture.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct NetworkArchitecture {
    /// A summary of the system's network architecture.
    pub description: String,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagrams: Option<Vec<Diagram>>,
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
}

/// The overall level of expected impact resulting from unauthorized disclosure,
/// modification, or loss of access to information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SecurityImpactLevel {
    /// A target-level of availability for the system, based on the sensitivity of information
    /// within the system.
    pub security_objective_availability: String,
    /// A target-level of confidentiality for the system, based on the sensitivity of information
    /// within the system.
    pub security_objective_confidentiality: String,
    /// A target-level of integrity for the system, based on the sensitivity of information
    /// within the system.
    pub security_objective_integrity: String,
}

/// Describes the operational status of the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SystemCharacteristicsStatus {
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
    /// The current operating status.
    pub state: PurpleState,
}
/// The current operating status.
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
/// A target-level of availability for the system, based on the sensitivity of information
/// within the system.
///
/// A target-level of confidentiality for the system, based on the sensitivity of information
/// within the system.
///
/// A target-level of integrity for the system, based on the sensitivity of information
/// within the system.
///
/// The overall information system sensitivity categorization, such as defined by FIPS-199.
///
/// The prescribed base (Confidentiality, Integrity, or Availability) security impact level.
///
/// The selected (Confidentiality, Integrity, or Availability) security impact level.
///
/// A human-oriented, globally unique identifier qualified by the given identification system
/// used, such as NIST SP 800-60. This identifier has cross-instance scope and can be used to
/// reference this system elsewhere in this or other OSCAL instances. This id should be
/// assigned per-subject, which means it should be consistently used to identify the same
/// subject across revisions of the document.
///
/// The full name of the system.
///
/// A short name for the system, such as an acronym, that is suitable for display in a data
/// table or summary list.
///
/// The common name of the protocol, which should be the appropriate "service name" from the
/// IANA Service Name and Transport Protocol Port Number Registry.
///
/// Describes a function performed for a given authorized privilege by this user class.
///
/// A short common name, abbreviation, or acronym for the user.
///
/// The digest method by which a hash is derived.
///
/// Indicates the type of phone number.
///
/// A category describing the purpose of the component.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
#[derive(EnumString, Derivative)]
#[derivative(PartialEq)]
pub enum PurpleState {
    Disposition,
    Operational,
    Other,
    #[serde(rename = "under-development")]
    UnderDevelopment,
    #[serde(rename = "under-major-modification")]
    UnderMajorModification,
}

/// A human-oriented, globally unique identifier with cross-instance scope that can be used
/// to reference this system identification property elsewhere in this or other OSCAL
/// instances. When referencing an externally defined system identification, the system
/// identification must be used in the context of the external / imported OSCAL instance
/// (e.g., uri-reference). This string should be assigned per-subject, which means it should
/// be consistently used to identify the same system across revisions of the document.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SystemIdentification {
    pub id: String,
    /// Identifies the identification system from which the provided identifier was assigned.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_uri_opt",
        deserialize_with = "validation::deser_uri_opt"
    )]
    pub identifier_type: Option<String>,
}

/// Contains details about all information types that are stored, processed, or transmitted
/// by the system, such as privacy information, and those defined in NIST SP 800-60.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SystemInformation {
    pub information_types: Vec<InformationType>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
}

/// Contains details about one information type that is stored, processed, or transmitted by
/// the system, such as privacy information, and those defined in NIST SP 800-60.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct InformationType {
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub availability_impact: Option<ImpactLevel>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub categorizations: Option<Vec<InformationTypeCategorization>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidentiality_impact: Option<ImpactLevel>,
    /// A summary of how this information type is used within the system.
    pub description: String,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integrity_impact: Option<ImpactLevel>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
    /// A human readable name for the information type. This title should be meaningful within
    /// the context of the system.
    pub title: String,
    /// A machine-oriented, globally unique identifier with cross-instance scope that can be used
    /// to reference this information type elsewhere in this or other OSCAL instances. The
    /// locally defined UUID of the information type can be used to reference the data item
    /// locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned
    /// per-subject, which means it should be consistently used to identify the same subject
    /// across revisions of the document.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_uuid_opt",
        deserialize_with = "validation::deser_uuid_opt"
    )]
    #[derivative(PartialEq = "ignore")]
    pub uuid: Option<String>,
}

/// The expected level of impact resulting from the described information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ImpactLevel {
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub adjustment_justification: Option<String>,
    pub base: String,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected: Option<String>,
}

/// A set of information type identifiers qualified by the given identification system used,
/// such as NIST SP 800-60.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct InformationTypeCategorization {
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub information_type_ids: Option<Vec<String>>,
    /// Specifies the information type identification system used.
    pub system: String,
}

/// Provides information as to how the system is implemented.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SystemImplementation {
    pub components: Vec<Component>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inventory_items: Option<Vec<InventoryItem>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leveraged_authorizations: Option<Vec<LeveragedAuthorization>>,
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
    pub users: Vec<SystemUser>,
}

/// A defined component that can be part of an implemented system.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Component {
    /// A description of the component, including information about its function.
    #[serde(deserialize_with = "validation::deser_markup")]
    pub description: String,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<ServiceProtocolInformation>>,
    /// A summary of the technological or business purpose of the component.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub purpose: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub responsible_roles: Option<Vec<ResponsibleRole>>,
    /// Describes the operational status of the system component.
    pub status: ComponentStatus,
    /// A human readable name for the system component.
    #[serde(deserialize_with = "validation::deser_markup")]
    pub title: String,
    /// A category describing the purpose of the component.
    #[serde(rename = "type")]
    #[serde(
        serialize_with = "validation::ser_token",
        deserialize_with = "validation::deser_token"
    )]
    pub component_type: String,
    /// A machine-oriented, globally unique identifier with cross-instance scope that can be used
    /// to reference this component elsewhere in this or other OSCAL instances. The locally
    /// defined UUID of the component can be used to reference the data item locally or globally
    /// (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject, which
    /// means it should be consistently used to identify the same subject across revisions of the
    /// document.
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    #[derivative(PartialEq = "ignore")]
    pub uuid: String,
}

/// Information about the protocol used to provide a service.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ServiceProtocolInformation {
    /// The common name of the protocol, which should be the appropriate "service name" from the
    /// IANA Service Name and Transport Protocol Port Number Registry.
    pub name: String,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port_ranges: Option<Vec<PortRange>>,
    /// A human readable name for the protocol (e.g., Transport Layer Security).
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub title: Option<String>,
    /// A machine-oriented, globally unique identifier with cross-instance scope that can be used
    /// to reference this service protocol information elsewhere in this or other OSCAL
    /// instances. The locally defined UUID of the service protocol can be used to reference the
    /// data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be
    /// assigned per-subject, which means it should be consistently used to identify the same
    /// subject across revisions of the document.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_uuid_opt",
        deserialize_with = "validation::deser_uuid_opt"
    )]
    #[derivative(PartialEq = "ignore")]
    pub uuid: Option<String>,
}

/// Where applicable this is the IPv4 port range on which the service operates.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PortRange {
    /// Indicates the ending port number in a port range
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_non_neg_int_opt",
        deserialize_with = "validation::deser_non_neg_int_opt"
    )]
    pub end: Option<i64>,
    /// Indicates the starting port number in a port range
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_non_neg_int_opt",
        deserialize_with = "validation::deser_non_neg_int_opt"
    )]
    pub start: Option<i64>,
    /// Indicates the transport type.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transport: Option<Transport>,
}
/// Indicates the transport type.
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
/// A human-oriented identifier reference to a role performed.
///
/// A human-oriented reference to a parameter within a control, who's catalog has been
/// imported into the current implementation context.
///
/// A reference to a control with a corresponding id value. When referencing an externally
/// defined control, the Control Identifier Reference must be used in the context of the
/// external / imported OSCAL instance (e.g., uri-reference).
///
/// A human-oriented identifier reference to a control statement.
///
/// A reference to a role performed by a party.
///
/// The type of action documented by the assembly, such as an approval.
///
/// A unique identifier for the role.
///
/// Reference to a role by UUID.
///
/// Describes the type of relationship provided by the link's hypertext reference. This can
/// be an indicator of the link's purpose.
///
/// Identifies the implementation status of the control or control objective.
///
/// Indicates the type of address.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
#[derive(EnumString, Derivative)]
#[derivative(PartialEq)]
pub enum Transport {
    #[serde(rename = "TCP")]
    Tcp,
    #[serde(rename = "UDP")]
    Udp,
}

/// Describes the operational status of the system component.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ComponentStatus {
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
    /// The operational status.
    pub state: FluffyState,
}
/// The operational status.
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
/// A human-oriented identifier reference to a role performed.
///
/// A human-oriented reference to a parameter within a control, who's catalog has been
/// imported into the current implementation context.
///
/// A reference to a control with a corresponding id value. When referencing an externally
/// defined control, the Control Identifier Reference must be used in the context of the
/// external / imported OSCAL instance (e.g., uri-reference).
///
/// A human-oriented identifier reference to a control statement.
///
/// A reference to a role performed by a party.
///
/// The type of action documented by the assembly, such as an approval.
///
/// A unique identifier for the role.
///
/// Reference to a role by UUID.
///
/// Describes the type of relationship provided by the link's hypertext reference. This can
/// be an indicator of the link's purpose.
///
/// Identifies the implementation status of the control or control objective.
///
/// Indicates the type of address.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
#[derive(EnumString, Derivative)]
#[derivative(PartialEq)]
pub enum FluffyState {
    Disposition,
    Operational,
    Other,
    #[serde(rename = "under-development")]
    UnderDevelopment,
}

/// A single managed inventory item within the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct InventoryItem {
    /// A summary of the inventory item stating its purpose within the system.
    #[serde(deserialize_with = "validation::deser_markup")]
    pub description: String,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub implemented_components: Option<Vec<ImplementedComponent>>,
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
    /// A machine-oriented, globally unique identifier with cross-instance scope that can be used
    /// to reference this inventory item elsewhere in this or other OSCAL instances. The locally
    /// defined UUID of the inventory item can be used to reference the data item locally or
    /// globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject,
    /// which means it should be consistently used to identify the same subject across revisions
    /// of the document.
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    #[derivative(PartialEq = "ignore")]
    pub uuid: String,
}

/// The set of components that are implemented in a given system inventory item.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ImplementedComponent {
    /// A machine-oriented identifier reference to a component that is implemented as part of an
    /// inventory item.
    #[derivative(PartialEq = "ignore")]
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    pub component_uuid: String,
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
}

/// A description of another authorized system from which this system inherits capabilities
/// that satisfy security requirements. Another term for this concept is a common control
/// provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LeveragedAuthorization {
    #[serde(
        serialize_with = "validation::ser_date",
        deserialize_with = "validation::deser_date"
    )]
    pub date_authorized: String,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    /// A machine-oriented identifier reference to the party that manages the leveraged system.
    #[derivative(PartialEq = "ignore")]
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    pub party_uuid: String,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
    /// A human readable name for the leveraged authorization in the context of the system.
    #[serde(deserialize_with = "validation::deser_markup")]
    pub title: String,
    /// A machine-oriented, globally unique identifier with cross-instance scope and can be used
    /// to reference this leveraged authorization elsewhere in this or other OSCAL instances. The
    /// locally defined UUID of the leveraged authorization can be used to reference the data
    /// item locally or globally (e.g., in an imported OSCAL instance). This UUID should be
    /// assigned per-subject, which means it should be consistently used to identify the same
    /// subject across revisions of the document.
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    #[derivative(PartialEq = "ignore")]
    pub uuid: String,
}

/// A type of user that interacts with the system based on an associated role.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SystemUser {
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorized_privileges: Option<Vec<Privilege>>,
    /// A summary of the user's purpose within the system.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub description: Option<String>,
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
    #[serde(deserialize_with = "validation::deser_token_vec_opt")]
    pub role_ids: Option<Vec<String>>,
    /// A short common name, abbreviation, or acronym for the user.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    /// A name given to the user, which may be used by a tool for display and navigation.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub title: Option<String>,
    /// A machine-oriented, globally unique identifier with cross-instance scope that can be used
    /// to reference this user class elsewhere in this or other OSCAL instances. The locally
    /// defined UUID of the system user can be used to reference the data item locally or
    /// globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject,
    /// which means it should be consistently used to identify the same subject across revisions
    /// of the document.
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    #[derivative(PartialEq = "ignore")]
    pub uuid: String,
}

/// Identifies a specific system privilege held by the user, along with an associated
/// description and/or rationale for the privilege.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Privilege {
    /// A summary of the privilege's purpose within the system.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub description: Option<String>,
    pub functions_performed: Vec<String>,
    /// A human readable name for the privilege.
    pub title: String,
}
