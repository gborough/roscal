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
pub struct AssessmentResults {
    #[serde(rename = "$schema")]
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    pub assessment_results: SecurityAssessmentResultsSar,
}

/// Security assessment results, such as those provided by a FedRAMP assessor in the FedRAMP
/// Security Assessment Report.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SecurityAssessmentResultsSar {
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub back_matter: Option<BackMatter>,
    pub import_ap: ImportAssessmentPlan,
    /// Used to define data objects that are used in the assessment plan, that do not appear in
    /// the referenced SSP.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_definitions: Option<AssessmentResultsLocalDefinitions>,
    pub metadata: DocumentMetadata,
    pub results: Vec<AssessmentResult>,
    /// A machine-oriented, globally unique identifier with cross-instance scope that can be used
    /// to reference this assessment results instance in this or other OSCAL instances. The
    /// locally defined UUID of the assessment result can be used to reference the data item
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

/// Used by assessment-results to import information about the original plan for assessing
/// the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ImportAssessmentPlan {
    /// A resolvable URL reference to the assessment plan governing the assessment activities.
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

/// Used to define data objects that are used in the assessment plan, that do not appear in
/// the referenced SSP.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AssessmentResultsLocalDefinitions {
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activities: Option<Vec<Activity>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub objectives_and_methods: Option<Vec<AssessmentSpecificControlObjective>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
}

/// Identifies an assessment or related process that can be performed. In the assessment
/// plan, this is an intended activity which may be associated with an assessment task. In
/// the assessment results, this an activity that was actually performed as part of an
/// assessment.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Activity {
    /// A human-readable description of this included activity.
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
    pub related_controls: Option<ReviewedControlsAndControlObjectives>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub responsible_roles: Option<Vec<ResponsibleRole>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<Step>>,
    /// The title for this included activity.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub title: Option<String>,
    /// A machine-oriented, globally unique identifier with cross-instance scope that can be used
    /// to reference this assessment activity elsewhere in this or other OSCAL instances. The
    /// locally defined UUID of the activity can be used to reference the data item locally or
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

/// Identifies the controls being assessed and their control objectives.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ReviewedControlsAndControlObjectives {
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub control_objective_selections: Option<Vec<ReferencedControlObjectives>>,
    pub control_selections: Vec<AssessedControls>,
    /// A human-readable description of control objectives.
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
}

/// Identifies the control objectives of the assessment. In the assessment plan, these are
/// the planned objectives. In the assessment results, these are the assessed objectives, and
/// reflects any changes from the plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ReferencedControlObjectives {
    /// A human-readable description of this collection of control objectives.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub description: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude_objectives: Option<Vec<SelectObjective>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_all: Option<IncludeAll>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_objectives: Option<Vec<SelectObjective>>,
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

/// Used to select a control objective for inclusion/exclusion based on the control
/// objective's identifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SelectObjective {
    /// Points to an assessment objective.
    #[serde(
        serialize_with = "validation::ser_token",
        deserialize_with = "validation::deser_token"
    )]
    pub objective_id: String,
}

/// Include all controls from the imported catalog or profile resources.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct IncludeAll {}

/// Identifies the controls being assessed. In the assessment plan, these are the planned
/// controls. In the assessment results, these are the actual controls, and reflects any
/// changes from the plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AssessedControls {
    /// A human-readable description of in-scope controls specified for assessment.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub description: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude_controls: Option<Vec<SelectControl>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_all: Option<IncludeAll>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_controls: Option<Vec<SelectControl>>,
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

/// Used to select a control for inclusion/exclusion based on one or more control
/// identifiers. A set of statement identifiers can be used to target the inclusion/exclusion
/// to only specific control statements providing more granularity over the specific
/// statements that are within the asessment scope.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SelectControl {
    /// A reference to a control with a corresponding id value. When referencing an externally
    /// defined control, the Control Identifier Reference must be used in the context of the
    /// external / imported OSCAL instance (e.g., uri-reference).
    #[serde(
        serialize_with = "validation::ser_token",
        deserialize_with = "validation::deser_token"
    )]
    pub control_id: String,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_token_vec_opt",
        deserialize_with = "validation::deser_token_vec_opt"
    )]
    pub statement_ids: Option<Vec<String>>,
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

/// Identifies an individual step in a series of steps related to an activity, such as an
/// assessment test or examination procedure.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Step {
    /// A human-readable description of this step.
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
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewed_controls: Option<ReviewedControlsAndControlObjectives>,
    /// The title for this step.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub title: Option<String>,
    /// A machine-oriented, globally unique identifier with cross-instance scope that can be used
    /// to reference this step elsewhere in this or other OSCAL instances. The locally defined
    /// UUID of the step (in a series of steps) can be used to reference the data item locally or
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

