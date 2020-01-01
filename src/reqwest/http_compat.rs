pub(crate) trait HttpCompat<T> {
    fn compat(self) -> T;
}


#[cfg(feature = "http-01")]
impl HttpCompat<http_0_1::method::Method> for http::method::Method {
    fn compat(self) -> http_0_1::method::Method {
        use std::str::FromStr;
        http_0_1::Method::from_str(self.as_str()).unwrap()
    }
}

impl HttpCompat<http::method::Method> for http::method::Method {
    fn compat(self) -> http::method::Method {
        self
    }
}

#[cfg(feature = "http-01")]
impl HttpCompat<http_0_1::status::StatusCode> for http::status::StatusCode {
    fn compat(self) -> http_0_1::status::StatusCode {
        http_0_1::StatusCode::from_u16(self.as_u16()).unwrap()
    }
}

#[cfg(feature = "http-01")]
impl HttpCompat<http::status::StatusCode> for http_0_1::status::StatusCode {
    fn compat(self) -> http::status::StatusCode {
        http::StatusCode::from_u16(self.as_u16()).unwrap()
    }
}

impl HttpCompat<http::status::StatusCode> for http::status::StatusCode {
    fn compat(self) -> http::status::StatusCode {
        self
    }
}

#[cfg(feature = "http-01")]
impl HttpCompat<http_0_1::header::HeaderName> for http::header::HeaderName {
    fn compat(self) -> http_0_1::header::HeaderName {
        use std::str::FromStr;
        http_0_1::header::HeaderName::from_str(self.as_str()).unwrap()
    }
}

#[cfg(feature = "http-01")]
impl HttpCompat<http::header::HeaderName> for http_0_1::header::HeaderName {
    fn compat(self) -> http::header::HeaderName {
        use std::str::FromStr;
        http::header::HeaderName::from_str(self.as_str()).unwrap()
    }
}

impl HttpCompat<http::header::HeaderName> for http::header::HeaderName {
    fn compat(self) -> http::header::HeaderName {
        self
    }
}

#[cfg(feature = "http-01")]
impl HttpCompat<http_0_1::header::HeaderValue> for http::header::HeaderValue {
    fn compat(self) -> http_0_1::header::HeaderValue {
        http_0_1::header::HeaderValue::from_str(self.to_str().unwrap()).unwrap()
    }
}

#[cfg(feature = "http-01")]
impl HttpCompat<http::header::HeaderValue> for http_0_1::header::HeaderValue {
    fn compat(self) -> http::header::HeaderValue {
        http::header::HeaderValue::from_str(self.to_str().unwrap()).unwrap()
    }
}

impl HttpCompat<http::header::HeaderValue> for http::header::HeaderValue {
    fn compat(self) -> http::header::HeaderValue {
        self
    }
}

#[cfg(feature = "http-01")]
impl HttpCompat<http_0_1::HeaderMap> for http::HeaderMap {
    fn compat(self) -> http_0_1::HeaderMap {
        let mut ret = http_0_1::HeaderMap::with_capacity(self.len());

        let mut header_name = http_0_1::header::HeaderName::from_static("");

        for (k, v) in self.into_iter() {
            if let Some(h) = k {
                header_name = h.compat();
            }
            ret.append(header_name.clone(), v.compat());
        }
        ret
    }
}

#[cfg(feature = "http-01")]
impl HttpCompat<http::HeaderMap> for http_0_1::HeaderMap {
    fn compat(self) -> http::HeaderMap {
        let mut ret = http::HeaderMap::with_capacity(self.len());

        let mut header_name = http::header::HeaderName::from_static("");

        for (k, v) in self.into_iter() {
            if let Some(h) = k {
                header_name = h.compat();
            }
            ret.append(header_name.clone(), v.compat());
        }
        ret
    }
}

impl HttpCompat<http::HeaderMap> for http::HeaderMap {
    fn compat(self) -> http::HeaderMap {
        self
    }
}