use std::path::Path;

/// Search paths used by Prost.
pub struct ProtoPaths<'a> {
    pub nervous_system: &'a Path,
}

impl ProtoPaths<'_> {
    fn to_vec(&self) -> Vec<&Path> {
        vec![self.nervous_system]
    }
}

const COPY_TYPE_NAMES: [&str; 3] = ["Duration", "Tokens", "Percentage"];

/// Build protos using prost_build.
pub fn generate_prost_files(proto_paths: ProtoPaths<'_>, out_dir: &Path) {
    let mut config = prost_build::Config::new();
    config.protoc_arg("--experimental_allow_proto3_optional");

    // Candid-ify generated Rust types.
    config.type_attribute(".", "#[derive(candid::CandidType, candid::Deserialize)]");
    // Because users of the types we supply put these on their types, we must
    // also add these derives.
    config.type_attribute(".", "#[derive(comparable::Comparable, serde::Serialize)]");

    let src_file = proto_paths
        .nervous_system
        .join("ic_nervous_system/pb/v1/nervous_system.proto");

    // Make most types copy (currently, only Image is not Copy).
    for type_name in COPY_TYPE_NAMES {
        config.type_attribute(
            format!("ic_nervous_system.pb.v1.{}", type_name),
            "#[derive(Copy)]",
        );
    }

    // Assert that all files and directories exist.
    assert!(src_file.exists());
    let search_paths = proto_paths.to_vec();
    for p in search_paths {
        assert!(p.exists());
    }

    config.out_dir(out_dir);
    std::fs::create_dir_all(out_dir).expect("failed to create output directory");
    config.out_dir(out_dir);

    config
        .compile_protos(&[src_file], &proto_paths.to_vec())
        .unwrap();

    ic_utils_rustfmt::rustfmt(out_dir).expect("failed to rustfmt protobufs");
}
