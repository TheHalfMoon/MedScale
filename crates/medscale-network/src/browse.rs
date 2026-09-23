//! Governed Browse transport (Spec 080). The only code that can send a
//! Browse request to the network.
//!
//! Defense in depth against server-side request forgery:
//! 1. `validate_url` accepts only `https`, port 443, and a DNS host name
//!    (no IP literal, no userinfo);
//! 2. the live transport resolves names through `PublicOnlyResolver`, which
//!    drops every forbidden address at connect time (so a DNS answer cannot
//!    be swapped between check and connect);
//! 3. redirects are never followed here: each `Location` is returned to
//!    Core, which re-validates it as a new hop.
//!
//! Requests carry no cookies, credentials, proxy or ambient headers.

use std::fmt;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::time::Duration;

use medscale_contracts::browse::{BrowseDenyReason, URL_MAX_CHARS, is_dns_host_name};

/// A URL that passed `validate_url`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedUrl {
    /// Canonical form: `https://{host}{path}`.
    pub url: String,
    pub host: String,
    /// Path plus query, beginning with `/`.
    pub path: String,
}

/// Parses and checks a Browse URL. Fragment is dropped; userinfo, other
/// schemes and ports, and IP-literal hosts are refused.
pub fn validate_url(raw: &str) -> Result<ValidatedUrl, BrowseDenyReason> {
    let raw = raw.trim();
    if raw.is_empty() || raw.chars().count() > URL_MAX_CHARS {
        return Err(BrowseDenyReason::MalformedUrl);
    }
    if raw.chars().any(|c| c.is_whitespace() || c.is_control()) {
        return Err(BrowseDenyReason::MalformedUrl);
    }
    let lower = raw.to_ascii_lowercase();
    let rest = if lower.starts_with("https://") {
        &raw["https://".len()..]
    } else if lower.contains("://") || lower.starts_with("//") {
        return Err(BrowseDenyReason::SchemeNotHttps);
    } else {
        return Err(BrowseDenyReason::MalformedUrl);
    };
    let rest = rest.split('#').next().unwrap_or("");
    let (authority, path) = match rest.find(['/', '?']) {
        Some(i) => (&rest[..i], &rest[i..]),
        None => (rest, "/"),
    };
    if authority.contains('@') {
        return Err(BrowseDenyReason::MalformedUrl);
    }
    if authority.starts_with('[') {
        return Err(BrowseDenyReason::IpLiteralHost);
    }
    let (host, port) = match authority.rsplit_once(':') {
        Some((h, p)) => (h, Some(p)),
        None => (authority, None),
    };
    if let Some(port) = port
        && port != "443"
    {
        return Err(BrowseDenyReason::PortNotAllowed);
    }
    let host = host.to_ascii_lowercase();
    let host = host.strip_suffix('.').unwrap_or(&host).to_owned();
    if host.parse::<IpAddr>().is_ok() || looks_numeric_host(&host) {
        return Err(BrowseDenyReason::IpLiteralHost);
    }
    if !is_dns_host_name(&host) {
        return Err(BrowseDenyReason::MalformedUrl);
    }
    let path = if path.starts_with('?') {
        format!("/{path}")
    } else {
        path.to_owned()
    };
    Ok(ValidatedUrl {
        url: format!("https://{host}{path}"),
        host,
        path,
    })
}

/// Numeric host forms some resolvers accept as IPv4 (`2130706433`,
/// `0x7f.1`, `127.1`).
fn looks_numeric_host(host: &str) -> bool {
    host.split('.').all(|label| {
        !label.is_empty()
            && (label.bytes().all(|b| b.is_ascii_digit())
                || (label.starts_with("0x") && label[2..].bytes().all(|b| b.is_ascii_hexdigit())))
    })
}

/// Resolves a `Location` header against the current URL. Only absolute
/// `https://` targets and absolute paths are supported; anything else is
/// returned unchanged for `validate_url` to refuse.
#[must_use]
pub fn resolve_redirect(current: &ValidatedUrl, location: &str) -> String {
    let location = location.trim();
    if location.starts_with('/') && !location.starts_with("//") {
        format!("https://{}{}", current.host, location)
    } else {
        location.to_owned()
    }
}

