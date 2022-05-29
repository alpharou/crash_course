mod immutable_by_default;

fn main() {
	println!("{:?}", (12, true, "hello"));

	immutable_by_default::run();
}
