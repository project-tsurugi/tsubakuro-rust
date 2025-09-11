#[cfg(test)]
mod test {
    use crate::{create_connection_option, test::create_test_args};
    use tokio::test;
    use tsubakuro_rust_core::prelude::*;

    #[test]
    async fn user_password_credential() {
        let args = create_test_args();
        if let Some(credential) = args.user_password_credential() {
            let option = create_connection_option(args.endpoint(), credential).unwrap();
            let session = Session::connect(&option).await.unwrap();
            session.set_fail_on_drop_error(true);

            let has_encryption_key = session.has_encryption_key().await;
            if has_encryption_key {
                let user = session.user_name();
                assert_eq!(args.user().unwrap(), &user.unwrap());
            }

            session.close().await.unwrap();

            if has_encryption_key {
                let credential = Credential::from_user_password("not exists", None::<String>);
                let option = create_connection_option(args.endpoint(), credential).unwrap();
                let error = Session::connect(&option).await.unwrap_err();
                match error {
                    TgError::ServerError(_, _, code, _) => {
                        assert_eq!("SCD-00201", code.structured_code());
                    }
                    _ => panic!("unexpected error: {error:?}"),
                }
            }
        }
    }

    const USER_LIMIT: usize = 60;

    #[test]
    async fn user_password_credential_user_too_long() {
        let args = create_test_args();

        let credential = Credential::from_user_password("*".repeat(USER_LIMIT), None::<String>);
        let option = create_connection_option(args.endpoint(), credential).unwrap();
        match Session::connect(&option).await {
            Ok(_) => {}
            Err(error) => match error {
                TgError::ServerError(_, _, code, _) => {
                    assert_eq!("SCD-00201", code.structured_code());
                }
                _ => panic!("unexpected error: {error:?}"),
            },
        }

        let credential = Credential::from_user_password("*".repeat(USER_LIMIT + 1), None::<String>);
        let option = create_connection_option(args.endpoint(), credential).unwrap();
        let error = Session::connect(&option).await.unwrap_err();
        match error {
            TgError::ClientError(message, _) => {
                assert_eq!("user too long", message);
            }
            _ => panic!("unexpected error: {error:?}"),
        }
    }

    const PASSWORD_LIMIT: usize = 60;

    #[test]
    async fn user_password_credential_password_too_long() {
        let args = create_test_args();

        let credential =
            Credential::from_user_password("tsurugi", Some("*".repeat(PASSWORD_LIMIT)));
        let option = create_connection_option(args.endpoint(), credential).unwrap();
        match Session::connect(&option).await {
            Ok(_) => {}
            Err(error) => match error {
                TgError::ServerError(_, _, code, _) => {
                    assert_eq!("SCD-00201", code.structured_code());
                }
                _ => panic!("unexpected error: {error:?}"),
            },
        }

        let credential =
            Credential::from_user_password("tsurugi", Some("*".repeat(PASSWORD_LIMIT + 1)));
        let option = create_connection_option(args.endpoint(), credential).unwrap();
        let error = Session::connect(&option).await.unwrap_err();
        match error {
            TgError::ClientError(message, _) => {
                assert_eq!("password too long", message);
            }
            _ => panic!("unexpected error: {error:?}"),
        }
    }

    #[test]
    async fn user_password_credential_user_password_too_long() {
        let args = create_test_args();

        let credential = Credential::from_user_password(
            "*".repeat(USER_LIMIT),
            Some("*".repeat(PASSWORD_LIMIT)),
        );
        let option = create_connection_option(args.endpoint(), credential).unwrap();
        match Session::connect(&option).await {
            Ok(_) => {}
            Err(error) => match error {
                TgError::ServerError(_, _, code, _) => {
                    assert_eq!("SCD-00201", code.structured_code());
                }
                _ => panic!("unexpected error: {error:?}"),
            },
        }

        let credential = Credential::from_user_password(
            "*".repeat(USER_LIMIT + 1),
            Some("*".repeat(PASSWORD_LIMIT + 1)),
        );
        let option = create_connection_option(args.endpoint(), credential).unwrap();
        let error = Session::connect(&option).await.unwrap_err();
        match error {
            TgError::ClientError(message, _) => {
                assert_eq!("user too long", message);
            }
            _ => panic!("unexpected error: {error:?}"),
        }
    }

    #[test]
    async fn auth_token_credential() {
        let args = create_test_args();
        if let Some(credential) = args.auth_token_credential() {
            let option = create_connection_option(args.endpoint(), credential).unwrap();
            let session = Session::connect(&option).await.unwrap();
            session.set_fail_on_drop_error(true);

            session.close().await.unwrap();
        }
    }
    #[test]
    async fn file_credential() {
        let args = create_test_args();
        if let Some(credential) = args.file_credential() {
            let option = create_connection_option(args.endpoint(), credential).unwrap();
            let session = Session::connect(&option).await.unwrap();
            session.set_fail_on_drop_error(true);

            session.close().await.unwrap();
        }
    }
}
