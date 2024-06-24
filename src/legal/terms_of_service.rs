use crate::prelude::*;

struct Tos;
impl Component for Tos {
    fn render(&self) -> String {
        r#"
        <div class="prose bg-slate-300 rounded p-2 md:p-4">
            <h1>Terms of Service</h1>
            <p>
              This document outlines the terms and conditions under which users
              may access and use the Orton-Gillingham (OG) Lesson Planner. This
              tool is a free and open-source project from New Jersey. By
              accessing or using the services, you agree to be bound by these
              terms. If you do not agree with any part of these terms, please
              do not use the website.
            </p>

            <h2>1. Acceptance of Terms</h2>
            <p>
              By using the services provided by OG Lesson Planner, you
              acknowledge that you have read, understood, and agree to be bound
              by these terms of service.
            </p>

            <h2>2. Service Description</h2>
            <p>
              OG Lesson Planner is a tool to help create lesson plans using the
              Orton-Gillingham methodology.
            </p>

            <h2>3. User Accounts</h2>
            <p>
              Users must create an account to access the service. You are
              responsible for maintaining the confidentiality of your account
              information, including your password.
            </p>

            <h2>4. Payment and Subscription</h2>
            <p>
              This tool is made available for free, however the tool creators
              reserve the right to begin charging for services at any time
              without advance notice.
            </p>

            <h2>Subscription Cancellation</h2>
            <p>
              No subscription is available at this time. Contact
              <a href="mailto:jdevries3133@gmail.com">jdevries3133@gmail.com</a>
              to request account deletion or export of your user data.
            </p>

            <h2>5. User Conduct</h2>
            <p>
              Users agree to use the service in a lawful and responsible
              manner. Any misuse or unauthorized use may result in the
              termination of your account.
            </p>

            <h2>6. Intellectual Property</h2>
            <p>
              This is a free and open-source project made generally available
              under the Affero General Public License (AGPL). See <a
              href="https://github.com/jdevries3133/og">our source
              code repository on GitHub</a> for details.
            </p>

            <h2>7. Limitation of Liability</h2>
            <p>
              Orton-Gillingham planner is not liable for any direct, indirect,
              incidental, consequential, or punitive damages arising out of the
              use or inability to use the service.
            </p>

            <h2>8. Termination of Services</h2>
            <p>
              We reserves the right to terminate or suspend access to the
              service at our discretion, without notice, for any reason,
              including but not limited to a breach of these terms.
            </p>

            <h2>9. Changes to Terms</h2>
            <p>
              We reserves the right to modify or update these terms of service
              at any time. Users will be notified of significant changes, and
              continued use of the services after such modifications
              constitutes acceptance of the updated terms.
            </p>

            <p>
              By using the service, you agree to abide by these terms of
              service. If you have any questions or concerns, please contact us
              at <a
              href="mailto:jdevries3133@gmail.com">jdevries3133@gmail.com</a>.
            </p>
        </div>
        "#
        .into()
    }
}

pub async fn get_tos() -> impl IntoResponse {
    Page {
        title: "Terms of Service",
        children: &PageContainer { children: &Tos {} },
    }
    .render()
}
