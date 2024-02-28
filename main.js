import init, { run_app } from './pkg/shiren6_utils.js';
async function main() {
	await init('./pkg/shiren6_utils_bg.wasm');
	run_app();
}
main()