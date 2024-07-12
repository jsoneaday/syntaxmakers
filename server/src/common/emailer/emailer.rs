use async_trait::async_trait;
use log::error;
use uuid::Uuid;
use crate::common::repository::base::Repository;
use crate::common::repository::developers::repo::ConfirmDevEmailFn;
use crate::common::repository::employers::repo::ConfirmEmpEmailFn;
use super::model::EmailError;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;
use dotenv::dotenv;

pub const SIGNUP_EMAIL_CONFIRMATION_BODY: &str = "Thank you for signing up to SyntaxMakers. The Jobs Posting site for specialized programming languages. <br/><br/>Please click the button below to confirm your email address:";
pub const CHANGE_EMAIL_CONFIRMATION_BODY: &str = "Your SyntaxMakers account email has been changed. Please click the button below to confirm your email address:";
pub const CHANGE_PASSWORD_CONFIRMATION_BODY: &str = "You have requested to change your SyntaxMakers account password. Please click the button below to change your password:";

pub const EMAIL_CONFIRMATION_SUBJECT: &str = "SyntaxMakers: email confirmation";
pub const CHANGE_PASSWORD_SUBJECT: &str = "SyntaxMakers: password change";

#[derive(Clone, Debug)]
pub struct Emailer {
    /// smtp provider service user_name
    user_name: String,
    /// smtp provider service password
    password: String,
    smtp_provider: String
}

#[async_trait]
pub trait EmailerSendService {
    async fn send_email(&self, sender_full_name: String, sender_address: String, receiver_full_name: String, receiver_adddress: String, subject: String, body: String) -> Result<(), EmailError>;

    async fn send_email_confirm_requirement(&self, is_email_change: bool, is_dev: bool, profile_id: i64, email_subject: String, email_body: String, full_name: String, new_email: String, unique_key: Uuid) 
        -> Result<(), EmailError>;    
}

#[allow(unused)]
#[async_trait]
impl EmailerSendService for Emailer {
    async fn send_email(&self, sender_full_name: String, sender_address: String, receiver_full_name: String, receiver_adddress: String, subject: String, body: String) -> Result<(), EmailError> {
        if !is_safe_text(body.as_str()) {
            return Err(EmailError::EmailBodyInvalidOnlyPlainTextAllowed);
        }

        let from = format!("{} <{}>", sender_full_name, sender_address);
        let email = Message::builder()
            .from(from.parse().unwrap())
            .reply_to(from.parse().unwrap())
            .to(format!("{} <{}>", receiver_full_name, receiver_adddress).parse().unwrap())
            .subject(subject)
            .header(ContentType::TEXT_HTML) // todo: reset to plain
            .body(body) 
            .unwrap();

        let creds = Credentials::new(self.user_name.clone(), self.password.clone());
        let mailer = SmtpTransport::relay(&self.smtp_provider).unwrap().credentials(creds).build();

        match mailer.send(&email) {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("{}", e);
                Err(EmailError::EmailSendFailed)
            }
        }     
    }

    /// e.g. email_body - "Thank you for signing up. Please click the button below to confirm your email address:"
    async fn send_email_confirm_requirement(&self, is_email_change: bool, is_dev: bool, profile_id: i64, email_subject: String, email_body: String, full_name: String, new_email: String, unique_key: Uuid) 
        -> Result<(), EmailError> {
        let body = if is_email_change {
            Emailer::get_send_change_email_body(
                is_dev,
                profile_id,
                full_name.as_str(), 
                email_body.as_str(), 
                env::var("CONFIRMATION_LINK").unwrap().as_str(),            
                &new_email,
                unique_key.to_string().as_str()
            )
        } else {
            Emailer::get_send_change_password_body(
                is_dev,
                profile_id,
                full_name.as_str(), 
                email_body.as_str(), 
                env::var("CHANGE_PASSWORD_LINK").unwrap().as_str(),            
                &new_email,
                unique_key.to_string().as_str()
            )
        };

        let email = Message::builder()
            .from("SyntaxMakers Support <support@syntaxmakers.com>".parse().unwrap())
            .reply_to("SyntaxMakers Support <support@syntaxmakers.com>".parse().unwrap())
            .to(format!("{} <{}>", full_name, new_email).parse().unwrap())
            .subject(email_subject)
            .header(ContentType::TEXT_HTML)
            .body(body) 
            .unwrap();
        let creds = Credentials::new(self.user_name.clone(), self.password.clone());
        let mailer = SmtpTransport::relay(&self.smtp_provider).unwrap().credentials(creds).build();

        match mailer.send(&email) {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("{}", e);
                Err(EmailError::EmailConfirmationSendFailed)
            }
        }        
    }
}

