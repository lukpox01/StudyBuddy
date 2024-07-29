use lettre::transport::smtp::authentication::Credentials;
use lettre::Message;
use lettre::SmtpTransport;
use lettre::Transport;

pub async fn send_verification_email(
    to: &str,
    code: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let email = Message::builder()
        .from("no-reply@example.com".parse()?)
        .to(to.parse()?)
        .subject("Email Verification")
        .body(format!(
            "Please verify your email using this code: {}",
            code
        ))?;

    let mailer = SmtpTransport::builder_dangerous("smtp.example.com")
        .credentials(Credentials::from(("username", "password")))
        .build();

    mailer.send(&email)?;
    Ok(())
}