/// A local definition of a control objective for this assessment. Uses catalog syntax for
/// control objective and assessment actions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AssessmentSpecificControlObjective {
    /// A reference to a control with a corresponding id value. When referencing an externally
    /// defined control, the Control Identifier Reference must be used in the context of the
    /// external / imported OSCAL instance (e.g., uri-reference).
    pub control_id: String,
    /// A human-readable description of this control objective.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    pub parts: Vec<Part>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
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
/// The common name of the protocol, which should be the appropriate "service name" from the
/// IANA Service Name and Transport Protocol Port Number Registry.
///
/// Describes a function performed for a given authorized privilege by this user class.
///
/// A short common name, abbreviation, or acronym for the user.
///
/// Indicates the value of the facet.
///
/// The digest method by which a hash is derived.
///
/// Indicates the type of phone number.
///
/// A category describing the purpose of the component.
///
/// Identifies how the observation was made.
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

/// Used by the assessment results and POA&M. In the assessment results, this identifies all
/// of the assessment observations and findings, initial and residual risks, deviations, and
/// disposition. In the POA&M, this identifies initial and residual risks, deviations, and
/// disposition.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AssessmentResult {
    /// A log of all assessment-related actions taken.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assessment_log: Option<AssessmentLog>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attestations: Option<Vec<AttestationStatements>>,
    /// A human-readable description of this set of test results.
    #[serde(deserialize_with = "validation::deser_markup")]
    pub description: String,
    /// Date/time stamp identifying the end of the evidence collection reflected in these
    /// results. In a continuous motoring scenario, this may contain the same value as start if
    /// appropriate.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_dttz_opt",
        deserialize_with = "validation::deser_dttz_opt"
    )]
    pub end: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub findings: Option<Vec<Finding>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    /// Used to define data objects that are used in the assessment plan, that do not appear in
    /// the referenced SSP.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_definitions: Option<ResultLocalDefinitions>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observations: Option<Vec<Observation>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
    pub reviewed_controls: ReviewedControlsAndControlObjectives,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub risks: Option<Vec<IdentifiedRisk>>,
    /// Date/time stamp identifying the start of the evidence collection reflected in these
    /// results.
    #[serde(
        serialize_with = "validation::ser_dttz",
        deserialize_with = "validation::deser_dttz"
    )]
    pub start: String,
    /// The title for this set of results.
    #[serde(deserialize_with = "validation::deser_markup")]
    pub title: String,
    /// A machine-oriented, globally unique identifier with cross-instance scope that can be used
    /// to reference this set of results in this or other OSCAL instances. The locally defined
    /// UUID of the assessment result can be used to reference the data item locally or globally
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

/// A log of all assessment-related actions taken.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AssessmentLog {
    pub entries: Vec<AssessmentLogEntry>,
}

/// Identifies the result of an action and/or task that occurred as part of executing an
/// assessment plan or an assessment event that occurred in producing the assessment results.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AssessmentLogEntry {
    /// A human-readable description of this event.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub description: Option<String>,
    /// Identifies the end date and time of an event. If the event is a point in time, the start
    /// and end will be the same date and time.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_dttz_opt",
        deserialize_with = "validation::deser_dttz_opt"
    )]
    pub end: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logged_by: Option<Vec<LoggedBy>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_tasks: Option<Vec<TaskReference>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
    /// Identifies the start date and time of an event.
    #[serde(
        serialize_with = "validation::ser_dttz",
        deserialize_with = "validation::deser_dttz"
    )]
    pub start: String,
    /// The title for this event.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub title: Option<String>,
    /// A machine-oriented, globally unique identifier with cross-instance scope that can be used
    /// to reference an assessment event in this or other OSCAL instances. The locally defined
    /// UUID of the assessment log entry can be used to reference the data item locally or
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

/// Used to indicate who created a log entry in what role.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LoggedBy {
    /// A machine-oriented identifier reference to the party who is making the log entry.
    #[derivative(PartialEq = "ignore")]
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    pub party_uuid: String,
    /// A point to the role-id of the role in which the party is making the log entry.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_token_opt",
        deserialize_with = "validation::deser_token_opt"
    )]
    pub role_id: Option<String>,
}

/// Identifies an individual task for which the containing object is a consequence of.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct TaskReference {
    /// Used to detail assessment subjects that were identfied by this task.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identified_subject: Option<IdentifiedSubject>,
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
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subjects: Option<Vec<SubjectOfAssessment>>,
    /// A machine-oriented identifier reference to a unique task.
    #[derivative(PartialEq = "ignore")]
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    pub task_uuid: String,
}

/// Used to detail assessment subjects that were identfied by this task.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct IdentifiedSubject {
    /// A machine-oriented identifier reference to a unique assessment subject placeholder
    /// defined by this task.
    #[derivative(PartialEq = "ignore")]
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    pub subject_placeholder_uuid: String,
    pub subjects: Vec<SubjectOfAssessment>,
}

