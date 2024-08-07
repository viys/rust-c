extern crate cc;

fn main() {
    cc::Build::new()
        .file("c_src/animal.c") // 替换为您的 C 源文件路径
        .compile("animal");   // 这将生成 libexample.a
}
