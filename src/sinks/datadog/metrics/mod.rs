use std::sync::OnceLock;

use crate::sinks::util::Compression;

mod config;
mod encoder;
mod normalizer;
mod request_builder;
mod service;
mod sink;

#[cfg(all(test, feature = "datadog-metrics-integration-tests"))]
mod integration_tests;
#[cfg(test)]
mod tests;

pub use self::config::DatadogMetricsConfig;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum DatadogMetricsCompression {
    Deflate,
    Zstd,
}

impl DatadogMetricsCompression {
    pub(super) const fn content_encoding(self) -> &'static str {
        match self {
            Self::Deflate => "deflate",
            Self::Zstd => "zstd",
        }
    }

    pub(super) const fn as_compression(self) -> Compression {
        match self {
            Self::Deflate => Compression::zlib_default(),
            Self::Zstd => Compression::zstd_default(),
        }
    }
}

pub(super) fn request_compression() -> DatadogMetricsCompression {
    static COMPRESSION: OnceLock<DatadogMetricsCompression> = OnceLock::new();
    *COMPRESSION.get_or_init(|| {
        match std::env::var("VECTOR_DATADOG_METRICS_COMPRESSION")
            .ok()
            .map(|value| value.to_ascii_lowercase())
        {
            None => DatadogMetricsCompression::Zstd,
            Some(value) => match value.as_str() {
                "deflate" => DatadogMetricsCompression::Deflate,
                "zstd" => DatadogMetricsCompression::Zstd,
                _ => {
                    warn!(
                        message = "Invalid Datadog metrics compression value, falling back to zstd.",
                        value,
                        expected = "deflate|zstd",
                        internal_log_rate_limit = false
                    );
                    DatadogMetricsCompression::Zstd
                }
            },
        }
    })
}