/// Identifies system elements being assessed, such as components, inventory items, and
/// locations. In the assessment plan, this identifies a planned assessment subject. In the
/// assessment results this is an actual assessment subject, and reflects any changes from
/// the plan. exactly what will be the focus of this assessment. Any subjects not identified
/// in this way are out-of-scope.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SubjectOfAssessment {
    /// A human-readable description of the collection of subjects being included in this
    /// assessment.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub description: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude_subjects: Option<Vec<SelectAssessmentSubject>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_all: Option<IncludeAll>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_subjects: Option<Vec<SelectAssessmentSubject>>,
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
    /// Indicates the type of assessment subject, such as a component, inventory, item, location,
    /// or party represented by this selection statement.
    #[serde(rename = "type")]
    #[serde(
        serialize_with = "validation::ser_token",
        deserialize_with = "validation::deser_token"
    )]
    pub subject_of_assessment_type: String,
}

/// Identifies a set of assessment subjects to include/exclude by UUID.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SelectAssessmentSubject {
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
    /// A machine-oriented identifier reference to a component, inventory-item, location, party,
    /// user, or resource using it's UUID.
    #[derivative(PartialEq = "ignore")]
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    pub subject_uuid: String,
    /// Used to indicate the type of object pointed to by the uuid-ref within a subject.
    #[serde(rename = "type")]
    #[serde(
        serialize_with = "validation::ser_token",
        deserialize_with = "validation::deser_token"
    )]
    pub select_assessment_subject_type: String,
}

/// A set of textual statements, typically written by the assessor.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AttestationStatements {
    pub parts: Vec<AssessmentPart>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub responsible_parties: Option<Vec<ResponsibleParty>>,
}

/// A partition of an assessment plan or results or a child of another part.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AssessmentPart {
    /// A textual label that provides a sub-type or characterization of the part's name. This can
    /// be used to further distinguish or discriminate between the semantics of multiple parts of
    /// the same control with the same name and ns.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_token_opt",
        deserialize_with = "validation::deser_token_opt"
    )]
    pub class: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    /// A textual label that uniquely identifies the part's semantic type.
    #[serde(
        serialize_with = "validation::ser_token",
        deserialize_with = "validation::deser_token"
    )]
    pub name: String,
    /// A namespace qualifying the part's name. This allows different organizations to associate
    /// distinct semantics with the same name.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_uri_opt",
        deserialize_with = "validation::deser_uri_opt"
    )]
    pub ns: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<AssessmentPart>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
    /// Permits multiple paragraphs, lists, tables etc.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prose: Option<String>,
    /// A name given to the part, which may be used by a tool for display and navigation.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub title: Option<String>,
    /// A machine-oriented, globally unique identifier with cross-instance scope that can be used
    /// to reference this part elsewhere in this or other OSCAL instances. The locally defined
    /// UUID of the part can be used to reference the data item locally or globally (e.g., in an
    /// ported OSCAL instance). This UUID should be assigned per-subject, which means it should
    /// be consistently used to identify the same subject across revisions of the document.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_uuid_opt",
        deserialize_with = "validation::deser_uuid_opt"
    )]
    #[derivative(PartialEq = "ignore")]
    pub uuid: Option<String>,
}

/// Describes an individual finding.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Finding {
    /// A human-readable description of this finding.
    #[serde(deserialize_with = "validation::deser_markup")]
    pub description: String,
    /// A machine-oriented identifier reference to the implementation statement in the SSP to
    /// which this finding is related.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_uuid_opt",
        deserialize_with = "validation::deser_uuid_opt"
    )]
    pub implementation_statement_uuid: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origins: Option<Vec<Origin>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_observations: Option<Vec<FindingRelatedObservation>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_risks: Option<Vec<AssociatedRisk>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
    pub target: TargetClass,
    /// The title for this finding.
    #[serde(deserialize_with = "validation::deser_markup")]
    pub title: String,
    /// A machine-oriented, globally unique identifier with cross-instance scope that can be used
    /// to reference this finding in this or other OSCAL instances. The locally defined UUID of
    /// the finding can be used to reference the data item locally or globally (e.g., in an
    /// imported OSCAL instance). This UUID should be assigned per-subject, which means it should
    /// be consistently used to identify the same subject across revisions of the document.
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    #[derivative(PartialEq = "ignore")]
    pub uuid: String,
}

/// Identifies the source of the finding, such as a tool, interviewed person, or activity.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Origin {
    pub actors: Vec<OriginatingActor>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_tasks: Option<Vec<TaskReference>>,
}

