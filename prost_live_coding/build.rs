fn main(){
    println!("rerun-if-changed=src/pb/person.proto");
    println!("rerun-if-changed=build.rs");
    prost_build::Config::new()
        .out_dir("src/pb")
        .btree_map(&["scores"])
        .type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]")
        .field_attribute("data", "#[serde(skip_serializing_if = \"Vec::is_empty\")]")
        .compile_protos(&["src/pb/person.proto"], &["."])
        .unwrap();    
}