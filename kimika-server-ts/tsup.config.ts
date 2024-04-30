import { defineConfig } from "tsup"

export default defineConfig({
	entry: ["./main.ts"],
	splitting: false,
	clean: true,
	shims: true,
	minify: true,
	outExtension() {
		return { js: ".js" }
	},
	format: "iife",
})
