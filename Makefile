build:
	(cd cowltool && cargo build --release)
	(cd cowlctool && ghc main.hs -hidir target -odir target -o target/cowlc -O2)

check: 
	(cd cowltool && cargo check)
	(cd cowlctool && ghc main.hs -hidir target -odir target -o target/cowlc)

checkh: 
	(cd cowlctool && ghc main.hs -hidir target -odir target -o target/cowlc)

checkr: 
	(cd cowltool && cargo check)

install:
	echo "Coming soon! Cowl is still a work-in-progress."

