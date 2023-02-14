fn main(){
    tonic_build::configure()
        .out_dir("pb")
        .compile(&["proto/abi.proto"], &["."])
        .unwrap();
}