/// True for every address Browse must never connect to (decision register
/// Q45): loopback, private, link-local (incl. cloud metadata), shared/CGNAT,
/// multicast, broadcast, documentation, benchmarking, reserved, unspecified,
/// IPv6 unique-local/link-local/site-local, and IPv4-mapped/compatible or
/// NAT64 forms of any of these.
#[must_use]
pub fn is_forbidden_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => forbidden_v4(v4),
        IpAddr::V6(v6) => forbidden_v6(v6),
    }
}

fn forbidden_v4(ip: Ipv4Addr) -> bool {
    let [a, b, c, _] = ip.octets();
    ip.is_unspecified()
        || ip.is_loopback()
        || ip.is_private()
        || ip.is_link_local()
        || ip.is_multicast()
        || ip.is_broadcast()
        || ip.is_documentation()
        || a == 0
        || (a == 100 && (64..=127).contains(&b))
        || (a == 192 && b == 0 && c == 0)
        || (a == 198 && (b == 18 || b == 19))
        || a >= 240
}

fn forbidden_v6(ip: Ipv6Addr) -> bool {
    if let Some(v4) = ip.to_ipv4_mapped() {
        return forbidden_v4(v4);
    }
    let s = ip.segments();
    // IPv4-compatible (::a.b.c.d, deprecated) and NAT64 (64:ff9b::/96).
    if s[..6] == [0, 0, 0, 0, 0, 0] || (s[0] == 0x64 && s[1] == 0xff9b && s[2..6] == [0, 0, 0, 0]) {
        let v4 = Ipv4Addr::new(
            (s[6] >> 8) as u8,
            (s[6] & 0xff) as u8,
            (s[7] >> 8) as u8,
            (s[7] & 0xff) as u8,
        );
        return ip.is_unspecified() || ip.is_loopback() || forbidden_v4(v4);
    }
    ip.is_unspecified()
        || ip.is_loopback()
        || ip.is_multicast()
        || (s[0] & 0xfe00) == 0xfc00 // unique local fc00::/7
        || (s[0] & 0xffc0) == 0xfe80 // link local fe80::/10
        || (s[0] & 0xffc0) == 0xfec0 // site local (deprecated)
        || (s[0] == 0x2001 && s[1] == 0x0db8) // documentation
}

/// One HTTP response as seen by Core.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrowseHttpResponse {
    pub status: u16,
    pub location: Option<String>,
    pub content_type: String,
    pub www_authenticate: bool,
    /// At most `limit + 1` bytes; more than `limit` means too large.
    pub body: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BrowseTransportError {
    /// The target resolved only to forbidden addresses.
    ForbiddenAddress,
    Timeout,
    Failed,
}

/// The Browse egress seam. Core owns policy; the transport only performs a
/// single GET of an already validated URL, without following redirects.
pub trait BrowseTransport: Send + Sync {
    fn get(
        &self,
        url: &ValidatedUrl,
        max_body_bytes: usize,
        timeout: Duration,
    ) -> Result<BrowseHttpResponse, BrowseTransportError>;
}

/// Live transport over `ureq` with the public-only resolver.
#[derive(Debug, Default)]
pub struct UreqBrowseTransport;

/// Wraps the default resolver and drops forbidden addresses.
#[derive(Debug, Default)]
pub struct PublicOnlyResolver {
    inner: ureq::unversioned::resolver::DefaultResolver,
}

impl ureq::unversioned::resolver::Resolver for PublicOnlyResolver {
    fn resolve(
        &self,
        uri: &ureq::http::Uri,
        config: &ureq::config::Config,
        timeout: ureq::unversioned::transport::NextTimeout,
    ) -> Result<ureq::unversioned::resolver::ResolvedSocketAddrs, ureq::Error> {
        let all = self.inner.resolve(uri, config, timeout)?;
        let mut kept = self.empty();
        for addr in all.iter().filter(|a| !is_forbidden_ip(a.ip())) {
            kept.push(*addr);
        }
        if kept.is_empty() {
            return Err(ureq::Error::HostNotFound);
        }
        Ok(kept)
    }
}

