/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is dual-licensed under either the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree or the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree. You may select, at your option, one of the
 * above-listed licenses.
 */

use allocative::Allocative;
use sorted_vector_map::SortedVectorMap;

use crate::interpreter::rule_defs::cmd_args::CommandLineBuilder;

/// A command line's expansion, suitable to actually run it.
pub struct ExpandedCommandLine {
    pub exe: Vec<String>,
    pub args: Vec<String>,
    pub env: SortedVectorMap<String, String>,
}

/// The digest of an ExpandedCommandLine.
#[derive(Eq, PartialEq, Debug, Allocative)]
pub struct ExpandedCommandLineDigest(
    // This is OK to skip because hash is stored inline.
    #[allocative(skip)] blake3::Hash,
);

impl ExpandedCommandLineDigest {
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

/// Obtain a hash of this command line. Conceptually this is as if we serialized the command
/// line to a length-suffixed list then hashed it, except we never actually produce the
/// serialized representation.
pub struct ExpandedCommandLineFingerprinter {
    digest: blake3::Hasher,
    count: u64,
}

impl ExpandedCommandLineFingerprinter {
    pub fn new() -> Self {
        Self {
            digest: blake3::Hasher::new(),
            count: 0,
        }
    }

    pub fn push_count(&mut self) {
        self.digest.update(self.count.to_le_bytes().as_slice());
        self.count = 0;
    }

    pub fn finalize(self) -> ExpandedCommandLineDigest {
        if self.count > 0 {
            panic!("Called finalize without adding length of all elements");
        }

        ExpandedCommandLineDigest(self.digest.finalize())
    }
}

impl CommandLineBuilder for ExpandedCommandLineFingerprinter {
    fn push_arg(&mut self, s: String) {
        self.count += 1;

        let bytes = s.as_bytes();
        self.digest.update(bytes);
        self.digest.update(bytes.len().to_le_bytes().as_slice());
    }
}

#[cfg(test)]
mod tests {
    use sorted_vector_map::sorted_vector_map;

    use super::*;

    impl ExpandedCommandLine {
        pub fn fingerprint(&self) -> ExpandedCommandLineDigest {
            let mut fingerprinter = ExpandedCommandLineFingerprinter::new();
            for e in self.exe.iter() {
                fingerprinter.push_arg(e.to_owned());
            }
            fingerprinter.push_count();

            for e in self.args.iter() {
                fingerprinter.push_arg(e.to_owned());
            }
            fingerprinter.push_count();

            for (k, v) in self.env.iter() {
                fingerprinter.push_arg(k.to_owned());
                fingerprinter.push_arg(v.to_owned());
            }

            fingerprinter.push_count();
            fingerprinter.finalize()
        }
    }

    #[test]
    fn test_cli() {
        let cmd1 = ExpandedCommandLine {
            exe: Default::default(),
            args: vec!["foo".to_owned(), "bar".to_owned()],
            env: Default::default(),
        };

        let cmd2 = ExpandedCommandLine {
            exe: Default::default(),
            args: vec!["foob".to_owned(), "ar".to_owned()],
            env: Default::default(),
        };

        let cmd3 = ExpandedCommandLine {
            exe: Default::default(),
            args: vec!["foobar".to_owned()],
            env: Default::default(),
        };

        let cmd4 = ExpandedCommandLine {
            exe: vec!["foo".to_owned()],
            args: vec!["bar".to_owned()],
            env: Default::default(),
        };

        assert_ne!(cmd1.fingerprint(), cmd2.fingerprint());
        assert_ne!(cmd1.fingerprint(), cmd3.fingerprint());
        assert_ne!(cmd2.fingerprint(), cmd3.fingerprint());
        assert_ne!(cmd3.fingerprint(), cmd4.fingerprint());
    }

    #[test]
    fn test_env() {
        let cmd1 = ExpandedCommandLine {
            exe: Default::default(),
            args: Default::default(),
            env: sorted_vector_map! { "FOO".to_owned() => "BAR".to_owned() },
        };

        let cmd2 = ExpandedCommandLine {
            exe: Default::default(),
            args: Default::default(),
            env: sorted_vector_map! { "FOO2".to_owned()=> "BAR".to_owned() },
        };

        let cmd3 = ExpandedCommandLine {
            exe: Default::default(),
            args: Default::default(),
            env: sorted_vector_map! { "FOO".to_owned()=> "BAR2".to_owned() },
        };

        assert_ne!(cmd1.fingerprint(), cmd2.fingerprint());
        assert_ne!(cmd1.fingerprint(), cmd3.fingerprint());
        assert_ne!(cmd2.fingerprint(), cmd3.fingerprint());
    }

    #[test]
    fn test_hash_stability() {
        fn env() -> SortedVectorMap<String, String> {
            sorted_vector_map! { "FOO1".to_owned() => "BAR1".to_owned(), "FOO2".to_owned() => "BAR2".to_owned() }
        }

        let mut cmd = ExpandedCommandLine {
            exe: Default::default(),
            args: vec!["cmd".to_owned()],
            env: env(),
        };

        let digest = cmd.fingerprint();

        for _ in 0..10 {
            cmd.env = env();
            assert_eq!(cmd.fingerprint(), digest);
        }
    }
}
