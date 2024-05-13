TESTFLAGS = --nocapture
UNDIZZY   = cargo run undizzy
DIZZY     = cargo run dizzy

ARGS =

test:
		cargo test -- ${TESTFLAGS} ${ARGS}

testUndizzy:
		 ${UNDIZZY} -f testfiles/undizzy_test1.dat & ${UNDIZZY} -f testfiles/undizzy_test2.dat

testDizzy:
		 ${DIZZY} testfiles/dizzy_test1.dat & ${DIZZY} testfiles/dizzy_test2.dat

