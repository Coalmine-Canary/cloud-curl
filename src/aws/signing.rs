use std::time::SystemTime;

use aws_sigv4::http_request::{sign, SigningSettings, SigningParams, SignableRequest};
use hyper::Request;

use crate::config::ENV;

use super::credentials::{Credentials, Profile};

pub fn sign_request(request: Request<String> ) -> Result<Request<String>, String> {
  let mut request = request;
  let profile = match &ENV.AWS_PROFILE {
      Some(p) => p.clone(),
      None => "default".into()
  };

  let credentials = Credentials::get(&profile)?;
  let profile = Profile::get(&profile)?;

  let signing_settings = SigningSettings::default();
  let token = credentials.security_token.as_ref().unwrap_or(&String::from("")).clone(); // Necessary because it does not live long enough in `if`
  let signing_params = if credentials.security_token.is_some() {
    
    SigningParams::builder()
      .access_key(&credentials.access_key_id.as_str())
      .secret_key(&credentials.secret_key.as_str())
      .security_token(&token)
      .region(profile.region.as_str())
      .service_name("es")
      .time(SystemTime::now())
      .settings(signing_settings)
      .build()
      .unwrap()
  } else {
    SigningParams::builder()
      .access_key(&credentials.access_key_id.as_str())
      .secret_key(&credentials.secret_key.as_str())
      .region(profile.region.as_str())
      .service_name("es")
      .time(SystemTime::now())
      .settings(signing_settings)
      .build()
      .unwrap()
  };
   
  // Convert the HTTP request into a signable request
  let signable_request = SignableRequest::from(&request);

  // Sign and then apply the signature to the request
  let (signing_instructions, _signature) = sign(signable_request, &signing_params).unwrap().into_parts();
  signing_instructions.apply_to_request(&mut request);

  Ok(request)
}

// pub fn get_credentials() -> Result<(String, String), String> {
//     // Checks environment variables and aws credentials file -- will be the place to check cloud provider in use in the future

//     let env: &Lazy<config::Env> = &config::ENV;

//     if env.ACCESS_KEY.is_some() && env.SECRET_KEY.is_some() {
//         return Ok((env.ACCESS_KEY.clone().unwrap(), env.SECRET_KEY.clone().unwrap()))
//     }

//     let profile = match &env.AWS_PROFILE {
//         Some(p) => p.clone(),
//         None => "default".into()
//     };

//     let credentials = credentials::Credentials::get(&profile)?;
//     return Ok((credentials.access_key_id.clone(), credentials.secret_key.clone()))
// }