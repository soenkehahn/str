bundle: str.tgz

str.tgz: dist/index.js
	yarn pack --filename str.tgz

dist/index.js: index.ts
	yarn tsc
