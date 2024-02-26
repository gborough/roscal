use crate::{assessment::*, control::*, implementation::*, UpdateUuid};

macro_rules! impl_update_uuid {
    ( $( $t:ty ),* ) => {
        $(
            impl UpdateUuid for $t {
                fn update_uuid_v4(&mut self, rhs: &Self) -> &mut Self {
                    if self != rhs {
                        self.uuid = String::from(uuid::Uuid::new_v4());

                        return self
                    }

                    self
                }

                fn update_uuid_v5(&mut self, rhs: &Self) -> &mut Self {
                    if self != rhs {
                        self.uuid = String::from(uuid::Uuid::new_v5(
                            &uuid::Uuid::NAMESPACE_URL,
                            b"http://csrc.nist.gov/ns/oscal",
                        ));

                        return self
                    }

                    self
                }
            }
        )*
    };
}

impl_update_uuid!(
    assessment_plan::SecurityAssessmentPlanSap,
    assessment_results::SecurityAssessmentResultsSar,
    poam::PlanOfActionAndMilestonesPoaM,
    catalog::CatalogClass,
    profile::ProfileClass,
    component_definition::ComponentDefinitionClass,
    ssp::SystemSecurityPlanSsp
);
