# Rust Email Library for MailSlurp
MailSlurp is an API for sending and receiving emails from dynamically allocated email addresses. It's designed for developers and QA teams to test applications, process inbound emails, send templated notifications, attachments, and more.

## Resources
- [Homepage](https://www.mailslurp.com)
- Get an [API KEY](https://app.mailslurp.com/sign-up/)
- [Documentation](https://www.mailslurp.com/docs/rust/docs/)
- [Rust Crate](https://crates.io/crates/mailslurp)
- [Lib.rs Docs](https://lib.rs/crates/mailslurp)
- [GitHub Source](https://github.com/mailslurp/mailslurp-client-rust)
- [Code Examples](https://github.com/mailslurp/examples)

## Install
Use cargo to add the MailSlurp crate to your project:

```bash
cargo add mailslurp
```

Or edit your Cargo.toml:

```toml
[dependencies]
mailslurp = "x.x.x"
```

Then run `cargo fetch`.

### Other dependencies
The MailSlurp library uses the `reqwest` HTTP client and async functions. Add `tokio` and `reqwest` to your Cargo file:

```toml
[dependencies]
tokio = { version = "1.4.0", features = ["full"] }
reqwest = { version = "0.11", features = ["json", "multipart"] }
```

## Configure
The MailSlurp SDK lets you create real email accounts for testing and development.

### Set API Key
MailSlurp is free to use but you must have an API Key. [Create an account](https://app.mailslurp.com/sign-up/) to obtain one:

![api key](https://www.mailslurp.com/assets/guides/find-api-key.png)

### Import and configure
MailSlurp is under the `mailslurp` namespace with `apis` and `models` modules. Controllers are provided that mimic the endpoints of the [REST API](https://api.mailslurp.com/swagger-ui.html).

```rust
use mailslurp::apis::configuration;
use mailslurp::apis::inbox_controller_api;

fn main() {
    // allow a long timeout so you can wait for emails to arrive
    const TIMEOUT: Duration = Duration::from_millis(60_000);
    let client: Client = reqwest::ClientBuilder::new()
        .timeout(TIMEOUT)
        .connect_timeout(TIMEOUT)
        .build()?;

    // read mailslurp api key from environment variable or a string
    let api_key: String = env::var("MAILSLURP_API_KEY")?;
    
    // configure mailslurp with base path, api key, and reqwest client
    let configuration = configuration::Configuration {
        // required fields
        base_path: "https://api.mailslurp.com".to_owned(),
        api_key: Some(configuration::ApiKey {
            prefix: None,
            key: api_key,
        }),
        client,
        // leave as none
        user_agent: None,
        basic_auth: None,
        oauth_access_token: None,
        bearer_access_token: None,
    };
}
```

## Calling controllers
The MailSlurp SDK is generated from the REST API and some methods take many optional parameters. Fill them with None or implement a Default trait if you require.

```rust
fn use_controllers() {
    // create an inbox
    let create_inbox_params = inbox_controller_api::CreateInboxParams{
        allow_team_access: None,
        description: None,
        email_address: None,
        expires_at: None,
        expires_in: None,
        favourite: None,
        name: None,
        tags: None,
        use_domain_pool: Some(true)
    };
    // methods are async and return results
    let inbox = inbox_controller_api::create_inbox(&configuration, create_inbox_params).await.ok().unwrap();
    // entity fields are options but see rest api docs for what will be set
    assert!(inbox.email_address.unwrap().contains("@mailslurp"));
}
```

## Common usage
Here are some examples for how to send and receive emails and attachments in Rust using MailSlurp.

### Create email accounts
MailSlurp inboxes have an email address and ID. Use the ID for further operations against the inbox.

```rust
fn create_inbox() {
    // create an inbox
    let create_inbox_params = inbox_controller_api::CreateInboxParams{
        allow_team_access: None,
        description: None,
        email_address: None,
        expires_at: None,
        expires_in: None,
        favourite: None,
        name: None,
        tags: None,
        use_domain_pool: Some(true)
    };
    // methods are async and return results
    let inbox = inbox_controller_api::create_inbox(&configuration, create_inbox_params).await.ok().unwrap();
}
```

### Send emails
You can send HTML emails in MailSlurp:

```rust
fn send_email() {
    let send_email_params = SendEmailAndConfirmParams{ inbox_id: inbox.id.unwrap().to_owned(), send_email_options: Some(SendEmailOptions{
        // common params
        to: Some(vec!["test@gmail.com".to_owned()]),
        body: Some("<html>Email body</html>".to_owned()),
        subject: Some("Test subject".to_owned()),
        is_html: Some(true),
        // extras
        attachments: None,
        bcc: None,
        cc: None,
        charset: None,
        from: None,
        reply_to: None,
        send_strategy: None,
        template: None,
        template_variables: None,
        to_contacts: None,
        to_group: None
    }) };
    let sent = inbox_controller_api::send_email_and_confirm(&configuration, send_email_params).await.ok().unwrap();
    assert!(sent.subject.unwrap().contains("Test subject"));
}
```

To send attachments first upload each attachment with the attachment controller and save the returned IDs to a variable. Then pass those IDs to the send method as the `attachments` property.

### Receive email
You can receive emails right in code and tests using the wait controller.

```rust
fn receive_email() {
    let wait_for_params = WaitForLatestEmailParams {
        inbox_id: inbox.id.to_owned(),
        timeout: Some(30_000),
        unread_only: Some(true)
    };
    let email = wait_for_controller_api::wait_for_latest_email(&configuration, wait_for_params).await.ok().unwrap();
    assert!(email.body.unwrap().contains("Hi"));
}
```

## Testing async
MailSlurp methods are async. Use tokio-test or another implementation to test with async functions.

```rust
#[tokio::test]
async fn my_test() -> color_eyre::Result<()> {
    // use color-eyre for better result reports
    color_eyre::install()?;

    // create an inbox
    let create_inbox_params = inbox_controller_api::CreateInboxParams { /* etc */ };
}
```

See [Rust examples](https://www.mailslurp.com/tags/rust) page for more help.

## More information
See the official [Rust homepage](https://www.mailslurp.com/docs/rust/) or the [getting started guide](https://www.mailslurp.com/guides/getting-started/) for more information.