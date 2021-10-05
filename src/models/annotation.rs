// SPDX-FileCopyrightText: 2020-2021 HH Partners
//
// SPDX-License-Identifier: MIT

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// <https://spdx.github.io/spdx-spec/8-annotations/>
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Annotation {
    /// https://spdx.github.io/spdx-spec/8-annotations/#81-annotator
    pub annotator: String,

    /// https://spdx.github.io/spdx-spec/8-annotations/#82-annotation-date
    pub annotation_date: DateTime<Utc>,

    /// https://spdx.github.io/spdx-spec/8-annotations/#83-annotation-type
    pub annotation_type: AnnotationType,

    /// https://spdx.github.io/spdx-spec/8-annotations/#84-spdx-identifier-reference
    // TODO: According to the spec this is mandatory, but the example file doesn't
    // have it.
    pub spdx_identifier_reference: Option<String>,

    /// https://spdx.github.io/spdx-spec/8-annotations/#85-annotation-comment
    #[serde(rename = "comment")]
    pub annotation_comment: String,
}

/// <https://spdx.github.io/spdx-spec/8-annotations/#83-annotation-type>
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AnnotationType {
    Review,
    Other,
}

#[cfg(test)]
mod test {
    use chrono::TimeZone;

    use crate::models::SPDX;

    use super::*;

    #[test]
    fn annotator() {
        let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
        assert_eq!(
            spdx.annotations[0].annotator,
            "Person: Jane Doe ()".to_string()
        );
    }

    #[test]
    fn annotation_date() {
        let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
        assert_eq!(
            spdx.annotations[0].annotation_date,
            Utc.ymd(2010, 1, 29).and_hms(18, 30, 22)
        );
    }

    #[test]
    fn annotation_type() {
        let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
        assert_eq!(spdx.annotations[0].annotation_type, AnnotationType::Other);
    }

    #[test]
    fn annotation_comment() {
        let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
        assert_eq!(
            spdx.annotations[0].annotation_comment,
            "Document level annotation"
        );
    }
}