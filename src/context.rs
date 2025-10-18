use std::collections::HashMap;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct RequestContext {
    inner: HashMap<String, Value>,
}

pub const ATTR_USER_ID: &str = "user.id";
pub const ATTR_USER_EMAIL: &str = "user.email";
pub const ATTR_USER_ANONYMOUS: &str = "user.anonymous";
pub const ATTR_COUNTRY_CODE: &str = "country_code";
pub const ATTR_REGION: &str = "region";
pub const ATTR_CITY: &str = "city";
pub const ATTR_MANUFACTURER: &str = "manufacturer";
pub const ATTR_DEVICE_TYPE: &str = "device_type";
pub const ATTR_OS: &str = "os";
pub const ATTR_OS_VERSION: &str = "os_version";
pub const ATTR_BROWSER: &str = "browser";
pub const ATTR_BROWSER_VERSION: &str = "browser_version";
pub const ATTR_LANGUAGE: &str = "language";
pub const ATTR_CONNECTION_TYPE: &str = "connection_type";
pub const ATTR_AGE: &str = "age";
pub const ATTR_GENDER: &str = "gender";
pub const ATTR_IP: &str = "ip";
pub const ATTR_APP_VERSION: &str = "app_version";
pub const ATTR_PLATFORM: &str = "platform";

impl RequestContext {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn with_user_id(mut self, id: &str) -> Self {
        self.inner.insert(ATTR_USER_ID.to_string(), Value::String(id.to_string()));
        self
    }

    pub fn with_user_email(mut self, email: &str) -> Self {
        self.inner.insert(ATTR_USER_EMAIL.to_string(), Value::String(email.to_string()));
        self
    }

    pub fn with_user_anonymous(mut self, anonymous: bool) -> Self {
        self.inner.insert(ATTR_USER_ANONYMOUS.to_string(), Value::Bool(anonymous));
        self
    }

    pub fn with_country_code(mut self, code: &str) -> Self {
        self.inner.insert(ATTR_COUNTRY_CODE.to_string(), Value::String(code.to_string()));
        self
    }

    pub fn with_region(mut self, region: &str) -> Self {
        self.inner.insert(ATTR_REGION.to_string(), Value::String(region.to_string()));
        self
    }

    pub fn with_city(mut self, city: &str) -> Self {
        self.inner.insert(ATTR_CITY.to_string(), Value::String(city.to_string()));
        self
    }

    pub fn with_manufacturer(mut self, manufacturer: &str) -> Self {
        self.inner.insert(ATTR_MANUFACTURER.to_string(), Value::String(manufacturer.to_string()));
        self
    }

    pub fn with_device_type(mut self, device_type: &str) -> Self {
        self.inner.insert(ATTR_DEVICE_TYPE.to_string(), Value::String(device_type.to_string()));
        self
    }

    pub fn with_os(mut self, os: &str) -> Self {
        self.inner.insert(ATTR_OS.to_string(), Value::String(os.to_string()));
        self
    }

    pub fn with_os_version(mut self, version: &str) -> Self {
        self.inner.insert(ATTR_OS_VERSION.to_string(), Value::String(version.to_string()));
        self
    }

    pub fn with_browser(mut self, browser: &str) -> Self {
        self.inner.insert(ATTR_BROWSER.to_string(), Value::String(browser.to_string()));
        self
    }

    pub fn with_browser_version(mut self, version: &str) -> Self {
        self.inner.insert(ATTR_BROWSER_VERSION.to_string(), Value::String(version.to_string()));
        self
    }

    pub fn with_language(mut self, language: &str) -> Self {
        self.inner.insert(ATTR_LANGUAGE.to_string(), Value::String(language.to_string()));
        self
    }

    pub fn with_connection_type(mut self, connection_type: &str) -> Self {
        self.inner.insert(ATTR_CONNECTION_TYPE.to_string(), Value::String(connection_type.to_string()));
        self
    }

    pub fn with_age(mut self, age: i32) -> Self {
        self.inner.insert(ATTR_AGE.to_string(), Value::Number(age.into()));
        self
    }

    pub fn with_gender(mut self, gender: &str) -> Self {
        self.inner.insert(ATTR_GENDER.to_string(), Value::String(gender.to_string()));
        self
    }

    pub fn with_ip(mut self, ip: &str) -> Self {
        self.inner.insert(ATTR_IP.to_string(), Value::String(ip.to_string()));
        self
    }

    pub fn with_app_version(mut self, version: &str) -> Self {
        self.inner.insert(ATTR_APP_VERSION.to_string(), Value::String(version.to_string()));
        self
    }

    pub fn with_platform(mut self, platform: &str) -> Self {
        self.inner.insert(ATTR_PLATFORM.to_string(), Value::String(platform.to_string()));
        self
    }

    pub fn set(mut self, key: &str, value: Value) -> Self {
        self.inner.insert(key.to_string(), value);
        self
    }

    pub fn set_many(mut self, values: HashMap<String, Value>) -> Self {
        for (key, value) in values {
            self.inner.insert(key, value);
        }
        self
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn to_map(&self) -> HashMap<String, Value> {
        self.inner.clone()
    }
}

impl RequestContext {
    pub fn default() -> Self {
        Self::new()
    }
}
