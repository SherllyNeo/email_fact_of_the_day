use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
    Tokio1Executor,
};
mod api_call;
extern crate reqwest;
use api_call::api_get_request;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /* Grabs email and email password from enviroment variables */
    let email = env::var("EMAIL").unwrap();
    let app_pass = env::var("EMAIL_PASS").unwrap();

    let mailing_list: Vec<&str> = vec![&email]; /* Has to be valid emails  */


    /* For each subscribe it will make credentials, log in and send the email fact and grab a random fact. This means that every user will get a random fact */
    for sub in mailing_list {
        let smtp_credentials = Credentials::new(email.to_string(), app_pass.to_string());
        let fact = api_get_request("https://uselessfacts.jsph.pl/random.txt?language=en").await;

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")?
            .credentials(smtp_credentials)
            .build();

        let from = format!("Jacob's fact app <{}>",email);
        let to = format!("<{}>",sub);
        let subject = "Today's random fact chosen by Jacob's app!";
        let body = format!("HEY! look at today's fact \n {} \n Hope you enjoyed this! \n",fact).to_string();

        send_email_smtp(&mailer, &from, &to, subject, body).await.expect("could not send email");

    }
    Ok(())
}


async fn send_email_smtp(
    mailer: &AsyncSmtpTransport<Tokio1Executor>,
    from: &str,
    to: &str,
    subject: &str,
    body: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let email = Message::builder()
        .from(from.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .body(body.to_string())?;

    mailer.send(email).await?;

    Ok(())
}
