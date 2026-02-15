use prost::Message;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tonic_rest_build::{generate, ProstSerdeConfig, RestCodegenConfig};

fn collect_proto_files(root: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    fn walk(dir: &Path, out: &mut Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk(&path, out)?;
                continue;
            }

            if path.extension().and_then(|x| x.to_str()) != Some("proto") {
                continue;
            }

            let Some(file_name) = path.file_name().and_then(|x| x.to_str()) else {
                continue;
            };

            if file_name.ends_with("_doc.proto") {
                continue;
            }

            out.push(path.to_string_lossy().replace('\\', "/"));
        }
        Ok(())
    }

    let mut files = Vec::new();
    walk(root, &mut files)?;
    files.sort();
    Ok(files)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../api/protos");

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let descriptor_path = out_dir.join("file_descriptor_set.bin");
    let proto_root = PathBuf::from("../../api/protos");
    let proto_files = collect_proto_files(&proto_root)?;

    if proto_files.is_empty() {
        return Err("no .proto files found under ../../api/protos (excluding *_doc.proto)".into());
    }

    let proto_file_refs: Vec<&str> = proto_files.iter().map(String::as_str).collect();

    // Use buf to generate descriptor set (avoids protoc path issues with buf cache)
    let api_dir = PathBuf::from("../../api");
    let output = Command::new("buf")
        .current_dir(&api_dir)
        .args(["build", "-o", descriptor_path.to_str().unwrap()])
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "buf build failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    let descriptor_bytes = fs::read(&descriptor_path)?;
    let descriptor_set = prost_types::FileDescriptorSet::decode(&*descriptor_bytes)?;

    // Use buf's descriptor set directly instead of calling protoc again
    let mut prost_config = prost_build::Config::new();
    prost_config.file_descriptor_set_path(&descriptor_path);

    ProstSerdeConfig::new(&descriptor_bytes, &proto_file_refs)
        .wkt_root("tonic_rest::serde")
        .apply(&mut prost_config);

    tonic_prost_build::configure()
        .build_server(true)
        .build_client(false)
        .compile_fds_with_config(descriptor_set, prost_config)?;

    let rest_config = RestCodegenConfig::new()
        .package("rusktop.service.v1", "rusktop::service::v1")
        .package("user.service.v1", "user::service::v1")
        .proto_root("crate::proto");

    let code = generate(&descriptor_bytes, &rest_config)?;
    fs::write(out_dir.join("rest_routes.rs"), code)?;

    Ok(())
}
