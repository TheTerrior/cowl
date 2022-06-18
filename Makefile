build:
	(cd cowltool && cargo build --release)
	(cd cowlctool && go build)
#	(cd cowlctool && kotlinc main.kt -include-runtime -d main.jar)
#	(cd cowlctool && ghc main.hs -hidir target -odir target -o target/cowlc -O2)

check: 
	(cd cowltool && cargo check)
	(cd cowlctool && go build)
#	(cd cowlctool && kotlinc main.kt -include-runtime -d main.jar)
#	(cd cowlctool && ghc main.hs -hidir target -odir target -o target/cowlc)

#checkh: 
#	(cd cowlctool && ghc main.hs -hidir target -odir target -o target/cowlc)

#checkk:
#	(cd cowlctool && kotlinc main.kt -include-runtime -d main.jar)

checkr: 
	(cd cowltool && cargo check)

checkg:
	(cd cowlctool && go build)

install:
	echo "Coming soon! Cowl is still a work-in-progress."

#NOTE: to run kotlin, do java -jar main.jar
