//
// SPDX-License-Identifier: Apache-2.0
//
// Copyright (C) 2021 Shun Sakai
//

use assert_cmd::Command;
use predicates::prelude::*;

fn command() -> Command {
    let mut command = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    command.current_dir("tests/");

    command
}

#[test]
fn cbor2cbor() {
    command()
        .arg("-f")
        .arg("cbor")
        .arg("-t")
        .arg("cbor")
        .arg("resource/sample.cbor")
        .assert()
        .stdout(predicate::eq(
            include_bytes!("resource/sample.cbor") as &[u8]
        ));
}

#[test]
fn cbor2json() {
    command()
        .arg("-f")
        .arg("cbor")
        .arg("-t")
        .arg("json")
        .arg("resource/sample.cbor")
        .assert()
        .stdout(predicate::eq(include_str!("resource/sample.json")));
}

#[test]
fn cbor2messagepack() {
    command()
        .arg("-f")
        .arg("cbor")
        .arg("-t")
        .arg("messagepack")
        .arg("resource/sample.cbor")
        .assert()
        .stdout(predicate::eq(
            include_bytes!("resource/sample.msgpack") as &[u8]
        ));
}

#[test]
fn cbor2toml() {
    command()
        .arg("-f")
        .arg("cbor")
        .arg("-t")
        .arg("toml")
        .arg("resource/sample.cbor")
        .assert()
        .stdout(predicate::eq(include_str!("resource/sample.toml")));
}

#[test]
fn cbor2yaml() {
    command()
        .arg("-f")
        .arg("cbor")
        .arg("-t")
        .arg("yaml")
        .arg("resource/sample.cbor")
        .assert()
        .stdout(predicate::eq(include_str!("resource/sample.yaml")));
}

#[test]
fn json2cbor() {
    command()
        .arg("-f")
        .arg("json")
        .arg("-t")
        .arg("cbor")
        .arg("resource/sample.json")
        .assert()
        .stdout(predicate::eq(
            include_bytes!("resource/sample.cbor") as &[u8]
        ));
}

#[test]
fn json2json() {
    command()
        .arg("-f")
        .arg("json")
        .arg("-t")
        .arg("json")
        .arg("resource/sample.json")
        .assert()
        .stdout(predicate::eq(include_str!("resource/sample.json")));
}

#[test]
fn json2messagepack() {
    command()
        .arg("-f")
        .arg("json")
        .arg("-t")
        .arg("messagepack")
        .arg("resource/sample.json")
        .assert()
        .stdout(predicate::eq(
            include_bytes!("resource/sample.msgpack") as &[u8]
        ));
}

#[test]
fn json2toml() {
    command()
        .arg("-f")
        .arg("json")
        .arg("-t")
        .arg("toml")
        .arg("resource/sample.json")
        .assert()
        .stdout(predicate::eq(include_str!("resource/sample.toml")));
}

#[test]
fn json2yaml() {
    command()
        .arg("-f")
        .arg("json")
        .arg("-t")
        .arg("yaml")
        .arg("resource/sample.json")
        .assert()
        .stdout(predicate::eq(include_str!("resource/sample.yaml")));
}

#[test]
fn messagepack2cbor() {
    command()
        .arg("-f")
        .arg("messagepack")
        .arg("-t")
        .arg("cbor")
        .arg("resource/sample.msgpack")
        .assert()
        .stdout(predicate::eq(
            include_bytes!("resource/sample.cbor") as &[u8]
        ));
}

#[test]
fn messagepack2json() {
    command()
        .arg("-f")
        .arg("messagepack")
        .arg("-t")
        .arg("json")
        .arg("resource/sample.msgpack")
        .assert()
        .stdout(predicate::eq(include_str!("resource/sample.json")));
}

#[test]
fn messagepack2messagepack() {
    command()
        .arg("-f")
        .arg("messagepack")
        .arg("-t")
        .arg("messagepack")
        .arg("resource/sample.msgpack")
        .assert()
        .stdout(predicate::eq(
            include_bytes!("resource/sample.msgpack") as &[u8]
        ));
}

#[test]
fn messagepack2toml() {
    command()
        .arg("-f")
        .arg("messagepack")
        .arg("-t")
        .arg("toml")
        .arg("resource/sample.msgpack")
        .assert()
        .stdout(predicate::eq(include_str!("resource/sample.toml")));
}

#[test]
fn messagepack2yaml() {
    command()
        .arg("-f")
        .arg("messagepack")
        .arg("-t")
        .arg("yaml")
        .arg("resource/sample.msgpack")
        .assert()
        .stdout(predicate::eq(include_str!("resource/sample.yaml")));
}

#[test]
fn toml2cbor() {
    command()
        .arg("-f")
        .arg("toml")
        .arg("-t")
        .arg("cbor")
        .arg("resource/sample.toml")
        .assert()
        .stdout(predicate::eq(
            include_bytes!("resource/sample.cbor") as &[u8]
        ));
}

#[test]
fn toml2json() {
    command()
        .arg("-f")
        .arg("toml")
        .arg("-t")
        .arg("json")
        .arg("resource/sample.toml")
        .assert()
        .stdout(predicate::eq(include_str!("resource/sample.json")));
}

#[test]
fn toml2messagepack() {
    command()
        .arg("-f")
        .arg("toml")
        .arg("-t")
        .arg("messagepack")
        .arg("resource/sample.toml")
        .assert()
        .stdout(predicate::eq(
            include_bytes!("resource/sample.msgpack") as &[u8]
        ));
}

#[test]
fn toml2toml() {
    command()
        .arg("-f")
        .arg("toml")
        .arg("-t")
        .arg("toml")
        .arg("resource/sample.toml")
        .assert()
        .stdout(predicate::eq(include_str!("resource/sample.toml")));
}

#[test]
fn toml2yaml() {
    command()
        .arg("-f")
        .arg("toml")
        .arg("-t")
        .arg("yaml")
        .arg("resource/sample.toml")
        .assert()
        .stdout(predicate::eq(include_str!("resource/sample.yaml")));
}

#[test]
fn yaml2cbor() {
    command()
        .arg("-f")
        .arg("yaml")
        .arg("-t")
        .arg("cbor")
        .arg("resource/sample.yaml")
        .assert()
        .stdout(predicate::eq(
            include_bytes!("resource/sample.cbor") as &[u8]
        ));
}

#[test]
fn yaml2json() {
    command()
        .arg("-f")
        .arg("yaml")
        .arg("-t")
        .arg("json")
        .arg("resource/sample.yaml")
        .assert()
        .stdout(predicate::eq(include_str!("resource/sample.json")));
}

#[test]
fn yaml2messagepack() {
    command()
        .arg("-f")
        .arg("yaml")
        .arg("-t")
        .arg("messagepack")
        .arg("resource/sample.yaml")
        .assert()
        .stdout(predicate::eq(
            include_bytes!("resource/sample.msgpack") as &[u8]
        ));
}

#[test]
fn yaml2toml() {
    command()
        .arg("-f")
        .arg("yaml")
        .arg("-t")
        .arg("toml")
        .arg("resource/sample.yaml")
        .assert()
        .stdout(predicate::eq(include_str!("resource/sample.toml")));
}

#[test]
fn yaml2yaml() {
    command()
        .arg("-f")
        .arg("yaml")
        .arg("-t")
        .arg("yaml")
        .arg("resource/sample.yaml")
        .assert()
        .stdout(predicate::eq(include_str!("resource/sample.yaml")));
}