/// The actor that produces an observation, a finding, or a risk. One or more actor type can
/// be used to specify a person that is using a tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct OriginatingActor {
    /// A machine-oriented identifier reference to the tool or person based on the associated
    /// type.
    #[derivative(PartialEq = "ignore")]
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    pub actor_uuid: String,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
    /// For a party, this can optionally be used to specify the role the actor was performing.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_token_opt",
        deserialize_with = "validation::deser_token_opt"
    )]
    pub role_id: Option<String>,
    /// The kind of actor.
    #[serde(rename = "type")]
    pub originating_actor_type: ActorType,
}
/// The kind of actor.
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
/// Points to an assessment objective.
///
/// A reference to a control with a corresponding id value. When referencing an externally
/// defined control, the Control Identifier Reference must be used in the context of the
/// external / imported OSCAL instance (e.g., uri-reference).
///
/// Used to constrain the selection to only specificity identified statements.
///
/// A human-oriented identifier reference to a role performed.
///
/// An optional textual providing a sub-type or characterization of the part's name, or a
/// category to which the part belongs.
///
/// A unique identifier for the part.
///
/// A textual label that uniquely identifies the part's semantic type, which exists in a
/// value space qualified by the ns.
///
/// A reference to a role performed by a party.
///
/// The type of action documented by the assembly, such as an approval.
///
/// A unique identifier for the role.
///
/// A point to the role-id of the role in which the party is making the log entry.
///
/// A textual label that provides a sub-type or characterization of the part's name. This can
/// be used to further distinguish or discriminate between the semantics of multiple parts of
/// the same control with the same name and ns.
///
/// For a party, this can optionally be used to specify the role the actor was performing.
///
/// A machine-oriented identifier reference for a specific target qualified by the type.
///
/// Reference to a role by UUID.
///
/// The name of the risk metric within the specified system.
///
/// Describes the type of relationship provided by the link's hypertext reference. This can
/// be an indicator of the link's purpose.
///
/// Indicates the type of address.
///
/// Used to indicate the type of object pointed to by the uuid-ref within a subject.
///
/// Indicates the type of assessment subject, such as a component, inventory, item, location,
/// or party represented by this selection statement.
///
/// A textual label that uniquely identifies the part's semantic type.
///
/// Identifies the implementation status of the control or control objective.
///
/// The reason the objective was given it's status.
///
/// The type of task.
///
/// Identifies the nature of the observation. More than one may be used to further qualify
/// and enable filtering.
///
/// Identifies whether this is a recommendation, such as from an assessor or tool, or an
/// actual plan accepted by the system owner.
///
/// Describes the status of the associated risk.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
#[derive(EnumString, Derivative)]
#[derivative(PartialEq)]
pub enum ActorType {
    #[serde(rename = "assessment-platform")]
    AssessmentPlatform,
    Party,
    Tool,
}

/// Relates the finding to a set of referenced observations that were used to determine the
/// finding.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct FindingRelatedObservation {
    /// A machine-oriented identifier reference to an observation defined in the list of
    /// observations.
    #[derivative(PartialEq = "ignore")]
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    pub observation_uuid: String,
}

/// Relates the finding to a set of referenced risks that were used to determine the finding.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AssociatedRisk {
    /// A machine-oriented identifier reference to a risk defined in the list of risks.
    #[derivative(PartialEq = "ignore")]
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    pub risk_uuid: String,
}

/// Captures an assessor's conclusions regarding the degree to which an objective is
/// satisfied.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct TargetClass {
    /// A human-readable description of the assessor's conclusions regarding the degree to which
    /// an objective is satisfied.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub description: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub implementation_status: Option<ImplementationStatus>,
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
    /// A determination of if the objective is satisfied or not within a given system.
    pub status: StatusClass,
    /// A machine-oriented identifier reference for a specific target qualified by the type.
    #[serde(
        serialize_with = "validation::ser_token",
        deserialize_with = "validation::deser_token"
    )]
    pub target_id: String,
    /// The title for this objective status.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub title: Option<String>,
    /// Identifies the type of the target.
    #[serde(rename = "type")]
    pub objective_status_type: FindingTargetType,
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
/// Identifies the type of the target.
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
/// The common name of the protocol, which should be the appropriate "service name" from the
/// IANA Service Name and Transport Protocol Port Number Registry.
///
/// Describes a function performed for a given authorized privilege by this user class.
///
/// A short common name, abbreviation, or acronym for the user.
///
/// Indicates the value of the facet.
///
/// The digest method by which a hash is derived.
///
/// Indicates the type of phone number.
///
/// A category describing the purpose of the component.
///
/// Identifies how the observation was made.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
#[derive(EnumString, Derivative)]
#[derivative(PartialEq)]
pub enum FindingTargetType {
    #[serde(rename = "objective-id")]
    ObjectiveId,
    #[serde(rename = "statement-id")]
    StatementId,
}