impl UreqBrowseTransport {
    fn agent(timeout: Duration) -> ureq::Agent {
        let config = ureq::Agent::config_builder()
            .max_redirects(0)
            .http_status_as_error(false)
            .https_only(true)
            .proxy(None)
            .timeout_global(Some(timeout))
            .user_agent("MedScale-GovernedBrowse/1")
            .build();
        ureq::Agent::with_parts(
            config,
            ureq::unversioned::transport::DefaultConnector::default(),
            PublicOnlyResolver::default(),
        )
    }
}

impl BrowseTransport for UreqBrowseTransport {
    fn get(
        &self,
        url: &ValidatedUrl,
        max_body_bytes: usize,
        timeout: Duration,
    ) -> Result<BrowseHttpResponse, BrowseTransportError> {
        let agent = Self::agent(timeout);
        let mut response = match agent.get(&url.url).call() {
            Ok(r) => r,
            Err(ureq::Error::HostNotFound) => return Err(BrowseTransportError::ForbiddenAddress),
            Err(ureq::Error::Timeout(_)) => return Err(BrowseTransportError::Timeout),
            Err(_) => return Err(BrowseTransportError::Failed),
        };
        let header = |name: &str| {
            response
                .headers()
                .get(name)
                .and_then(|v| v.to_str().ok())
                .map(str::to_owned)
        };
        let status = response.status().as_u16();
        let location = header("location");
        let content_type = header("content-type").unwrap_or_default();
        let www_authenticate = header("www-authenticate").is_some();
        let limit = u64::try_from(max_body_bytes)
            .unwrap_or(u64::MAX)
            .saturating_add(1);
        let body = match response.body_mut().with_config().limit(limit).read_to_vec() {
            Ok(bytes) => bytes,
            Err(ureq::Error::BodyExceedsLimit(_)) => vec![0; max_body_bytes + 1],
            Err(ureq::Error::Timeout(_)) => return Err(BrowseTransportError::Timeout),
            Err(_) => return Err(BrowseTransportError::Failed),
        };
        Ok(BrowseHttpResponse {
            status,
            location,
            content_type,
            www_authenticate,
            body,
        })
    }
}

/// Offline scripted transport for hermetic tests and demos: no sockets.
/// Unknown URLs fail as transport errors; a scripted `ForbiddenAddress`
/// models a host whose DNS answer is private.
#[derive(Default)]
pub struct ScriptedBrowseTransport {
    routes: Vec<(String, Result<BrowseHttpResponse, BrowseTransportError>)>,
}

impl fmt::Debug for ScriptedBrowseTransport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ScriptedBrowseTransport")
            .field("routes", &self.routes.len())
            .finish()
    }
}

impl ScriptedBrowseTransport {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Scripts the answer for one canonical URL.
    #[must_use]
    pub fn route(
        mut self,
        url: &str,
        answer: Result<BrowseHttpResponse, BrowseTransportError>,
    ) -> Self {
        self.routes.push((url.to_owned(), answer));
        self
    }

    /// A 200 response with the given content type and body.
    pub fn ok(content_type: &str, body: &[u8]) -> Result<BrowseHttpResponse, BrowseTransportError> {
        Ok(BrowseHttpResponse {
            status: 200,
            location: None,
            content_type: content_type.to_owned(),
            www_authenticate: false,
            body: body.to_vec(),
        })
    }

    /// A 302 redirect.
    pub fn redirect(location: &str) -> Result<BrowseHttpResponse, BrowseTransportError> {
        Ok(BrowseHttpResponse {
            status: 302,
            location: Some(location.to_owned()),
            content_type: String::new(),
            www_authenticate: false,
            body: Vec::new(),
        })
    }
}

