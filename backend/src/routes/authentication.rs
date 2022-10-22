use argon2::{self, Config};
use rand::Rng;
use warp::http::StatusCode;

use crate::error::Error;
use crate::store::Store;
use crate::types::account::{Account, AccountID};

pub async fn register(store: Store, account: Account) -> Result<impl warp::Reply, warp::Rejection> {
    // take the password as a byte array and pass it to our hash function
    let hashed_password = hash(account.password.as_bytes());

    let account = Account {
        id: account.id,
        email: account.email,
        password: hashed_password,
    };

    match store.add_account(account).await {
        Ok(_) => Ok(warp::reply::with_status("Account added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub fn hash(password: &[u8]) -> String {
    // create 32 random bytes and store them in a slice
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

pub async fn login(store: Store, login: Account) -> Result<impl warp::Reply, warp::Rejection> {
    // Check if the user exists in our DB
    match store.get_account(login.email).await {
        Ok(account) => match verify_password(&account.password, login.password.as_bytes()) {
            Ok(verified) => {
                if verified {
                    // if account exists and is verified, create a token with AccountID in it
                    Ok(warp::reply::json(&issue_token(
                        account.id.expect("id not found"),
                    )))
                } else {
                    Err(warp::reject::custom(Error::WrongPassword))
                }
            }
            // if hashing library fails, send a 500 code back to the user
            Err(e) => Err(warp::reject::custom(Error::ArgonLibraryError(e))),
        },
        Err(e) => Err(warp::reject::custom(e)),
    }
}

fn verify_password(hash: &str, password: &[u8]) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(hash, password)
}

fn issue_token(account_id: AccountID) -> String {
    let state = serde_json::to_string(&account_id).expect("Failed to serialize state");
    // Issua a token which puts the AccountID into a string and packs it the paseto token
    local_paseto(&state, None, "RANDOM WORDS WINTER MACINTOSH PC".as_bytes())
        .expect("Failed to create token")
}
