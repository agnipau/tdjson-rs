use {
    std::{time::Duration, env},
    tdjson::TypedClient,
    tdlib_types::{methods, types},
};

// Making Telegram bots using tdlib is of course very verbose.
// If you want to do something where you don't need the power of full tdlib
// bindings look at https://github.com/teloxide/teloxide.
fn main() {
    tdjson::set_log_verbosity_level(2);
    let mut client = TypedClient::new(Duration::from_secs(1));

    let mut me = None;

    while let Some(update) = client.next() {
        match update {
            types::Response::UpdateAuthorizationState(auth_state) => {
                match auth_state.authorization_state {
                    types::AuthorizationState::AuthorizationStateWaitTdlibParameters(_) => {
                        client
                            .send(methods::SetTdlibParameters {
                                parameters: types::TdlibParameters {
                                    api_hash: env::var("TDLIB_API_HASH").unwrap().into(),
                                    api_id: env::var("TDLIB_API_ID").unwrap().parse().unwrap(),
                                    application_version: "0.1".into(),
                                    database_directory: "tdlib/db".into(),
                                    device_model: "Unknown".into(),
                                    enable_storage_optimizer: true,
                                    files_directory: "tdlib/files".into(),
                                    ignore_file_names: true,
                                    system_language_code: "en".into(),
                                    system_version: "Unknown".into(),
                                    use_chat_info_database: true,
                                    use_message_database: true,
                                    use_file_database: true,
                                    use_secret_chats: false,
                                    use_test_dc: false,
                                },
                            })
                            .expect("Failed to set TDLib parameters");
                    }
                    types::AuthorizationState::AuthorizationStateWaitEncryptionKey(_) => client
                        .send(methods::CheckDatabaseEncryptionKey {
                            encryption_key: "".into(),
                        })
                        .expect("Failed to check db encryption key"),
                    types::AuthorizationState::AuthorizationStateWaitPhoneNumber(_) => client
                        .send(methods::CheckAuthenticationBotToken {
                            token: env::var("BOT_TOKEN").unwrap(),
                        })
                        .unwrap(),
                    types::AuthorizationState::AuthorizationStateReady(_) => {
                        println!("Authorization complete");
                        client.send(methods::GetMe {}).unwrap();
                    }
                    _ => {}
                }
            }
            types::Response::User(user) => {
                me = Some(user);
            }
            types::Response::UpdateNewMessage(msg) => {
                if let Some(ref me) = me {
                    let msg = msg.message;
                    if msg.sender_user_id == me.id {
                        continue;
                    }
                    if let types::MessageContent::MessageText(types::MessageText {
                        mut text,
                        web_page: _,
                    }) = msg.content
                    {
                        text.text = text.text.chars().rev().collect();

                        let input_message_content =
                            types::InputMessageContent::InputMessageText(types::InputMessageText {
                                clear_draft: true,
                                disable_web_page_preview: false,
                                text,
                            });
                        let req = serde_json::json!({
                            "@type": "sendMessage",
                            "chat_id": msg.chat_id,
                            "reply_to_message_id": msg.id,
                            "input_message_content": input_message_content,
                        });

                        client
                            .send_untyped(&serde_json::to_string(&req).unwrap())
                            .unwrap();
                    }
                }
            }
            _ => {}
        }
    }
}