/// A determination of if the objective is satisfied or not within a given system.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct StatusClass {
    /// The reason the objective was given it's status.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
    /// An indication as to whether the objective is satisfied or not.
    pub state: ObjectiveStatusState,
}
/// An indication as to whether the objective is satisfied or not.
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
/// Points to an assessment objective.
///
/// A reference to a control with a corresponding id value. When referencing an externally
/// defined control, the Control Identifier Reference must be used in the context of the
/// external / imported OSCAL instance (e.g., uri-reference).
///
/// Used to constrain the selection to only specificity identified statements.
///
/// A human-oriented identifier reference to a role performed.
///
/// An optional textual providing a sub-type or characterization of the part's name, or a
/// category to which the part belongs.
///
/// A unique identifier for the part.
///
/// A textual label that uniquely identifies the part's semantic type, which exists in a
/// value space qualified by the ns.
///
/// A reference to a role performed by a party.
///
/// The type of action documented by the assembly, such as an approval.
///
/// A unique identifier for the role.
///
/// A point to the role-id of the role in which the party is making the log entry.
///
/// A textual label that provides a sub-type or characterization of the part's name. This can
/// be used to further distinguish or discriminate between the semantics of multiple parts of
/// the same control with the same name and ns.
///
/// For a party, this can optionally be used to specify the role the actor was performing.
///
/// A machine-oriented identifier reference for a specific target qualified by the type.
///
/// Reference to a role by UUID.
///
/// The name of the risk metric within the specified system.
///
/// Describes the type of relationship provided by the link's hypertext reference. This can
/// be an indicator of the link's purpose.
///
/// Indicates the type of address.
///
/// Used to indicate the type of object pointed to by the uuid-ref within a subject.
///
/// Indicates the type of assessment subject, such as a component, inventory, item, location,
/// or party represented by this selection statement.
///
/// A textual label that uniquely identifies the part's semantic type.
///
/// Identifies the implementation status of the control or control objective.
///
/// The reason the objective was given it's status.
///
/// The type of task.
///
/// Identifies the nature of the observation. More than one may be used to further qualify
/// and enable filtering.
///
/// Identifies whether this is a recommendation, such as from an assessor or tool, or an
/// actual plan accepted by the system owner.
///
/// Describes the status of the associated risk.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
#[derive(EnumString, Derivative)]
#[derivative(PartialEq)]
pub enum ObjectiveStatusState {
    #[serde(rename = "not-satisfied")]
    NotSatisfied,
    Satisfied,
}

/// Used to define data objects that are used in the assessment plan, that do not appear in
/// the referenced SSP.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ResultLocalDefinitions {
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assessment_assets: Option<AssessmentAssets>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Component>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inventory_items: Option<Vec<InventoryItem>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<Task>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<SystemUser>>,
}

/// Identifies the assets used to perform this assessment, such as the assessment team,
/// scanning tools, and assumptions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AssessmentAssets {
    pub assessment_platforms: Vec<AssessmentPlatform>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Component>>,
}

/// Used to represent the toolset used to perform aspects of the assessment.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AssessmentPlatform {
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
    /// The title or name for the assessment platform.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub title: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uses_components: Option<Vec<UsesComponent>>,
    /// A machine-oriented, globally unique identifier with cross-instance scope that can be used
    /// to reference this assessment platform elsewhere in this or other OSCAL instances. The
    /// locally defined UUID of the assessment platform can be used to reference the data item
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

