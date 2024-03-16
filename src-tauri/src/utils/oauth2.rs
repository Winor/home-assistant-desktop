// use anyhow::{self, Error};
// use oauth2::basic::BasicClient;
// use oauth2::url::Url;
// use oauth2::{
//     AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
//     RequestTokenError, Scope, TokenResponse, TokenUrl,
// };

// type MyClient = oauth2::Client<
//     oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
//     oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>,
//     oauth2::basic::BasicTokenType,
//     oauth2::StandardTokenIntrospectionResponse<
//         oauth2::EmptyExtraTokenFields,
//         oauth2::basic::BasicTokenType,
//     >,
//     oauth2::StandardRevocableToken,
//     oauth2::StandardErrorResponse<oauth2::RevocationErrorResponseType>,
// >;
// // use oauth2::reqwest::async_http_client;
// // use tokio::net::TcpListener;
// struct Oauth {
//     client: MyClient,
//     redirect: String,
//     url: String
// }

// impl Oauth {
//     fn get_url(&self) -> Url {
//         let (auth_url, _csrf_state) = self.client
//             .authorize_url(CsrfToken::new_random)
//             .add_extra_param("client_id", &self.redirect)
//             .add_extra_param("redirect_uri", &self.redirect)
//             .url();
//         // This is the URL you should redirect the user to, in order to trigger the authorization
//         auth_url
//     }

//     fn get_token(&self) {
        
//     }
// }

// pub fn create_client(url: String, redirect: String) -> Result<Oauth, Error> {
//     let client = BasicClient::new(
//         ClientId::new(url.to_owned()),
//         Some(ClientSecret::new("client_secret".to_string())),
//         AuthUrl::new(format!("{}/auth/authorize", url))?,
//         Some(TokenUrl::new(format!("{}/auth/token", url))?),
//     )
//     // Set the URL the user will be redirected to after the authorization process.
//     .set_redirect_uri(RedirectUrl::new(redirect.to_owned())?);
//     return Ok(Oauth { client: client, redirect: redirect, url: url });
// }