use std::future::Future;
use std::net::{SocketAddr, IpAddr};
use std::pin::Pin;
use std::str::FromStr;

use actix_web::dev::Payload;
use actix_web::web::Data;
use actix_web::{FromRequest, HttpRequest};

use ::hcaptcha::{HcaptchaCaptcha, HcaptchaClient, HcaptchaRequest};

use crate::error::ApiError;

pub struct Hcaptcha {
    _private: (),
}

#[derive(thiserror::Error, Debug)]
pub enum HcaptchaError {
    #[error("missing hCaptcha challenge response header")]
    Missing,
    #[error("insufficient information to verify hCaptcha challenge")]
    InsufficientInformation,
    #[error("invalid hCaptcha challenge response header")]
    Invalid(#[from] ::hcaptcha::HcaptchaError),
}

impl Hcaptcha {
    fn create_request(req: &HttpRequest) -> Result<HcaptchaRequest, HcaptchaError> {
        let response = req
            .headers()
            .get("X-CAPTCHA-KEY")
            .and_then(|key| key.to_str().ok())
            .ok_or(HcaptchaError::Missing)?;
        let connection_info = req.connection_info();
        let real_ip = connection_info.realip_remote_addr().unwrap();
        let user_ip =  if real_ip.contains(':') {
            SocketAddr::from_str(real_ip)
                .map(|sa| sa.ip())
        } else {
            IpAddr::from_str(real_ip)
        }
            .map_err(|e| {
                error!("{}", e);
                HcaptchaError::InsufficientInformation
            })?;
        info!("real_ip:{}, parsed: {}", real_ip, user_ip);
        let config = req.app_data::<Data<crate::config::HCaptcha>>().unwrap();
        let captcha = HcaptchaCaptcha::new(response)?;
        let hc = HcaptchaRequest::new(config.secret.as_str(), captcha)?
            .set_sitekey(config.site_key.as_str())?
            .set_remoteip(&*user_ip.to_string())?;
        Ok(hc)
    }
}

impl FromRequest for Hcaptcha {
    type Config = ();
    type Error = ApiError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        if cfg!(feature = "bypass-hcaptcha") {
            return Box::pin(async { Ok(Hcaptcha { _private: () }) });
        }

        let hc = Self::create_request(req);
        if hc.is_err() {
            return Box::pin(async { Err(hc.unwrap_err().into()) })
        }

        Box::pin(async move {
            let client = HcaptchaClient::new();
            client.verify_client_response(hc.unwrap())
                .await
                .map(|_| Hcaptcha { _private: () })
                .map_err(|e| {
                    error!("Hcaptcha failed: {:?}", e);
                    HcaptchaError::Invalid(e).into()
                })
        })
    }
}