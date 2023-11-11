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
};
use tera::Tera;
use Despoina::cli::{Cli, Config};

#[derive(Deserialize)]
struct Student {
    pub id: String,
    pub name: String,
    pub email: String,
    pub grade: String,
    pub mistake: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let config_path = cli.config;
    let config_file = fs::read_to_string(config_path)?;

    let config: Config = serde_yaml::from_str(&config_file)?;

    let tera = Tera::new("templates/**/*.html")?;

    let mut tera_context = tera::Context::new();

    let csv_path = cli.file;
    let csv_file = File::open(csv_path)?;
    let mut csv_reader = csv::Reader::from_reader(csv_file);
    for result in csv_reader.deserialize() {
        let record: Student = result?;
        let student_name = &record.name;
        let student_email = &record.email;
        let student_grade = &record.grade;
        let student_mistake = &record.mistake;

        tera_context.insert("student_name", student_name);
        tera_context.insert("exercise_name", &config.email.subject);
        tera_context.insert("student_grade", student_grade);
        tera_context.insert("student_mistake", student_mistake);
        tera_context.insert("sender_email", &config.smtp.username);

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
            Ok(_) => println!("Email sent to {}", student_email),
            Err(e) => panic!("Could not send email: {e:?}"),
        }
    }

    Ok(())
}
