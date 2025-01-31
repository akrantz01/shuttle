use crossterm::style::Color;

use crate::helpers::{self, APPS_FQDN};

// TODO: remove ignore when updated examples are merged: https://github.com/shuttle-hq/examples/pull/4
#[ignore]
#[test]
fn hello_world_actix_web() {
    let client = helpers::Services::new_docker(
        "hello-world (actix-web)",
        "actix-web/hello-world",
        Color::Green,
    );
    client.deploy();

    let request_text = client
        .get("hello")
        .header("Host", format!("hello-world-actix-web-app.{}", *APPS_FQDN))
        .send()
        .unwrap()
        .text()
        .unwrap();

    assert_eq!(request_text, "Hello World!");
}
