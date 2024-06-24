use crate::prelude::*;

struct PrivacyPolicy;
impl Component for PrivacyPolicy {
    fn render(&self) -> String {
        r#"
        <div class="prose bg-slate-300 rounded p-2 md:p-4">
            <h1>Privacy Policy</h1>
            <p>
                Orton-Gillingham (OG) Lesson Planner will never share user data
                with a third party for any reason. This includes the email
                address you share with us, as well as user data generated while
                using the tool. Data is encrypted in-transit, and secured from
                unauthorized access at rest. Contact us at <a
                href="mailto:jdevries3133@gmail.com">jdevries3133@gmail.com</a>
                to request deletion or export of your OG Lesson Planner account
                and associated user data.
            </p>
        </div>
        "#
        .into()
    }
}

pub async fn get_privacy_policy() -> impl IntoResponse {
    Page {
        title: "Privacy Policy",
        children: &PageContainer {
            children: &PrivacyPolicy {},
        },
    }
    .render()
}
