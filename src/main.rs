use clap::Parser;
use csv;
use lettre::{
    message::header, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    AsyncTransport, Message, Tokio1Executor,
};
use serde::Deserialize;
use std::{
    error::Error,
    fs::{self, File},
    path::PathBuf,
};
use tera::Tera;

#[derive(Deserialize)]
struct Student {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    smtp: SmtpConfig,
    email: EmailConfig,
}

#[derive(Debug, Deserialize)]
struct SmtpConfig {
    server: String,
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct EmailConfig {
    subject: String,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE", required = true)]
    config: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let config_path = cli.config;
    let config_file = fs::read_to_string(config_path)?;

    let config: Config = serde_yaml::from_str(&config_file)?;

    let tera = Tera::new("templates/**/*.html")?;

    let mut tera_context = tera::Context::new();

    let csv_file = File::open("results.csv")?;
    let mut csv_reader = csv::Reader::from_reader(csv_file);
    for result in csv_reader.deserialize() {
        let record: Student = result?;
        let student_name = &record.name;
        let student_email = &record.email;

        tera_context.insert("name", student_name);

        let email_html = tera.render("email_body.html", &tera_context)?;

        let creds = Credentials::new(
            config.smtp.username.to_owned(),
            config.smtp.password.to_owned(),
        );

        let email = Message::builder()
            .from(config.smtp.username.parse()?)
            .to(student_email.parse()?)
            .subject(&config.email.subject)
            .header(header::ContentType::TEXT_HTML)
            .body(email_html)?;

        let mailer: AsyncSmtpTransport<Tokio1Executor> =
            AsyncSmtpTransport::<Tokio1Executor>::relay(&config.smtp.server)
                .unwrap()
                .credentials(creds)
                .build();

        match mailer.send(email).await {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {e:?}"),
        }
    }

    Ok(())
}
