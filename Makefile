pkg/package.json: Cargo.toml Cargo.lock src/*.rs
	nix-shell --run 'cargo fmt; bash build.sh'

pkg/README.md:
	ln ../README.md pkg

docs/index.html: .jsdoc.json pkg/package.json
	npx jsdoc pkg/wasm_key_manager.js --configure .jsdoc.json --destination docs --verbose

package-lock.json: package.json 
	npm install
	touch $@
node_modules: package-lock.json


pkg: pkg/package.json pkg/README.md
docs: docs/index.html


.PHONY: preview-package publish-docs publish-package test


preview-package: pkg pkg/README.md
	npm pack --dry-run ./pkg

publish-package: pkg pkg/README.md
	npm publish --access public ./pkg

publish-docs: pkg
	@echo "\nBuilding Key Manager docs"
	make docs
	ln -s docs v$$( cat ./pkg/package.json | jq -r .version )
	@echo "\nAdding Key Manager docs..."
	git add -f docs
	git add v$$( cat ./pkg/package.json | jq -r .version )

test: node_modules pkg
	npm run test
