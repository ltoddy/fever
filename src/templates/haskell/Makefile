install-prepare:
	@stack install ormolu
	@stack install hlint

clean:
	@-stack clean

fmt:
	@-ormolu --mode inplace $(shell find app -name '*.hs')
	@-ormolu --mode inplace $(shell find src -name '*.hs')
	@-ormolu --mode inplace $(shell find test -name '*.hs')

run:
	@stack run

build:
	@stack build

lint:
	@hlint . --report
