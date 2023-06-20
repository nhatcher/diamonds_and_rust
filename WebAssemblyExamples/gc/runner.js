async function main() {
	const m = readbuffer("module.wasm");
	const {instance} = await WebAssembly.instantiate(m, {});
	const p = instance.exports.create_point(40, 2);
	console.log("Length =", instance.exports.length(p));
}
main().catch(err => console.error(err));
