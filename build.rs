use cc;

fn main() {
    cc::Build::new().file("src/windows.c").compile("liblow.a")
}