/// The set of components that are used by the assessment platform.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct UsesComponent {
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
    pub status: Status,
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
/// Points to an assessment objective.
///
/// A reference to a control with a corresponding id value. When referencing an externally
/// defined control, the Control Identifier Reference must be used in the context of the
/// external / imported OSCAL instance (e.g., uri-reference).
///
/// Used to constrain the selection to only specificity identified statements.
///
/// A human-oriented identifier reference to a role performed.
///
/// An optional textual providing a sub-type or characterization of the part's name, or a
/// category to which the part belongs.
///
/// A unique identifier for the part.
///
/// A textual label that uniquely identifies the part's semantic type, which exists in a
/// value space qualified by the ns.
///
/// A reference to a role performed by a party.
///
/// The type of action documented by the assembly, such as an approval.
///
/// A unique identifier for the role.
///
/// A point to the role-id of the role in which the party is making the log entry.
///
/// A textual label that provides a sub-type or characterization of the part's name. This can
/// be used to further distinguish or discriminate between the semantics of multiple parts of
/// the same control with the same name and ns.
///
/// For a party, this can optionally be used to specify the role the actor was performing.
///
/// A machine-oriented identifier reference for a specific target qualified by the type.
///
/// Reference to a role by UUID.
///
/// The name of the risk metric within the specified system.
///
/// Describes the type of relationship provided by the link's hypertext reference. This can
/// be an indicator of the link's purpose.
///
/// Indicates the type of address.
///
/// Used to indicate the type of object pointed to by the uuid-ref within a subject.
///
/// Indicates the type of assessment subject, such as a component, inventory, item, location,
/// or party represented by this selection statement.
///
/// A textual label that uniquely identifies the part's semantic type.
///
/// Identifies the implementation status of the control or control objective.
///
/// The reason the objective was given it's status.
///
/// The type of task.
///
/// Identifies the nature of the observation. More than one may be used to further qualify
/// and enable filtering.
///
/// Identifies whether this is a recommendation, such as from an assessor or tool, or an
/// actual plan accepted by the system owner.
///
/// Describes the status of the associated risk.
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
pub struct Status {
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
    /// The operational status.
    pub state: State,
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
/// Points to an assessment objective.
///
/// A reference to a control with a corresponding id value. When referencing an externally
/// defined control, the Control Identifier Reference must be used in the context of the
/// external / imported OSCAL instance (e.g., uri-reference).
///
/// Used to constrain the selection to only specificity identified statements.
///
/// A human-oriented identifier reference to a role performed.
///
/// An optional textual providing a sub-type or characterization of the part's name, or a
/// category to which the part belongs.
///
/// A unique identifier for the part.
///
/// A textual label that uniquely identifies the part's semantic type, which exists in a
/// value space qualified by the ns.
///
/// A reference to a role performed by a party.
///
/// The type of action documented by the assembly, such as an approval.
///
/// A unique identifier for the role.
///
/// A point to the role-id of the role in which the party is making the log entry.
///
/// A textual label that provides a sub-type or characterization of the part's name. This can
/// be used to further distinguish or discriminate between the semantics of multiple parts of
/// the same control with the same name and ns.
///
/// For a party, this can optionally be used to specify the role the actor was performing.
///
/// A machine-oriented identifier reference for a specific target qualified by the type.
///
/// Reference to a role by UUID.
///
/// The name of the risk metric within the specified system.
///
/// Describes the type of relationship provided by the link's hypertext reference. This can
/// be an indicator of the link's purpose.
///
/// Indicates the type of address.
///
/// Used to indicate the type of object pointed to by the uuid-ref within a subject.
///
/// Indicates the type of assessment subject, such as a component, inventory, item, location,
/// or party represented by this selection statement.
///
/// A textual label that uniquely identifies the part's semantic type.
///
/// Identifies the implementation status of the control or control objective.
///
/// The reason the objective was given it's status.
///
/// The type of task.
///
/// Identifies the nature of the observation. More than one may be used to further qualify
/// and enable filtering.
///
/// Identifies whether this is a recommendation, such as from an assessor or tool, or an
/// actual plan accepted by the system owner.
///
/// Describes the status of the associated risk.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
#[derive(EnumString, Derivative)]
#[derivative(PartialEq)]
pub enum State {
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

/// Represents a scheduled event or milestone, which may be associated with a series of
/// assessment actions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Task {
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub associated_activities: Option<Vec<AssociatedActivity>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<TaskDependency>>,
    /// A human-readable description of this task.
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
    pub responsible_roles: Option<Vec<ResponsibleRole>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subjects: Option<Vec<SubjectOfAssessment>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<Task>>,
    /// The timing under which the task is intended to occur.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timing: Option<EventTiming>,
    /// The title for this task.
    #[serde(deserialize_with = "validation::deser_markup")]
    pub title: String,
    /// The type of task.
    #[serde(rename = "type")]
    #[serde(
        serialize_with = "validation::ser_token",
        deserialize_with = "validation::deser_token"
    )]
    pub task_type: String,
    /// A machine-oriented, globally unique identifier with cross-instance scope that can be used
    /// to reference this task elsewhere in this or other OSCAL instances. The locally defined
    /// UUID of the task can be used to reference the data item locally or globally (e.g., in an
    /// imported OSCAL instance). This UUID should be assigned per-subject, which means it should
    /// be consistently used to identify the same subject across revisions of the document.
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    #[derivative(PartialEq = "ignore")]
    pub uuid: String,
}

/// Identifies an individual activity to be performed as part of a task.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AssociatedActivity {
    /// A machine-oriented identifier reference to an activity defined in the list of activities.
    #[derivative(PartialEq = "ignore")]
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    pub activity_uuid: String,
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
    pub subjects: Vec<SubjectOfAssessment>,
}

/// Used to indicate that a task is dependent on another task.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct TaskDependency {
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
    /// A machine-oriented identifier reference to a unique task.
    #[derivative(PartialEq = "ignore")]
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    pub task_uuid: String,
}

/// The timing under which the task is intended to occur.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct EventTiming {
    /// The task is intended to occur at the specified frequency.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub at_frequency: Option<FrequencyCondition>,
    /// The task is intended to occur on the specified date.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_date: Option<OnDateCondition>,
    /// The task is intended to occur within the specified date range.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub within_date_range: Option<OnDateRangeCondition>,
}

/// The task is intended to occur at the specified frequency.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct FrequencyCondition {
    /// The task must occur after the specified period has elapsed.
    #[serde(
        serialize_with = "validation::ser_positive_int",
        deserialize_with = "validation::deser_positive_int"
    )]
    pub period: i64,
    /// The unit of time for the period.
    pub unit: TimeUnit,
}
/// The unit of time for the period.
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
/// The common name of the protocol, which should be the appropriate "service name" from the
/// IANA Service Name and Transport Protocol Port Number Registry.
///
/// Describes a function performed for a given authorized privilege by this user class.
///
/// A short common name, abbreviation, or acronym for the user.
///
/// Indicates the value of the facet.
///
/// The digest method by which a hash is derived.
///
/// Indicates the type of phone number.
///
/// A category describing the purpose of the component.
///
/// Identifies how the observation was made.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
#[derive(EnumString, Derivative)]
#[derivative(PartialEq)]
pub enum TimeUnit {
    Days,
    Hours,
    Minutes,
    Months,
    Seconds,
    Years,
}

