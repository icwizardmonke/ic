mod get_tecdsa_master_public_key {
    use crate::get_tecdsa_master_public_key;
    use crate::sign::canister_threshold_sig::ecdsa::IDkgReceivers;
    use crate::sign::canister_threshold_sig::ecdsa::IDkgTranscript;
    use crate::sign::canister_threshold_sig::ecdsa::IDkgTranscriptInternal;
    use crate::sign::canister_threshold_sig::ecdsa::MasterPublicKeyExtractionError;
    use crate::sign::tests::REG_V1;
    use assert_matches::assert_matches;
    use ic_base_types::SubnetId;
    use ic_crypto_test_utils::set_of;
    use ic_types::crypto::canister_threshold_sig::idkg::IDkgMaskedTranscriptOrigin;
    use ic_types::crypto::canister_threshold_sig::idkg::IDkgTranscriptId;
    use ic_types::crypto::canister_threshold_sig::idkg::IDkgTranscriptType;
    use ic_types::crypto::canister_threshold_sig::idkg::IDkgUnmaskedTranscriptOrigin;
    use ic_types::crypto::canister_threshold_sig::MasterEcdsaPublicKey;
    use ic_types::crypto::AlgorithmId;
    use ic_types::Height;
    use ic_types::PrincipalId;
    use ic_types_test_utils::ids::NODE_1;
    use std::collections::BTreeMap;
    use strum::IntoEnumIterator;

    #[test]
    fn should_return_error_if_transcript_type_is_masked() {
        let transcript = dummy_transcript(
            IDkgTranscriptType::Masked(IDkgMaskedTranscriptOrigin::Random),
            AlgorithmId::ThresholdEcdsaSecp256k1,
            vec![],
        );

        assert_matches!(
            get_tecdsa_master_public_key(&transcript),
            Err(MasterPublicKeyExtractionError::CannotExtractFromMasked)
        );
    }

    #[test]
    fn should_return_error_if_internal_transcript_cannot_be_deserialized() {
        let transcript = dummy_transcript(
            IDkgTranscriptType::Unmasked(IDkgUnmaskedTranscriptOrigin::ReshareUnmasked(
                dummy_transcript_id(),
            )),
            AlgorithmId::ThresholdEcdsaSecp256k1,
            vec![],
        );

        assert_matches!(
            get_tecdsa_master_public_key(&transcript),
            Err(MasterPublicKeyExtractionError::SerializationError( error ))
            if error.contains("SerializationError")
        );
    }

    #[test]
    fn should_return_error_if_algorithm_id_is_invalid() {
        AlgorithmId::iter()
            .filter(|algorithm_id| *algorithm_id != AlgorithmId::ThresholdEcdsaSecp256k1)
            .for_each(|wrong_algorithm_id| {
                let transcript = dummy_transcript(
                    IDkgTranscriptType::Unmasked(IDkgUnmaskedTranscriptOrigin::ReshareUnmasked(
                        dummy_transcript_id(),
                    )),
                    wrong_algorithm_id,
                    valid_internal_transcript_raw()
                        .serialize()
                        .expect("serialization of internal transcript raw should succeed"),
                );

                assert_matches!(
                    get_tecdsa_master_public_key(&transcript),
                    Err(MasterPublicKeyExtractionError::UnsupportedAlgorithm(_))
                );
            });
    }

    #[test]
    fn should_return_master_ecdsa_public_key() {
        let transcript = dummy_transcript(
            IDkgTranscriptType::Unmasked(IDkgUnmaskedTranscriptOrigin::ReshareUnmasked(
                dummy_transcript_id(),
            )),
            AlgorithmId::ThresholdEcdsaSecp256k1,
            valid_internal_transcript_raw()
                .serialize()
                .expect("serialization of internal transcript raw should succeed"),
        );
        let expected_valid_master_ecdsa_public_key = valid_master_ecdsa_public_key();

        assert_matches!(
            get_tecdsa_master_public_key(&transcript),
            Ok(tecdsa_master_public_key)
            if tecdsa_master_public_key == expected_valid_master_ecdsa_public_key
        );
    }

    /// Retrieved from a successful execution of
    /// `ic_crypto_internal_threshold_sig_ecdsa::transcript::new`.
    const VALID_INTERNAL_TRANSCRIPT_RAW: &str =
        "a173636f6d62696e65645f636f6d6d69746d656e74a16b427953756d6d6174696f\
        6ea168506564657273656ea166706f696e7473825822010252a937b4c129d822412\
        d79f39d3626f32e7a1cf85ba1dfb01c9671d7d434003f582201025b168f9f47284b\
        ed02b26197840033de1668d53ef8f4d6928b61cc7efec2a838";

    fn valid_internal_transcript_raw() -> IDkgTranscriptInternal {
        IDkgTranscriptInternal::deserialize(
            &hex::decode(VALID_INTERNAL_TRANSCRIPT_RAW)
                .expect("hex decoding of valid internal transcript raw should succeed"),
        )
        .expect("deserialization of valid internal transcript raw bytes should succeed")
    }

    fn valid_master_ecdsa_public_key() -> MasterEcdsaPublicKey {
        MasterEcdsaPublicKey {
            algorithm_id: AlgorithmId::EcdsaSecp256k1,
            public_key: hex::decode(
                "0252a937b4c129d822412d79f39d3626f32e7a1cf85ba1dfb01c9671d7d434003f",
            )
            .expect("hex decoding of public key bytes should succeed"),
        }
    }

    fn dummy_transcript_id() -> IDkgTranscriptId {
        IDkgTranscriptId::new(
            SubnetId::from(PrincipalId::new_subnet_test_id(42)),
            0,
            Height::new(0),
        )
    }

    fn dummy_transcript(
        transcript_type: IDkgTranscriptType,
        algorithm_id: AlgorithmId,
        internal_transcript_raw: Vec<u8>,
    ) -> IDkgTranscript {
        IDkgTranscript {
            verified_dealings: BTreeMap::new(),
            transcript_id: dummy_transcript_id(),
            receivers: IDkgReceivers::new(set_of(&[NODE_1])).expect("failed to create receivers"),
            registry_version: REG_V1,
            transcript_type,
            algorithm_id,
            internal_transcript_raw,
        }
    }
}