#[async_trait]
pub trait EmailerReceiveService<T: ConfirmDevEmailFn + ConfirmEmpEmailFn + Repository + Send + Sync> {
    async fn receive_email_confirm(&self, repo: &T, is_dev: bool, profile_id: i64, new_email: String, unique_key: Uuid) -> Result<(), EmailError>;
}

#[async_trait]
impl<T: ConfirmDevEmailFn + ConfirmEmpEmailFn + Repository + Send + Sync> EmailerReceiveService<T> for Emailer {
    async fn receive_email_confirm(&self, repo: &T, is_dev: bool, profile_id: i64, new_email: String, unique_key: Uuid) -> Result<(), EmailError> {
        if is_dev {
            match repo.confirm_dev_email(new_email, profile_id, unique_key.to_string()).await {
                Ok(()) => Ok(()),
                Err(e) => {
                    println!("emailer: {}", e);
                    Err(EmailError::from(e))
                }
            }
        } else {
            match repo.confirm_emp_email(new_email, profile_id, unique_key.to_string()).await {
                Ok(()) => Ok(()),
                Err(e) => {
                    error!("confirm_emp_email {}", e);
                    Err(EmailError::from(e))
                }
            }
        }
    }
}

impl Emailer {
    pub fn new() -> Self {
        dotenv().ok();
        Emailer {
            user_name: env::var("MAILGUN_USER_NAME").unwrap(),
            password: env::var("MAILGUN_PASSWORD").unwrap(),
            smtp_provider: env::var("SMTP_PROVIDER").unwrap()
        }
    }

    /// can confirm email or password change
    fn get_send_change_email_body(is_dev: bool, profile_id: i64, full_name: &str, msg_body: &str, email_confirm_link: &str, new_email: &str, unique_key: &str) -> String {
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
                                                    <a href=\"{}?is_dev={}&profile_id={}&new_email={}&unique_key={}&\" style=\"background-color: #007bff; border: none; color: white; padding: 15px 32px; text-align: center; text-decoration: none; display: inline-block; font-size: 16px; margin: 4px 2px; cursor: pointer;\">Confirm Email</a>
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
        email_confirm_link,
        is_dev,
        profile_id,
        new_email,
        unique_key    
    ))
    }

    fn get_send_change_password_body(is_dev: bool, profile_id: i64, full_name: &str, msg_body: &str, email_confirm_link: &str, new_email: &str, unique_key: &str) -> String {
        String::from(format!("
            <!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">
            <html xmlns=\"http://www.w3.org/1999/xhtml\">
            <head>
                <meta http-equiv=\"Content-Type\" content=\"text/html; charset=UTF-8\" />
                <title>Change Password</title>
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
                                                    Change Your Password
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
                                                    <a href=\"{}?is_dev={}&profile_id={}&new_email={}&unique_key={}&\" style=\"background-color: #007bff; border: none; color: white; padding: 15px 32px; text-align: center; text-decoration: none; display: inline-block; font-size: 16px; margin: 4px 2px; cursor: pointer;\">Confirm Email</a>
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
        email_confirm_link,
        is_dev,
        profile_id,
        new_email,
        unique_key    
    ))
    }
}

fn is_safe_text(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_alphanumeric() || c.is_ascii_punctuation() || c == ' ')
}