use std::fs::File;

fun write_file(path :string) {
    let mut f = File::create(path);
}
