<h1 align="center">
Despoina 🔮
</h1>
<h6 align="center">Despoina, the goddess of Mysteries. daughter of Demeter and Poseidon</h6>

## Introduction

This is a rust code to send each student's grade privately using
`lettre` to send emails from rust and `csv` to deal with CSV file
of grades and students' names.

## Send Grades

As you can see in you should have a sheet like
this (if your column names are different feel free to change them in code):

```python
id = "شماره دانشجویی"
name = "نام"
email = "ایمیل"
grade = 100
mistake = "توضیحات"
```

```html
<html>
  <body dir="rtl">
    <p>
      با سلام<br />
      دانشجوی عزیز {{ name }}<br />
      نمره تمرین شما
    </p>
  </body>
</html>
```

This is a tera template, and you have variables that is set for each student.
Please note that you can check the emails before actually sending them with `--dry-run` flag.

## Enter SMTP/Email Address

Enter your SMTP config and subject in config file.

```yaml
smtp:
  server: smtp.gmail.com
  username: example@gmail.com
  password: secret
email:
  subject: test
```

Note that if you use Gmail you must go to your Google account and change
_Less secure app access_ [here](https://myaccount.google.com/lesssecureapps) by turning on
_Allow less secure apps_. Otherwise, you need to use
[application-specific passwords](https://support.google.com/accounts/answer/185833?hl=en).

## Installation

You just need to install rust using [rustup](https://rustup.rs/).

```bash
cp conf.yml.example conf.yml
cargo run -- -c ./conf.yml
```
