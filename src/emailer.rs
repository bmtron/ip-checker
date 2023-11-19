use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use lettre::transport::smtp::authentication::Credentials;


async fn setup_email_creds_and_mailer() -> Result<AsyncSmtpTransport<Tokio1Executor>, Box<dyn std::error::Error>> {
    let smtp_credentials = Credentials::new("brendan@brendanmeehan.com".to_string(), "8h%hCz*SCe7DtF".to_string());

    let mailer: AsyncSmtpTransport<Tokio1Executor> = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.hostinger.com")?
        .credentials(smtp_credentials)
        .build();

    return Ok(mailer)
}

pub(crate) async fn send_ips_changed_alert() -> Result<(), Box<dyn std::error::Error>> {
    let mailer = setup_email_creds_and_mailer().await.unwrap();
    let from = "Mainframe Alerts <mainframe@brendanmeehan.com>";
    let to = "Me <brendan@brendanmeehan.com>";
    let subject = "Public IP has changed";
    let body = "You public facing IP address has changed. \n Make sure to download \
            and install updated VPN config files.".to_string();
    send_email_smtp(&mailer, from, to, subject, body).await
}
pub(crate) async fn send_test_email() -> Result<(), Box<dyn std::error::Error>> {

    let mailer = setup_email_creds_and_mailer().await.unwrap();
    let from = "Sender <mainframe@brendanmeehan.com>";
    let to = "receiver <brendan@brendanmeehan.com>";
    let subject = "Test email sent with rust";
    let body = "<h1>This is my first email test 123</h1>".to_string();

    send_email_smtp(&mailer, from, to, subject, body).await
}

async fn send_email_smtp(
    mailer: &AsyncSmtpTransport<Tokio1Executor>,
    from: &str,
    to: &str,
    subject: &str,
    body: String
) -> Result<(), Box<dyn std::error::Error>> {
    let email = Message::builder()
        .from(from.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .body(body.to_string())?;

    mailer.send(email).await?;

    Ok(())
}