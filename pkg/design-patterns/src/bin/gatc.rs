// SPDX-License-Identifier: GPL-3.0
// https://rust-unofficial.github.io/patterns/functional/generics-type-classes.html

use std::path::{Path, PathBuf};

use util::ProtoKind;
pub use util::{Bootp, Nfs};

enum AuthInfo {
    Nfs(nfs::AuthInfo),
    Bootp(bootp::AuthInfo),
}

struct FileDownloadRequest<P>
where
    P: ProtoKind,
{
    file_name: PathBuf,
    protocol: P,
}

// all common API parts go into a generic impl block
impl<P> FileDownloadRequest<P>
where
    P: ProtoKind,
{
    fn file_path(&self) -> &Path {
        &self.file_name
    }

    fn auth_info(&self) -> P::AuthInfo {
        self.protocol.auth_info()
    }
}

// all protocol-specific impls go into their own block
impl FileDownloadRequest<Nfs> {
    // ... other methods

    /// Gets an NFS mount point if this is an NFS request. Otherwise,
    /// return None.
    pub fn mount_point(&self) -> &Path {
        self.protocol.mount_point()
    }
}

fn main() {
    // let mut socket = bootp::listen()?;
    // while let Some(request) = socket.next_request()? {
    //     match request.mount_point().as_ref() {
    //         "/secure" => socket.send("Access denied"),
    //         _ => {} // continue on...
    //     }
    //     // Rest of the code here
    // }
}

mod bootp {
    use std::path::PathBuf;

    use crate::FileDownloadRequest;

    pub struct AuthInfo(); // no authentication in bootp

    pub fn listen() -> Result<(), ()> {
        todo!()
    }
}

mod nfs {
    #[derive(Clone)]
    pub struct AuthInfo(String); // NFS session management omitted
}

mod util {
    use std::path::{Path, PathBuf};

    use crate::{bootp, nfs};

    pub trait ProtoKind {
        type AuthInfo;
        fn auth_info(&self) -> Self::AuthInfo;
    }

    pub struct Nfs {
        auth: nfs::AuthInfo,
        mount_point: PathBuf,
    }

    impl Nfs {
        pub fn mount_point(&self) -> &Path {
            &self.mount_point
        }
    }

    impl ProtoKind for Nfs {
        type AuthInfo = nfs::AuthInfo;

        fn auth_info(&self) -> Self::AuthInfo {
            self.auth.clone()
        }
    }

    pub struct Bootp(); // no additional metadata

    impl ProtoKind for Bootp {
        type AuthInfo = bootp::AuthInfo;

        fn auth_info(&self) -> Self::AuthInfo {
            bootp::AuthInfo()
        }
    }
}