impl BrowseTransport for ScriptedBrowseTransport {
    fn get(
        &self,
        url: &ValidatedUrl,
        max_body_bytes: usize,
        _timeout: Duration,
    ) -> Result<BrowseHttpResponse, BrowseTransportError> {
        match self.routes.iter().find(|(u, _)| u == &url.url) {
            Some((_, Ok(response))) => {
                let mut r = response.clone();
                r.body.truncate(max_body_bytes + 1);
                Ok(r)
            }
            Some((_, Err(e))) => Err(e.clone()),
            None => Err(BrowseTransportError::Failed),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_url_accepts_only_https_dns_hosts_on_443() {
        let ok = validate_url("https://Example.ORG/a/b?x=1#frag").unwrap();
        assert_eq!(ok.url, "https://example.org/a/b?x=1");
        assert_eq!(ok.host, "example.org");
        assert_eq!(ok.path, "/a/b?x=1");
        assert_eq!(validate_url("https://example.org").unwrap().path, "/");
        assert_eq!(
            validate_url("https://example.org?q=1").unwrap().path,
            "/?q=1"
        );
        assert!(validate_url("https://example.org:443/").is_ok());
        let cases = [
            ("http://example.org/", BrowseDenyReason::SchemeNotHttps),
            ("ftp://example.org/", BrowseDenyReason::SchemeNotHttps),
            ("file:///etc/passwd", BrowseDenyReason::SchemeNotHttps),
            ("//example.org/", BrowseDenyReason::SchemeNotHttps),
            (
                "https://example.org:8443/",
                BrowseDenyReason::PortNotAllowed,
            ),
            ("https://127.0.0.1/", BrowseDenyReason::IpLiteralHost),
            (
                "https://169.254.169.254/latest/meta-data",
                BrowseDenyReason::IpLiteralHost,
            ),
            ("https://[::1]/", BrowseDenyReason::IpLiteralHost),
            ("https://2130706433/", BrowseDenyReason::IpLiteralHost),
            ("https://0x7f.1/", BrowseDenyReason::IpLiteralHost),
            ("https://127.1/", BrowseDenyReason::IpLiteralHost),
            (
                "https://user:pw@example.org/",
                BrowseDenyReason::MalformedUrl,
            ),
            ("https://localhost/", BrowseDenyReason::MalformedUrl),
            ("https://exa mple.org/", BrowseDenyReason::MalformedUrl),
            ("example.org", BrowseDenyReason::MalformedUrl),
            ("", BrowseDenyReason::MalformedUrl),
        ];
        for (url, reason) in cases {
            assert_eq!(validate_url(url), Err(reason), "{url}");
        }
    }

    #[test]
    fn forbidden_addresses_cover_private_and_special_ranges() {
        for ip in [
            "127.0.0.1",
            "10.1.2.3",
            "172.16.0.1",
            "192.168.1.1",
            "169.254.169.254",
            "100.64.0.1",
            "0.0.0.0",
            "224.0.0.1",
            "255.255.255.255",
            "192.0.2.1",
            "198.18.0.1",
            "240.0.0.1",
            "192.0.0.8",
            "::1",
            "::",
            "fc00::1",
            "fd12::1",
            "fe80::1",
            "ff02::1",
            "2001:db8::1",
            "::ffff:127.0.0.1",
            "::ffff:10.0.0.1",
            "::127.0.0.1",
            "64:ff9b::a9fe:a9fe",
        ] {
            assert!(is_forbidden_ip(ip.parse().unwrap()), "{ip}");
        }
        for ip in [
            "93.184.215.14",
            "1.1.1.1",
            "2606:4700:4700::1111",
            "::ffff:8.8.8.8",
        ] {
            assert!(!is_forbidden_ip(ip.parse().unwrap()), "{ip}");
        }
    }

    #[test]
    fn redirects_resolve_only_absolute_https_and_paths() {
        let cur = validate_url("https://example.org/a").unwrap();
        assert_eq!(resolve_redirect(&cur, "/b"), "https://example.org/b");
        assert_eq!(
            resolve_redirect(&cur, "https://other.org/c"),
            "https://other.org/c"
        );
        // Scheme-relative and relative forms go back unchanged and fail validation.
        assert!(validate_url(&resolve_redirect(&cur, "//evil.org/x")).is_err());
        assert!(validate_url(&resolve_redirect(&cur, "http://example.org/")).is_err());
    }

    #[test]
    fn scripted_transport_answers_only_scripted_urls() {
        let t = ScriptedBrowseTransport::new().route(
            "https://example.org/",
            ScriptedBrowseTransport::ok("text/plain", b"hello"),
        );
        let u = validate_url("https://example.org/").unwrap();
        let r = t.get(&u, 3, Duration::from_secs(1)).unwrap();
        assert_eq!(r.body.len(), 4, "truncated to limit + 1");
        let other = validate_url("https://other.org/").unwrap();
        assert_eq!(
            t.get(&other, 10, Duration::from_secs(1)),
            Err(BrowseTransportError::Failed)
        );
    }
}