/// The task is intended to occur on the specified date.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct OnDateCondition {
    /// The task must occur on the specified date.
    pub date: String,
}

/// The task is intended to occur within the specified date range.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct OnDateRangeCondition {
    /// The task must occur on or before the specified date.
    pub end: String,
    /// The task must occur on or after the specified date.
    pub start: String,
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

/// Describes an individual observation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Observation {
    /// Date/time stamp identifying when the finding information was collected.
    #[serde(
        serialize_with = "validation::ser_dttz",
        deserialize_with = "validation::deser_dttz"
    )]
    pub collected: String,
    /// A human-readable description of this assessment observation.
    pub description: String,
    /// Date/time identifying when the finding information is out-of-date and no longer valid.
    /// Typically used with continuous assessment scenarios.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_dttz_opt",
        deserialize_with = "validation::deser_dttz_opt"
    )]
    pub expires: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    pub methods: Vec<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origins: Option<Vec<Origin>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relevant_evidence: Option<Vec<RelevantEvidence>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subjects: Option<Vec<IdentifiesTheSubject>>,
    /// The title for this observation.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub title: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
    /// A machine-oriented, globally unique identifier with cross-instance scope that can be used
    /// to reference this observation elsewhere in this or other OSCAL instances. The locally
    /// defined UUID of the observation can be used to reference the data item locally or
    /// globally (e.g., in an imorted OSCAL instance). This UUID should be assigned per-subject,
    /// which means it should be consistently used to identify the same subject across revisions
    /// of the document.
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    #[derivative(PartialEq = "ignore")]
    pub uuid: String,
}

/// Links this observation to relevant evidence.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RelevantEvidence {
    /// A human-readable description of this evidence.
    pub description: String,
    /// A resolvable URL reference to relevant evidence.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_uri_ref_opt",
        deserialize_with = "validation::deser_uri_ref_opt"
    )]
    pub href: Option<String>,
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

/// A human-oriented identifier reference to a resource. Use type to indicate whether the
/// identified resource is a component, inventory item, location, user, or something else.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct IdentifiesTheSubject {
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
    /// A machine-oriented identifier reference to a component, inventory-item, location, party,
    /// user, or resource using it's UUID.
    #[derivative(PartialEq = "ignore")]
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    pub subject_uuid: String,
    /// The title or name for the referenced subject.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Used to indicate the type of object pointed to by the uuid-ref within a subject.
    #[serde(rename = "type")]
    pub identifies_the_subject_type: String,
}

/// An identified risk.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct IdentifiedRisk {
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub characterizations: Option<Vec<Characterization>>,
    /// The date/time by which the risk must be resolved.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_dttz_opt",
        deserialize_with = "validation::deser_dttz_opt"
    )]
    pub deadline: Option<String>,
    /// A human-readable summary of the identified risk, to include a statement of how the risk
    /// impacts the system.
    #[serde(deserialize_with = "validation::deser_markup")]
    pub description: String,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mitigating_factors: Option<Vec<MitigatingFactor>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origins: Option<Vec<Origin>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_observations: Option<Vec<RiskRelatedObservation>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remediations: Option<Vec<RiskResponse>>,
    /// A log of all risk-related tasks taken.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub risk_log: Option<RiskLog>,
    /// An summary of impact for how the risk affects the system.
    #[serde(deserialize_with = "validation::deser_markup")]
    pub statement: String,
    #[serde(
        serialize_with = "validation::ser_token",
        deserialize_with = "validation::deser_token"
    )]
    pub status: String,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threat_ids: Option<Vec<ThreatId>>,
    /// The title for this risk.
    #[serde(deserialize_with = "validation::deser_markup")]
    pub title: String,
    /// A machine-oriented, globally unique identifier with cross-instance scope that can be used
    /// to reference this risk elsewhere in this or other OSCAL instances. The locally defined
    /// UUID of the risk can be used to reference the data item locally or globally (e.g., in an
    /// imported OSCAL instance). This UUID should be assigned per-subject, which means it should
    /// be consistently used to identify the same subject across revisions of the document.
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    #[derivative(PartialEq = "ignore")]
    pub uuid: String,
}

/// A collection of descriptive data about the containing object from a specific origin.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Characterization {
    pub facets: Vec<Facet>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    pub origin: Origin,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
}

/// An individual characteristic that is part of a larger set produced by the same actor.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Facet {
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    /// The name of the risk metric within the specified system.
    #[serde(
        serialize_with = "validation::ser_token",
        deserialize_with = "validation::deser_token"
    )]
    pub name: String,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
    /// Specifies the naming system under which this risk metric is organized, which allows for
    /// the same names to be used in different systems controlled by different parties. This
    /// avoids the potential of a name clash.
    #[serde(
        serialize_with = "validation::ser_uri",
        deserialize_with = "validation::deser_uri"
    )]
    pub system: String,
    /// Indicates the value of the facet.
    pub value: String,
}

