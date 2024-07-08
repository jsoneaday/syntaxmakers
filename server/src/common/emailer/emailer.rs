use async_trait::async_trait;
use uuid::Uuid;
use super::model::EmailError;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;
use dotenv::dotenv;

pub const SIGNUP_EMAIL_CONFIRMATION_BODY: &str = "Thank you for signing up. Please click the button below to confirm your email address:";
pub const CHANGE_EMAIL_CONFIRMATION_BODY: &str = "Your email has been changed. Please click the button below to confirm your email address:";

#[derive(Clone, Debug)]
pub struct Emailer {
    user_name: String,
    password: String
}

#[async_trait]
pub trait EmailerService {
    async fn send_email_confirm_requirement(&self, profile_id: i64, email_body: String, full_name: String, new_email: String, unique_key: Uuid) 
        -> Result<(), EmailError>;

    async fn receive_email_confirm(&self, profile_id: i64, new_email: String, unique_key: Uuid) -> Result<(), EmailError>;
}

#[allow(unused)]
#[async_trait]
impl EmailerService for Emailer {
    /// e.g. email_body - "Thank you for signing up. Please click the button below to confirm your email address:"
    async fn send_email_confirm_requirement(&self, profile_id: i64, email_body: String, full_name: String, new_email: String, unique_key: Uuid) 
        -> Result<(), EmailError> {
        let subject = "SyntaxMakers: email confirmation";
        let body = Emailer::get_send_email_body(
            full_name.as_str(), 
            email_body.as_str(), 
            env::var("CONFIRMATION_LINK").unwrap().as_str()
        );

        let email = Message::builder()
            .from("SyntaxMakers Support <support@syntaxmakers.com>".parse().unwrap())
            .reply_to("SyntaxMakers Support <support@syntaxmakers.com>".parse().unwrap())
            .to(format!("{} <{}>", full_name, new_email).parse().unwrap())
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body) 
            .unwrap();
        let creds = Credentials::new(self.user_name.clone(), self.password.clone());
        let mailer = SmtpTransport::relay("smtp.mailgun.org").unwrap().credentials(creds).build();

        match mailer.send(&email) {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("{}", e);
                Err(EmailError::EmailSendFailed)
            }
        }        
    }

    async fn receive_email_confirm(&self, profile_id: i64, new_email: String, unique_key: Uuid) -> Result<(), EmailError> {
        Ok(())
    }
}

impl Emailer {
    pub fn new() -> Self {
        dotenv().ok();
        Emailer {
            user_name: env::var("MAILGUN_USER_NAME").unwrap(),
            password: env::var("MAILGUN_PASSWORD").unwrap()
        }
    }

    fn get_send_email_body(full_name: &str, msg_body: &str, email_confirm_link: &str) -> String {
        String::from(format!("
            <!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">
            <html xmlns=\"http://www.w3.org/1999/xhtml\">
            <head>
                <meta http-equiv=\"Content-Type\" content=\"text/html; charset=UTF-8\" />
                <title>Email Confirmation</title>
            </head>
            <body style=\"margin: 0; padding: 0; font-family: Arial, sans-serif; font-size: 16px; line-height: 1.6; color: #333333;\">
                <table border=\"0\" cellpadding=\"0\" cellspacing=\"0\" width=\"100%\">
                    <tr>
                        <td>
                            <table align=\"center\" border=\"0\" cellpadding=\"0\" cellspacing=\"0\" width=\"600\" style=\"border-collapse: collapse;\">
                                <tr>
                                    <td bgcolor=\"#ffffff\" style=\"padding: 40px 30px 40px 30px;\">
                                        <table border=\"0\" cellpadding=\"0\" cellspacing=\"0\" width=\"100%\">
                                            <tr>
                                                <td style=\"font-size: 24px; font-weight: bold;\">
                                                    Confirm Your Email Address
                                                </td>
                                            </tr>
                                            <tr>
                                                <td style=\"padding: 20px 0 30px 0;\">
                                                    Hello {},<br><br>
                                                    {}
                                                </td>
                                            </tr>
                                            <tr>
                                                <td align=\"center\" style=\"padding: 20px 0 30px 0;\">
                                                    <a href=\"{}\" style=\"background-color: #007bff; border: none; color: white; padding: 15px 32px; text-align: center; text-decoration: none; display: inline-block; font-size: 16px; margin: 4px 2px; cursor: pointer;\">Confirm Email</a>
                                                </td>
                                            </tr>
                                            <tr>
                                                <td>
                                                    If you didn't create an account, you can safely ignore this email.
                                                </td>
                                            </tr>
                                        </table>
                                    </td>
                                </tr>
                            </table>
                        </td>
                    </tr>
                </table>
            </body>
            </html>
        ", 
        full_name, 
        msg_body,
        email_confirm_link))
    }
}