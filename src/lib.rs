#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(test), deny(unused_crate_dependencies))]
#![deny(missing_docs, unreachable_pub)]
#![allow(clippy::needless_doctest_main)]
#![doc = include_str!("../README.md")]

/// WebTransport server methods for accepting incoming requests and handling responses.
pub mod server {
    use bytes::Buf;
    use h3::quic::Connection;

    type Error = h3::Error;

    /// [`Session`] is a WebTransport session built from a QUIC connection and its incoming and
    /// outgoing streams of data frames.
    pub struct Session<C, B>
    where
        C: Connection<B>,
        B: Buf,
    {
        connection: h3::server::Connection<C, B>,
        // TODO: include server stream
    }

    impl<C, B> Session<C, B>
    where
        C: Connection<B>,
        B: Buf,
    {
        /// Creates a new [`Session`] from an existing QUIC connection.
        pub async fn new(connection: C) -> Result<Self, Error> {
            let connection = h3::server::Connection::new(connection).await?;
            let session = Self { connection };
            tracing::trace!("WebTransport Session initialized");

            Ok(session)
        }
    }

    #[cfg(test)]
    mod tests {
        // TODO: include server-level tests
    }
}

/// WebTransport client implementation for making requests to servers that support the WebTransport
/// protocol.
pub mod client {
    // TODO: include client implementation
}