/// Describes an existing mitigating factor that may affect the overall determination of the
/// risk, with an optional link to an implementation statement in the SSP.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct MitigatingFactor {
    /// A human-readable description of this mitigating factor.
    #[serde(deserialize_with = "validation::deser_markup")]
    pub description: String,
    /// A machine-oriented, globally unique identifier with cross-instance scope that can be used
    /// to reference this implementation statement elsewhere in this or other OSCAL instancess.
    /// The locally defined UUID of the implementation statement can be used to reference the
    /// data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be
    /// assigned per-subject, which means it should be consistently used to identify the same
    /// subject across revisions of the document.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_uuid_opt",
        deserialize_with = "validation::deser_uuid_opt"
    )]
    pub implementation_uuid: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subjects: Option<Vec<IdentifiesTheSubject>>,
    /// A machine-oriented, globally unique identifier with cross-instance scope that can be used
    /// to reference this mitigating factor elsewhere in this or other OSCAL instances. The
    /// locally defined UUID of the mitigating factor can be used to reference the data item
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

/// Relates the finding to a set of referenced observations that were used to determine the
/// finding.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RiskRelatedObservation {
    /// A machine-oriented identifier reference to an observation defined in the list of
    /// observations.
    #[derivative(PartialEq = "ignore")]
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    pub observation_uuid: String,
}

/// Describes either recommended or an actual plan for addressing the risk.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RiskResponse {
    /// A human-readable description of this response plan.
    #[serde(deserialize_with = "validation::deser_markup")]
    pub description: String,
    /// Identifies whether this is a recommendation, such as from an assessor or tool, or an
    /// actual plan accepted by the system owner.
    #[serde(
        serialize_with = "validation::ser_token",
        deserialize_with = "validation::deser_token"
    )]
    pub lifecycle: String,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origins: Option<Vec<Origin>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_assets: Option<Vec<RequiredAsset>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<Task>>,
    /// The title for this response activity.
    #[serde(deserialize_with = "validation::deser_markup")]
    pub title: String,
    /// A machine-oriented, globally unique identifier with cross-instance scope that can be used
    /// to reference this remediation elsewhere in this or other OSCAL instances. The locally
    /// defined UUID of the risk response can be used to reference the data item locally or
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

/// Identifies an asset required to achieve remediation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RequiredAsset {
    /// A human-readable description of this required asset.
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
    pub subjects: Option<Vec<IdentifiesTheSubject>>,
    /// The title for this required asset.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub title: Option<String>,
    /// A machine-oriented, globally unique identifier with cross-instance scope that can be used
    /// to reference this required asset elsewhere in this or other OSCAL instances. The locally
    /// defined UUID of the asset can be used to reference the data item locally or globally
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

/// A log of all risk-related tasks taken.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RiskLog {
    pub entries: Vec<RiskLogEntry>,
}

/// Identifies an individual risk response that occurred as part of managing an identified
/// risk.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RiskLogEntry {
    /// A human-readable description of what was done regarding the risk.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub description: Option<String>,
    /// Identifies the end date and time of the event. If the event is a point in time, the start
    /// and end will be the same date and time.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_dttz_opt",
        deserialize_with = "validation::deser_dttz_opt"
    )]
    pub end: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logged_by: Option<Vec<LoggedBy>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_responses: Option<Vec<RiskResponseReference>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
    /// Identifies the start date and time of the event.
    #[serde(
        serialize_with = "validation::ser_dttz",
        deserialize_with = "validation::deser_dttz"
    )]
    pub start: String,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_token_opt",
        deserialize_with = "validation::deser_token_opt"
    )]
    pub status_change: Option<String>,
    /// The title for this risk log entry.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub title: Option<String>,
    /// A machine-oriented, globally unique identifier with cross-instance scope that can be used
    /// to reference this risk log entry elsewhere in this or other OSCAL instances. The locally
    /// defined UUID of the risk log entry can be used to reference the data item locally or
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

/// Identifies an individual risk response that this log entry is for.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RiskResponseReference {
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub props: Option<Vec<Property>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_tasks: Option<Vec<TaskReference>>,
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "validation::deser_markup_opt")]
    pub remarks: Option<String>,
    /// A machine-oriented identifier reference to a unique risk response.
    #[derivative(PartialEq = "ignore")]
    #[serde(
        serialize_with = "validation::ser_uuid",
        deserialize_with = "validation::deser_uuid"
    )]
    pub response_uuid: String,
}

/// A pointer, by ID, to an externally-defined threat.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Builder, Derivative)]
#[builder(setter(into, strip_option))]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ThreatId {
    /// An optional location for the threat data, from which this ID originates.
    #[builder(setter(into, strip_option), default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "validation::ser_uri_ref_opt",
        deserialize_with = "validation::deser_uri_ref_opt"
    )]
    pub href: Option<String>,
    pub id: String,
    /// Specifies the source of the threat information.
    pub system: String,
}
