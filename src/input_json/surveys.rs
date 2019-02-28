use crate::db::surveys::UpdateSurveyData;

#[derive(Serialize, Deserialize, Clone)]
pub struct UpdateSurveyJSON {
    _token: Option<String>,
    title: Option<String>,
    published: Option<bool>,
    description: Option<String>,
    category: Option<String>,
}

impl From<UpdateSurveyJSON> for UpdateSurveyData {
    fn from(d: UpdateSurveyJSON) -> Self {
        let UpdateSurveyJSON {
            _token,
            title,
            published,
            description,
            category,
        } = d;
        Self {
            title,
            published,
            description,
            category,
        }
    }
}
