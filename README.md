# nocaptcha
Anti-captcha API wrapper built in Rust.

Currently under development.

## Example
```rust
use nocaptcha::captcha::Anticaptcha;
use nocaptcha::tasks::RecaptchaV2Task;

#[tokio::test]
async fn test_recaptcha_v2() -> Result<(), Box<dyn std::error::Error>> {
    let mykey = String::from("Your-Key");

    let anticaptcha_client = Anticaptcha::new(mykey);
    let mut google_recaptcha_task = RecaptchaV2Task::new(
        "https://example.com/recaptcha",
        "6LfD3PIbAAAAAJs_eEHvoOl75_83eXSqpPSRFJ_u",
    )
    .set_invisible(false);
    .set_sdata("secret-extra-data-key");

    anticaptcha_client
        .create_task(&mut google_recaptcha_task)
        .await?;

    let task_result = anticaptcha_client.wait_for(&google_recaptcha_task).await?;
    assert_eq!(task_result.status, String::from("ready"));
    Ok(())
}
```
